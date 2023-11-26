use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a list of forum topics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ForumTopics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total number of forum topics found

    #[serde(default)]
    total_count: i32,
    /// List of forum topics

    #[serde(default)]
    topics: Vec<ForumTopic>,
    /// Offset date for the next getForumTopics request

    #[serde(default)]
    next_offset_date: i32,
    /// Offset message identifier for the next getForumTopics request

    #[serde(default)]
    next_offset_message_id: i64,
    /// Offset message thread identifier for the next getForumTopics request

    #[serde(default)]
    next_offset_message_thread_id: i64,
}

impl RObject for ForumTopics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ForumTopics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ForumTopicsBuilder {
        let mut inner = ForumTopics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ForumTopicsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn topics(&self) -> &Vec<ForumTopic> {
        &self.topics
    }

    pub fn next_offset_date(&self) -> i32 {
        self.next_offset_date
    }

    pub fn next_offset_message_id(&self) -> i64 {
        self.next_offset_message_id
    }

    pub fn next_offset_message_thread_id(&self) -> i64 {
        self.next_offset_message_thread_id
    }
}

#[doc(hidden)]
pub struct ForumTopicsBuilder {
    inner: ForumTopics,
}

#[deprecated]
pub type RTDForumTopicsBuilder = ForumTopicsBuilder;

impl ForumTopicsBuilder {
    pub fn build(&self) -> ForumTopics {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn topics(&mut self, topics: Vec<ForumTopic>) -> &mut Self {
        self.inner.topics = topics;
        self
    }

    pub fn next_offset_date(&mut self, next_offset_date: i32) -> &mut Self {
        self.inner.next_offset_date = next_offset_date;
        self
    }

    pub fn next_offset_message_id(&mut self, next_offset_message_id: i64) -> &mut Self {
        self.inner.next_offset_message_id = next_offset_message_id;
        self
    }

    pub fn next_offset_message_thread_id(
        &mut self,
        next_offset_message_thread_id: i64,
    ) -> &mut Self {
        self.inner.next_offset_message_thread_id = next_offset_message_thread_id;
        self
    }
}

impl AsRef<ForumTopics> for ForumTopics {
    fn as_ref(&self) -> &ForumTopics {
        self
    }
}

impl AsRef<ForumTopics> for ForumTopicsBuilder {
    fn as_ref(&self) -> &ForumTopics {
        &self.inner
    }
}
