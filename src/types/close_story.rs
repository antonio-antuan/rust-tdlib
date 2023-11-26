use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that a story is closed by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloseStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the sender of the story to close

    #[serde(default)]
    story_sender_chat_id: i64,
    /// The identifier of the story

    #[serde(default)]
    story_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CloseStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CloseStory {}

impl CloseStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CloseStoryBuilder {
        let mut inner = CloseStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "closeStory".to_string();

        CloseStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }
}

#[doc(hidden)]
pub struct CloseStoryBuilder {
    inner: CloseStory,
}

#[deprecated]
pub type RTDCloseStoryBuilder = CloseStoryBuilder;

impl CloseStoryBuilder {
    pub fn build(&self) -> CloseStory {
        self.inner.clone()
    }

    pub fn story_sender_chat_id(&mut self, story_sender_chat_id: i64) -> &mut Self {
        self.inner.story_sender_chat_id = story_sender_chat_id;
        self
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }
}

impl AsRef<CloseStory> for CloseStory {
    fn as_ref(&self) -> &CloseStory {
        self
    }
}

impl AsRef<CloseStory> for CloseStoryBuilder {
    fn as_ref(&self) -> &CloseStory {
        &self.inner
    }
}
