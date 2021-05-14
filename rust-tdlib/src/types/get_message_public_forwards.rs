use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns forwarded copies of a channel message to different public channels. For optimal performance the number of returned messages is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessagePublicForwards {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier of the message
    chat_id: i64,
    /// Message identifier
    message_id: i64,
    /// Offset of the first entry to return as received from the previous request; use empty string to get first chunk of results
    offset: String,
    /// The maximum number of messages to be returned; must be positive and can't be greater than 100. Fewer messages may be returned than specified by the limit, even if the end of the list has not been reached
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessagePublicForwards {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessagePublicForwards {}

impl GetMessagePublicForwards {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMessagePublicForwardsBuilder {
        let mut inner = GetMessagePublicForwards::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessagePublicForwards".to_string();

        RTDGetMessagePublicForwardsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn offset(&self) -> &String {
        &self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetMessagePublicForwardsBuilder {
    inner: GetMessagePublicForwards,
}

impl RTDGetMessagePublicForwardsBuilder {
    pub fn build(&self) -> GetMessagePublicForwards {
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

    pub fn offset<T: AsRef<str>>(&mut self, offset: T) -> &mut Self {
        self.inner.offset = offset.as_ref().to_string();
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetMessagePublicForwards> for GetMessagePublicForwards {
    fn as_ref(&self) -> &GetMessagePublicForwards {
        self
    }
}

impl AsRef<GetMessagePublicForwards> for RTDGetMessagePublicForwardsBuilder {
    fn as_ref(&self) -> &GetMessagePublicForwards {
        &self.inner
    }
}
