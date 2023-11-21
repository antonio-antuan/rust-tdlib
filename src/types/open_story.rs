use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs TDLib that a story is opened and is being viewed by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OpenStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The identifier of the sender of the opened story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// The identifier of the story

    #[serde(default)]
    story_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for OpenStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for OpenStory {}

impl OpenStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OpenStoryBuilder {
        let mut inner = OpenStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "openStory".to_string();

        OpenStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }
}

#[doc(hidden)]
pub struct OpenStoryBuilder {
    inner: OpenStory,
}

#[deprecated]
pub type RTDOpenStoryBuilder = OpenStoryBuilder;

impl OpenStoryBuilder {
    pub fn build(&self) -> OpenStory {
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

impl AsRef<OpenStory> for OpenStory {
    fn as_ref(&self) -> &OpenStory {
        self
    }
}

impl AsRef<OpenStory> for OpenStoryBuilder {
    fn as_ref(&self) -> &OpenStory {
        &self.inner
    }
}
