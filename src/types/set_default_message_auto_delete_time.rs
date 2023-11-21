use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the default message auto-delete time for new chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetDefaultMessageAutoDeleteTime {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New default message auto-delete time; must be from 0 up to 365 * 86400 and be divisible by 86400. If 0, then messages aren't deleted automatically
    message_auto_delete_time: MessageAutoDeleteTime,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetDefaultMessageAutoDeleteTime {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetDefaultMessageAutoDeleteTime {}

impl SetDefaultMessageAutoDeleteTime {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetDefaultMessageAutoDeleteTimeBuilder {
        let mut inner = SetDefaultMessageAutoDeleteTime::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setDefaultMessageAutoDeleteTime".to_string();

        SetDefaultMessageAutoDeleteTimeBuilder { inner }
    }

    pub fn message_auto_delete_time(&self) -> &MessageAutoDeleteTime {
        &self.message_auto_delete_time
    }
}

#[doc(hidden)]
pub struct SetDefaultMessageAutoDeleteTimeBuilder {
    inner: SetDefaultMessageAutoDeleteTime,
}

#[deprecated]
pub type RTDSetDefaultMessageAutoDeleteTimeBuilder = SetDefaultMessageAutoDeleteTimeBuilder;

impl SetDefaultMessageAutoDeleteTimeBuilder {
    pub fn build(&self) -> SetDefaultMessageAutoDeleteTime {
        self.inner.clone()
    }

    pub fn message_auto_delete_time<T: AsRef<MessageAutoDeleteTime>>(
        &mut self,
        message_auto_delete_time: T,
    ) -> &mut Self {
        self.inner.message_auto_delete_time = message_auto_delete_time.as_ref().clone();
        self
    }
}

impl AsRef<SetDefaultMessageAutoDeleteTime> for SetDefaultMessageAutoDeleteTime {
    fn as_ref(&self) -> &SetDefaultMessageAutoDeleteTime {
        self
    }
}

impl AsRef<SetDefaultMessageAutoDeleteTime> for SetDefaultMessageAutoDeleteTimeBuilder {
    fn as_ref(&self) -> &SetDefaultMessageAutoDeleteTime {
        &self.inner
    }
}
