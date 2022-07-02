use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of messages found by a search
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total count of messages found; 1 if unknown

    #[serde(default)]
    total_count: i32,
    /// List of messages

    #[serde(default)]
    messages: Vec<Message>,
    /// The offset for the next request. If empty, there are no more results

    #[serde(default)]
    next_offset: String,
}

impl RObject for FoundMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FoundMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FoundMessagesBuilder {
        let mut inner = FoundMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FoundMessagesBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn next_offset(&self) -> &String {
        &self.next_offset
    }
}

#[doc(hidden)]
pub struct FoundMessagesBuilder {
    inner: FoundMessages,
}

#[deprecated]
pub type RTDFoundMessagesBuilder = FoundMessagesBuilder;

impl FoundMessagesBuilder {
    pub fn build(&self) -> FoundMessages {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn messages(&mut self, messages: Vec<Message>) -> &mut Self {
        self.inner.messages = messages;
        self
    }

    pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
        self.inner.next_offset = next_offset.as_ref().to_string();
        self
    }
}

impl AsRef<FoundMessages> for FoundMessages {
    fn as_ref(&self) -> &FoundMessages {
        self
    }
}

impl AsRef<FoundMessages> for FoundMessagesBuilder {
    fn as_ref(&self) -> &FoundMessages {
        &self.inner
    }
}
