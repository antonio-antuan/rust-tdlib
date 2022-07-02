use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns users voted for the specified option in a non-anonymous polls. For optimal performance, the number of returned users is chosen by TDLib
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetPollVoters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to which the poll belongs

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the message containing the poll

    #[serde(default)]
    message_id: i64,
    /// 0-based identifier of the answer option

    #[serde(default)]
    option_id: i32,
    /// Number of users to skip in the result; must be non-negative

    #[serde(default)]
    offset: i32,
    /// The maximum number of users to be returned; must be positive and can't be greater than 50. For optimal performance, the number of returned users is chosen by TDLib and can be smaller than the specified limit, even if the end of the voter list has not been reached

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetPollVotersBuilder {
        let mut inner = GetPollVoters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getPollVoters".to_string();

        GetPollVotersBuilder { inner }
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
pub struct GetPollVotersBuilder {
    inner: GetPollVoters,
}

#[deprecated]
pub type RTDGetPollVotersBuilder = GetPollVotersBuilder;

impl GetPollVotersBuilder {
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

impl AsRef<GetPollVoters> for GetPollVotersBuilder {
    fn as_ref(&self) -> &GetPollVoters {
        &self.inner
    }
}
