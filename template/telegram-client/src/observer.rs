use std::sync::{RwLock};
use std::collections::HashMap;
use futures::channel::mpsc;
use futures::SinkExt;
use rtdlib::types::TdType;


pub struct Observer {
  channels: RwLock<HashMap<String, mpsc::Sender<TdType>>>,
}

impl Observer {
  pub fn new() -> Self {
    Self {
      channels: RwLock::new(HashMap::new())
    }
  }

  pub async fn notify(&self, extra: String, payload: TdType) {
    let mut map = self.channels.write().unwrap();
    if let Some(sender) = map.get_mut(&extra) {
      sender.send(payload).await;
    }
  }

  pub fn subscribe(&self, extra: String) -> mpsc::Receiver<TdType> {
    let (sender, mut receiver) = mpsc::channel::<TdType>(1);
    match self.channels.write() {
      Ok(mut map) => {
        map.insert(extra, sender);
      }
      _ => {}
    };
    receiver
  }

  pub fn unsubscribe(&self, extra: &str) {
    match self.channels.write() {
      Ok(mut map) => {
        map.remove(extra);
      }
      _ => {}
    };
  }
}
