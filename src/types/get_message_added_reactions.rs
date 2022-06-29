use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns reactions added for a message, along with their sender
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageAddedReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the message belongs

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message

    #[serde(default)]
    message_id: i64,
    /// If non-empty, only added reactions with the specified text representation will be returned

    #[serde(default)]
    reaction: String,
    /// Offset of the first entry to return as received from the previous request; use empty string to get the first chunk of results

    #[serde(default)]
    offset: String,
    /// The maximum number of reactions to be returned; must be positive and can't be greater than 100

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageAddedReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageAddedReactions {}

impl GetMessageAddedReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessageAddedReactionsBuilder {
        let mut inner = GetMessageAddedReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageAddedReactions".to_string();

        RTDGetMessageAddedReactionsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn reaction(&self) -> &String {
        &self.reaction
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetMessageAddedReactionsBuilder {
    inner: GetMessageAddedReactions,
}

impl RTDGetMessageAddedReactionsBuilder {
    pub fn build(&self) -> GetMessageAddedReactions {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn reaction<T: AsRef<str>>(&mut self, reaction: T) -> &mut Self {
        self.inner.reaction = reaction.as_ref().to_string();
        self
    }

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetMessageAddedReactions> for GetMessageAddedReactions {
    fn as_ref(&self) -> &GetMessageAddedReactions {
        self
    }
}

impl AsRef<GetMessageAddedReactions> for RTDGetMessageAddedReactionsBuilder {
    fn as_ref(&self) -> &GetMessageAddedReactions {
        &self.inner
    }
}
