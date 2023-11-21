use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deletes a previously sent story. Can be called only if story.can_be_deleted == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that posted the story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// Identifier of the story to delete

    #[serde(default)]
    story_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteStory {}

impl DeleteStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteStoryBuilder {
        let mut inner = DeleteStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteStory".to_string();

        DeleteStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }
}

#[doc(hidden)]
pub struct DeleteStoryBuilder {
    inner: DeleteStory,
}

#[deprecated]
pub type RTDDeleteStoryBuilder = DeleteStoryBuilder;

impl DeleteStoryBuilder {
    pub fn build(&self) -> DeleteStory {
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

impl AsRef<DeleteStory> for DeleteStory {
    fn as_ref(&self) -> &DeleteStory {
        self
    }
}

impl AsRef<DeleteStory> for DeleteStoryBuilder {
    fn as_ref(&self) -> &DeleteStory {
        &self.inner
    }
}
