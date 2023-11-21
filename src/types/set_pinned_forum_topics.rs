use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the order of pinned forum topics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPinnedForumTopics {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// The new list of pinned forum topics

    #[serde(default)]
    message_thread_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetPinnedForumTopics {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetPinnedForumTopics {}

impl SetPinnedForumTopics {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetPinnedForumTopicsBuilder {
        let mut inner = SetPinnedForumTopics::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setPinnedForumTopics".to_string();

        SetPinnedForumTopicsBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn message_thread_ids(&self) -> &Vec<i64> {
        &self.message_thread_ids
    }
}

#[doc(hidden)]
pub struct SetPinnedForumTopicsBuilder {
    inner: SetPinnedForumTopics,
}

#[deprecated]
pub type RTDSetPinnedForumTopicsBuilder = SetPinnedForumTopicsBuilder;

impl SetPinnedForumTopicsBuilder {
    pub fn build(&self) -> SetPinnedForumTopics {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn message_thread_ids(&mut self, message_thread_ids: Vec<i64>) -> &mut Self {
        self.inner.message_thread_ids = message_thread_ids;
        self
    }
}

impl AsRef<SetPinnedForumTopics> for SetPinnedForumTopics {
    fn as_ref(&self) -> &SetPinnedForumTopics {
        self
    }
}

impl AsRef<SetPinnedForumTopics> for SetPinnedForumTopicsBuilder {
    fn as_ref(&self) -> &SetPinnedForumTopics {
        &self.inner
    }
}
