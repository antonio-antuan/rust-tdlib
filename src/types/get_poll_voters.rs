use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns users voted for the specified option in a non-anonymous polls. For the optimal performance the number of returned users is chosen by the library
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPollVoters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the poll belongs
    chat_id: i64,
    /// Identifier of the message containing the poll
    message_id: i64,
    /// 0-based identifier of the answer option
    option_id: i32,
    /// Number of users to skip in the result; must be non-negative
    offset: i32,
    /// The maximum number of users to be returned; must be positive and can't be greater than 50. Fewer users may be returned than specified by the limit, even if the end of the voter list has not been reached
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetPollVoters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetPollVoters {}

impl GetPollVoters {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetPollVotersBuilder {
        let mut inner = GetPollVoters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPollVoters".to_string();

        RTDGetPollVotersBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn option_id(&self) -> i32 {
        self.option_id
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct RTDGetPollVotersBuilder {
    inner: GetPollVoters,
}

impl RTDGetPollVotersBuilder {
    pub fn build(&self) -> GetPollVoters {
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

    pub fn option_id(&mut self, option_id: i32) -> &mut Self {
        self.inner.option_id = option_id;
        self
    }

    pub fn offset(&mut self, offset: i32) -> &mut Self {
        self.inner.offset = offset;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetPollVoters> for GetPollVoters {
    fn as_ref(&self) -> &GetPollVoters {
        self
    }
}

impl AsRef<GetPollVoters> for RTDGetPollVotersBuilder {
    fn as_ref(&self) -> &GetPollVoters {
        &self.inner
    }
}
