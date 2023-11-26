use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a boost applied to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoost {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique identifier of the boost

    #[serde(default)]
    id: String,
    /// The number of identical boosts applied

    #[serde(default)]
    count: i32,
    /// Source of the boost

    #[serde(skip_serializing_if = "ChatBoostSource::_is_default")]
    source: ChatBoostSource,
    /// Point in time (Unix timestamp) when the chat was boosted

    #[serde(default)]
    start_date: i32,
    /// Point in time (Unix timestamp) when the boost will expire

    #[serde(default)]
    expiration_date: i32,
}

impl RObject for ChatBoost {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatBoost {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatBoostBuilder {
        let mut inner = ChatBoost::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatBoostBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn count(&self) -> i32 {
        self.count
    }

    pub fn source(&self) -> &ChatBoostSource {
        &self.source
    }

    pub fn start_date(&self) -> i32 {
        self.start_date
    }

    pub fn expiration_date(&self) -> i32 {
        self.expiration_date
    }
}

#[doc(hidden)]
pub struct ChatBoostBuilder {
    inner: ChatBoost,
}

#[deprecated]
pub type RTDChatBoostBuilder = ChatBoostBuilder;

impl ChatBoostBuilder {
    pub fn build(&self) -> ChatBoost {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn count(&mut self, count: i32) -> &mut Self {
        self.inner.count = count;
        self
    }

    pub fn source<T: AsRef<ChatBoostSource>>(&mut self, source: T) -> &mut Self {
        self.inner.source = source.as_ref().clone();
        self
    }

    pub fn start_date(&mut self, start_date: i32) -> &mut Self {
        self.inner.start_date = start_date;
        self
    }

    pub fn expiration_date(&mut self, expiration_date: i32) -> &mut Self {
        self.inner.expiration_date = expiration_date;
        self
    }
}

impl AsRef<ChatBoost> for ChatBoost {
    fn as_ref(&self) -> &ChatBoost {
        self
    }
}

impl AsRef<ChatBoost> for ChatBoostBuilder {
    fn as_ref(&self) -> &ChatBoost {
        &self.inner
    }
}
