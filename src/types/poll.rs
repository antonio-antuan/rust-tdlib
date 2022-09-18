use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Poll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique poll identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Poll question; 1-300 characters

    #[serde(default)]
    question: String,
    /// List of poll answer options

    #[serde(default)]
    options: Vec<PollOption>,
    /// Total number of voters, participating in the poll

    #[serde(default)]
    total_voter_count: i32,
    /// User identifiers of recent voters, if the poll is non-anonymous

    #[serde(default)]
    recent_voter_user_ids: Vec<i64>,
    /// True, if the poll is anonymous

    #[serde(default)]
    is_anonymous: bool,
    /// Type of the poll

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PollType::_is_default")]
    type_: PollType,
    /// Amount of time the poll will be active after creation, in seconds

    #[serde(default)]
    open_period: i32,
    /// Point in time (Unix timestamp) when the poll will automatically be closed

    #[serde(default)]
    close_date: i32,
    /// True, if the poll is closed

    #[serde(default)]
    is_closed: bool,
}

impl RObject for Poll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Poll {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PollBuilder {
        let mut inner = Poll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PollBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn question(&self) -> &String {
        &self.question
    }

    pub fn options(&self) -> &Vec<PollOption> {
        &self.options
    }

    pub fn total_voter_count(&self) -> i32 {
        self.total_voter_count
    }

    pub fn recent_voter_user_ids(&self) -> &Vec<i64> {
        &self.recent_voter_user_ids
    }

    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    pub fn type_(&self) -> &PollType {
        &self.type_
    }

    pub fn open_period(&self) -> i32 {
        self.open_period
    }

    pub fn close_date(&self) -> i32 {
        self.close_date
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}

#[doc(hidden)]
pub struct PollBuilder {
    inner: Poll,
}

#[deprecated]
pub type RTDPollBuilder = PollBuilder;

impl PollBuilder {
    pub fn build(&self) -> Poll {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn question<T: AsRef<str>>(&mut self, question: T) -> &mut Self {
        self.inner.question = question.as_ref().to_string();
        self
    }

    pub fn options(&mut self, options: Vec<PollOption>) -> &mut Self {
        self.inner.options = options;
        self
    }

    pub fn total_voter_count(&mut self, total_voter_count: i32) -> &mut Self {
        self.inner.total_voter_count = total_voter_count;
        self
    }

    pub fn recent_voter_user_ids(&mut self, recent_voter_user_ids: Vec<i64>) -> &mut Self {
        self.inner.recent_voter_user_ids = recent_voter_user_ids;
        self
    }

    pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
        self.inner.is_anonymous = is_anonymous;
        self
    }

    pub fn type_<T: AsRef<PollType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn open_period(&mut self, open_period: i32) -> &mut Self {
        self.inner.open_period = open_period;
        self
    }

    pub fn close_date(&mut self, close_date: i32) -> &mut Self {
        self.inner.close_date = close_date;
        self
    }

    pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
        self.inner.is_closed = is_closed;
        self
    }
}

impl AsRef<Poll> for Poll {
    fn as_ref(&self) -> &Poll {
        self
    }
}

impl AsRef<Poll> for PollBuilder {
    fn as_ref(&self) -> &Poll {
        &self.inner
    }
}
