use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Messages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total count of messages found

    #[serde(default)]
    total_count: i32,
    /// List of messages; messages may be null

    #[serde(default)]
    messages: Vec<Option<Message>>,
}

impl RObject for Messages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Messages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MessagesBuilder {
        let mut inner = Messages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MessagesBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn messages(&self) -> &Vec<Option<Message>> {
        &self.messages
    }
}

#[doc(hidden)]
pub struct MessagesBuilder {
    inner: Messages,
}

#[deprecated]
pub type RTDMessagesBuilder = MessagesBuilder;

impl MessagesBuilder {
    pub fn build(&self) -> Messages {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn messages(&mut self, messages: Vec<Option<Message>>) -> &mut Self {
        self.inner.messages = messages;
        self
    }
}

impl AsRef<Messages> for Messages {
    fn as_ref(&self) -> &Messages {
        self
    }
}

impl AsRef<Messages> for MessagesBuilder {
    fn as_ref(&self) -> &Messages {
        &self.inner
    }
}
