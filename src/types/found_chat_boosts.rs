use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of boosts applied to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FoundChatBoosts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Total number of boosts applied to the chat

    #[serde(default)]
    total_count: i32,
    /// List of boosts

    #[serde(default)]
    boosts: Vec<ChatBoost>,
    /// The offset for the next request. If empty, there are no more results

    #[serde(default)]
    next_offset: String,
}

impl RObject for FoundChatBoosts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl FoundChatBoosts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FoundChatBoostsBuilder {
        let mut inner = FoundChatBoosts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FoundChatBoostsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn boosts(&self) -> &Vec<ChatBoost> {
        &self.boosts
    }

    pub fn next_offset(&self) -> &String {
        &self.next_offset
    }
}

#[doc(hidden)]
pub struct FoundChatBoostsBuilder {
    inner: FoundChatBoosts,
}

#[deprecated]
pub type RTDFoundChatBoostsBuilder = FoundChatBoostsBuilder;

impl FoundChatBoostsBuilder {
    pub fn build(&self) -> FoundChatBoosts {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn boosts(&mut self, boosts: Vec<ChatBoost>) -> &mut Self {
        self.inner.boosts = boosts;
        self
    }

    pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
        self.inner.next_offset = next_offset.as_ref().to_string();
        self
    }
}

impl AsRef<FoundChatBoosts> for FoundChatBoosts {
    fn as_ref(&self) -> &FoundChatBoosts {
        self
    }
}

impl AsRef<FoundChatBoosts> for FoundChatBoostsBuilder {
    fn as_ref(&self) -> &FoundChatBoosts {
        &self.inner
    }
}
