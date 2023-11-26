use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains default auto-delete timer setting for new chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageAutoDeleteTime {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message auto-delete time, in seconds. If 0, then messages aren't deleted automatically

    #[serde(default)]
    time: i32,
}

impl RObject for MessageAutoDeleteTime {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageAutoDeleteTime {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageAutoDeleteTimeBuilder {
        let mut inner = MessageAutoDeleteTime::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageAutoDeleteTimeBuilder { inner }
    }

    pub fn time(&self) -> i32 {
        self.time
    }
}

#[doc(hidden)]
pub struct MessageAutoDeleteTimeBuilder {
    inner: MessageAutoDeleteTime,
}

#[deprecated]
pub type RTDMessageAutoDeleteTimeBuilder = MessageAutoDeleteTimeBuilder;

impl MessageAutoDeleteTimeBuilder {
    pub fn build(&self) -> MessageAutoDeleteTime {
        self.inner.clone()
    }

    pub fn time(&mut self, time: i32) -> &mut Self {
        self.inner.time = time;
        self
    }
}

impl AsRef<MessageAutoDeleteTime> for MessageAutoDeleteTime {
    fn as_ref(&self) -> &MessageAutoDeleteTime {
        self
    }
}

impl AsRef<MessageAutoDeleteTime> for MessageAutoDeleteTimeBuilder {
    fn as_ref(&self) -> &MessageAutoDeleteTime {
        &self.inner
    }
}
