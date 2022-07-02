use futures::channel::oneshot;
use std::collections::HashMap;
#[doc(hidden)]
use std::sync::RwLock;

lazy_static::lazy_static! {
    pub(super) static ref OBSERVER: Observer = Observer::new();
}

pub(super) struct Observer {
    channels: RwLock<HashMap<String, oneshot::Sender<serde_json::Value>>>,
}

impl Observer {
    fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
        }
    }

    pub fn notify(&self, t: serde_json::Value) -> Option<serde_json::Value> {
        match t.get("@extra") {
            None => Some(t),
            Some(serde_json::Value::String(extra)) => {
                let mut map = self.channels.write().unwrap();
                match map.remove(extra) {
                    None => {
                        log::warn!("no subscribers for {}", extra);
                    }
                    Some(sender) => {
                        log::trace!("signal send for {}", extra);
                        if let Err(t) = sender.send(t) {
                            log::warn!("request already closed, received update: {:?}", t)
                        };
                    }
                }
                None
            }
            Some(_) => {
                log::error!("invalid type for extra, data received: {}", t);
                None
            }
        }
    }

    pub fn subscribe(&self, extra: &str) -> oneshot::Receiver<serde_json::Value> {
        let (sender, receiver) = oneshot::channel();
        match self.channels.write() {
            Ok(mut map) => {
                map.insert(extra.to_string(), sender);
                log::trace!("subscribed for {}", extra);
            }
            _ => {
                log::warn!("can't acquire lock for notifier map");
            }
        };
        receiver
    }

    pub fn unsubscribe(&self, extra: &str) {
        if let Ok(mut map) = self.channels.write() {
            log::trace!("remove {} subscription", &extra);
            map.remove(extra);
        };
    }
}
