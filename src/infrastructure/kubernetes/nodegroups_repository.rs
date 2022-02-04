use async_trait::async_trait;
use std::sync::{atomic::AtomicBool, Arc};

use anyhow::Result;
use log::info;

use futures::StreamExt;
use reflector::store::Writer;
use tokio::task::JoinHandle;

use crate::domain::model::NodeGroupDto;
use crate::domain::ports::outgoing::Repository;
use crate::infrastructure::kubernetes::model::NodeGroup;
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
pub struct DefaultNodegroupsRepository {
  running: Arc<AtomicBool>,
  thread_handle: Arc<JoinHandle<Result<()>>>,
  store: Store<NodeGroup>,
}

impl DefaultNodegroupsRepository {
  pub fn new(client: Client) -> Self {
    let nodegroups: Api<NodeGroup> = Api::all(client);

    let store = reflector::store::Writer::<NodeGroup>::default();
    let reader = store.as_reader();
    let lp = ListParams::default().timeout(10); // short watch timeout in this example
    let my_future = Self::execute(store, nodegroups, lp);

    Self {
      running: Arc::new(AtomicBool::new(false)),
      thread_handle: Arc::new(tokio::spawn(async { my_future.await })),
      store: reader,
    }
  }

  async fn execute(writer: Writer<NodeGroup>, nodegroups: Api<NodeGroup>, lp: ListParams) -> Result<()> {
    let nodegroup_reflector = reflector(writer, watcher(nodegroups, lp));
    try_flatten_touched(nodegroup_reflector)
      .filter_map(|x| async move { std::result::Result::ok(x) })
      .for_each(|o| {
        info!("NodeGroup detected: {:?}", o.metadata.name);
        futures::future::ready(())
      })
      .await;
    Ok(())
  }
}

#[async_trait(?Send)]
impl Repository<NodeGroupDto> for DefaultNodegroupsRepository {
  fn find_by(&self, name: &str) -> Option<NodeGroupDto> {
    let key = ObjectRef::new(name);
    self.store.get(&key).and_then(|s| NodeGroupDto::try_from(s).ok())
  }

  fn find_all(&self) -> Option<Vec<NodeGroupDto>> {
    Some(
      self
        .store
        .state()
        .into_iter()
        .filter_map(|d| NodeGroupDto::try_from(d).ok())
        .collect(),
    )
  }
}
