use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs the server about the number of pending bot updates if they haven't been processed for a long time; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBotUpdatesStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The number of pending updates

    #[serde(default)]
    pending_update_count: i32,
    /// The last error message

    #[serde(default)]
    error_message: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetBotUpdatesStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetBotUpdatesStatus {}

impl SetBotUpdatesStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetBotUpdatesStatusBuilder {
        let mut inner = SetBotUpdatesStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setBotUpdatesStatus".to_string();

        SetBotUpdatesStatusBuilder { inner }
    }

    pub fn pending_update_count(&self) -> i32 {
        self.pending_update_count
    }

    pub fn error_message(&self) -> &String {
        &self.error_message
    }
}

#[doc(hidden)]
pub struct SetBotUpdatesStatusBuilder {
    inner: SetBotUpdatesStatus,
}

#[deprecated]
pub type RTDSetBotUpdatesStatusBuilder = SetBotUpdatesStatusBuilder;

impl SetBotUpdatesStatusBuilder {
    pub fn build(&self) -> SetBotUpdatesStatus {
        self.inner.clone()
    }

    pub fn pending_update_count(&mut self, pending_update_count: i32) -> &mut Self {
        self.inner.pending_update_count = pending_update_count;
        self
    }

    pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
        self.inner.error_message = error_message.as_ref().to_string();
        self
    }
}

impl AsRef<SetBotUpdatesStatus> for SetBotUpdatesStatus {
    fn as_ref(&self) -> &SetBotUpdatesStatus {
        self
    }
}

impl AsRef<SetBotUpdatesStatus> for SetBotUpdatesStatusBuilder {
    fn as_ref(&self) -> &SetBotUpdatesStatus {
        &self.inner
    }
}
