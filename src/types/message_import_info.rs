use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a message created with importMessages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageImportInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the original sender

    #[serde(default)]
    sender_name: String,
    /// Point in time (Unix timestamp) when the message was originally sent

    #[serde(default)]
    date: i32,
}

impl RObject for MessageImportInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MessageImportInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessageImportInfoBuilder {
        let mut inner = MessageImportInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessageImportInfoBuilder { inner }
    }

    pub fn sender_name(&self) -> &String {
        &self.sender_name
    }

    pub fn date(&self) -> i32 {
        self.date
    }
}

#[doc(hidden)]
pub struct MessageImportInfoBuilder {
    inner: MessageImportInfo,
}

#[deprecated]
pub type RTDMessageImportInfoBuilder = MessageImportInfoBuilder;

impl MessageImportInfoBuilder {
    pub fn build(&self) -> MessageImportInfo {
        self.inner.clone()
    }

    pub fn sender_name<T: AsRef<str>>(&mut self, sender_name: T) -> &mut Self {
        self.inner.sender_name = sender_name.as_ref().to_string();
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }
}

impl AsRef<MessageImportInfo> for MessageImportInfo {
    fn as_ref(&self) -> &MessageImportInfo {
        self
    }
}

impl AsRef<MessageImportInfo> for MessageImportInfoBuilder {
    fn as_ref(&self) -> &MessageImportInfo {
        &self.inner
    }
}
