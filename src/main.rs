#[macro_use]
extern crate lazy_static;
extern crate base64;

extern crate dotenv;

mod application;
mod domain;
mod infrastructure;
mod utils;
use actix_web::{middleware, web::Data, App, HttpServer};
use anyhow::{Context, Result};
use kube::client::Client;
mod env_config;

use crate::domain::model::InstanceType;
use crate::domain::ports::incoming::ScheduledService;
use crate::domain::ports::outgoing::WriteStore;
use crate::domain::services::probes::DefaultProbesService;
use crate::domain::services::{
  DefaultInstanceTypesService, DefaultInstanceTypesUpdater, DefaultNodegroupsService, DefaultSecretsService, DefaultTemplateService,
  ScheduledInstanceTypesService,
};
use crate::env_config::EnvConfig;
use crate::infrastructure::datasources::FileDataSource;
use crate::infrastructure::kubernetes::{DefaultNodegroupsRepository, DefaultSecretsRepository, KubesealClient};

use crate::infrastructure::{GitVersionControl, InMemoryStore};

#[inline]
pub fn main_error<E: std::fmt::Debug>(msg: &'static str) -> Box<dyn FnOnce(E) -> std::io::Error> {
  Box::new(move |err| {
    let msg = format!("{}: {:#?}", msg, err);
    std::io::Error::new(std::io::ErrorKind::Other, msg)
  })
}

//#[actix_rt::main]

/**
 * This creates 8 main-tokio threads and 10 actix-rt|system.
 * Gives much better control over things.
 */
fn main() -> Result<()> {
  actix_web::rt::System::with_tokio_rt(|| {
    tokio::runtime::Builder::new_multi_thread()
      .enable_all()
      .worker_threads(8)
      .thread_name("main-tokio")
      .build()
      .unwrap()
  })
  .block_on(async_main())
}

async fn async_main() -> Result<()> {
  std::env::set_var("RUST_LOG", "info,kube=info");
  env_logger::init();
  dotenv::dotenv().map_err(main_error("Error loading .env files"))?;

  let client = Client::try_default().await.expect("create client");
  let namespace = EnvConfig::namespace().context("Error retrieving the namespace")?;

  // We initialize the repository outside the http server. Otherwise, we will create a new reflector per thread.
  let secrets_repository = DefaultSecretsRepository::new(client.clone(), &namespace);
  let sealed_secret_client = KubesealClient::new(None);
  let nodegroup_repository = DefaultNodegroupsRepository::new(client.clone());

  //Shared store for instances types
  let store_path = EnvConfig::stores_path().context("Error determining the path for the instance types key-value store")?;
  let store = InMemoryStore::<InstanceType>::new(store_path)?;

  //Template services
  let template_service = DefaultTemplateService::new().context("Error creating templates")?;

  //Version control
  let gitops_config = EnvConfig::gitops_config()?;
  let git_service = GitVersionControl::new(gitops_config.clone(), None);

  //Create the Instance tpye cron service to update the store in the background
  create_cron_for_instance_types(store.clone())?;

  let _ = HttpServer::new(move || {
    let probes_service = DefaultProbesService::new();
    // Secrets init
    let secrets_service = DefaultSecretsService::new(secrets_repository.clone(), sealed_secret_client.clone(), git_service.clone());

    //Instance types
    let instance_types_service = DefaultInstanceTypesService::new(store.clone());

    // Nodegroups
    let nodegroup_service = DefaultNodegroupsService::new(
      nodegroup_repository.clone(),
      store.clone(),
      template_service.clone(),
      git_service.clone(),
    );

    App::new()
      .app_data(Data::new(secrets_service))
      .app_data(Data::new(nodegroup_service))
      .app_data(Data::new(probes_service))
      .app_data(Data::new(instance_types_service))
      .wrap(middleware::Logger::default())
      .configure(application::api::probes::routes::<DefaultProbesService>)
      .configure(
        application::api::nodegroups::routes::<
          DefaultNodegroupsService<DefaultNodegroupsRepository, InMemoryStore<InstanceType>, DefaultTemplateService, GitVersionControl>,
        >,
      )
      .configure(application::api::secrets::routes::<DefaultSecretsService<DefaultSecretsRepository, KubesealClient, GitVersionControl>>)
      .configure(application::api::instance_types::routes::<DefaultInstanceTypesService<InMemoryStore<InstanceType>>>)
  })
  .workers(10)
  .bind("0.0.0.0:8000")
  .expect("bind to 0.0.0.0:8000")
  .shutdown_timeout(5)
  .run()
  .await;

  Ok(())
}

fn create_cron_for_instance_types<S>(store: S) -> Result<()>
where
  S: WriteStore<InstanceType> + Clone + Send + Sync + 'static,
{
  let updater_service = DefaultInstanceTypesUpdater::new(store);
  let file_data_source = EnvConfig::instance_types_file_source()
    .context("Error determining the path for the instance types file data source")
    .map(FileDataSource::new)?;
  let scheduled_service = ScheduledInstanceTypesService::new(updater_service, file_data_source);
  scheduled_service.start()
}
