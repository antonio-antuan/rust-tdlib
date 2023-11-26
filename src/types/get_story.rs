use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that posted the story

    #[serde(default)]
    story_sender_chat_id: i64,
    /// Story identifier

    #[serde(default)]
    story_id: i32,
    /// Pass true to get only locally available information without sending network requests

    #[serde(default)]
    only_local: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStory {}

impl GetStory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetStoryBuilder {
        let mut inner = GetStory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStory".to_string();

        GetStoryBuilder { inner }
    }

    pub fn story_sender_chat_id(&self) -> i64 {
        self.story_sender_chat_id
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn only_local(&self) -> bool {
        self.only_local
    }
}

#[doc(hidden)]
pub struct GetStoryBuilder {
    inner: GetStory,
}

#[deprecated]
pub type RTDGetStoryBuilder = GetStoryBuilder;

impl GetStoryBuilder {
    pub fn build(&self) -> GetStory {
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

    pub fn only_local(&mut self, only_local: bool) -> &mut Self {
        self.inner.only_local = only_local;
        self
    }
}

impl AsRef<GetStory> for GetStory {
    fn as_ref(&self) -> &GetStory {
        self
    }
}

impl AsRef<GetStory> for GetStoryBuilder {
    fn as_ref(&self) -> &GetStory {
        &self.inner
    }
}
