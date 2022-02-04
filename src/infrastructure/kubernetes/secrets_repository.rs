use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use anyhow::Result;
use log::info;

use k8s_openapi::api::core::v1::Secret;

use futures::StreamExt;
use reflector::store::Writer;

use crate::domain::model::SecretDto;
use crate::domain::ports::outgoing::Repository;
use kube::{
  api::ListParams,
  client::Client,
  runtime::{
    reflector,
    reflector::{ObjectRef, Store},
    utils::try_flatten_touched,
    watcher,
  },
  Api,
};

#[derive(Clone)]
pub struct DefaultSecretsRepository {
  running: Arc<AtomicBool>,
  //thread_handle: Option<JoinHandle<Result<()>>>,
  store: Store<Secret>,
  namespace: String,
}

impl DefaultSecretsRepository {
  pub fn new(client: Client, namespace: &str) -> Self {
    let secrets: Api<Secret> = Api::namespaced(client, &*namespace);

    let store = reflector::store::Writer::<Secret>::default();
    let reader = store.as_reader();
    // let lp = ListParams::default().fields(&format!("metadata.name={}", "blog")).timeout(10);
    // ListParams::default().labels("app=blog");
    let lp = ListParams::default().timeout(10); // short watch timeout in this example
    let my_future = Self::execute(store, secrets, lp);
    tokio::spawn(async { my_future.await });
    Self {
      running: Arc::new(AtomicBool::new(false)),
      //thread_handle: Some(tokio::spawn(async {my_future.await})),
      store: reader,
      namespace: namespace.to_string(),
    }
  }

  async fn execute(writer: Writer<Secret>, secrets: Api<Secret>, lp: ListParams) -> Result<()> {
    let secrets_reflector = reflector(writer, watcher(secrets, lp));
    try_flatten_touched(secrets_reflector)
      .filter_map(|x| async move { std::result::Result::ok(x) })
      .for_each(|o| {
        info!("Secret ivan detected: {:?}", o.metadata.name);
        futures::future::ready(())
      })
      .await;
    Ok(())
  }
}

#[async_trait(?Send)]
impl Repository<SecretDto> for DefaultSecretsRepository {
  fn find_by(&self, name: &str) -> Option<SecretDto> {
    let key = ObjectRef::new(name).within(&self.namespace);
    self.store.get(&key).and_then(|s| SecretDto::try_from(s).ok())
  }

  fn find_all(&self) -> Option<Vec<SecretDto>> {
    Some(self.store.state().into_iter().filter_map(|d| SecretDto::try_from(d).ok()).collect())
  }
}

/*

let mut stream = watcher(deploys, ListParams::default()).boxed();
while let Some(event) = stream.try_next().await? {
    match event {
        Event::Applied(obj) => info!("UPDATED: {:?}", obj.metadata.name),
        Event::Deleted(obj) => info!("DELETED: {:?}", obj.metadata.name),
        Event::Restarted(obj) => {
            info!("RESTARTED ALL THE NODES!");
            obj.into_iter().for_each(|node| {
                info!("NODE_RESTARTED: {:?}", node.metadata.name);
            });
        }
        _ => {}
    };
}*/

/*
        loop {
            println!("Processing loop..");
            stream.try_next().await
            .and_then(|event|  {
                    println!("{:?}", event);
                    Ok(())
                });
        }
*/
/*
.map_ok(|ev| match ev {
        Event::Applied(obj) => Event::Applied(Watched::Secret(obj)),
        Event::Deleted(obj) => Event::Deleted(Watched::Secret(obj)),
        Event::Restarted(objs) => Event::Restarted(objs.into_iter().map(Watched::Secret).collect()),
    })
*/
