use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the list of all stories posted by the given chat; requires can_edit_stories rights for channel chats. The stories are returned in a reverse chronological order (i.e., in order of decreasing story_id). For optimal performance, the number of returned stories is chosen by TDLib
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatArchivedStories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the story starting from which stories must be returned; use 0 to get results from the last story

    #[serde(default)]
    from_story_id: i32,
    /// The maximum number of stories to be returned For optimal performance, the number of returned stories is chosen by TDLib and can be smaller than the specified limit

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatArchivedStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatArchivedStories {}

impl GetChatArchivedStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatArchivedStoriesBuilder {
        let mut inner = GetChatArchivedStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatArchivedStories".to_string();

        GetChatArchivedStoriesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn from_story_id(&self) -> i32 {
        self.from_story_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct GetChatArchivedStoriesBuilder {
    inner: GetChatArchivedStories,
}

#[deprecated]
pub type RTDGetChatArchivedStoriesBuilder = GetChatArchivedStoriesBuilder;

impl GetChatArchivedStoriesBuilder {
    pub fn build(&self) -> GetChatArchivedStories {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn from_story_id(&mut self, from_story_id: i32) -> &mut Self {
        self.inner.from_story_id = from_story_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<GetChatArchivedStories> for GetChatArchivedStories {
    fn as_ref(&self) -> &GetChatArchivedStories {
        self
    }
}

impl AsRef<GetChatArchivedStories> for GetChatArchivedStoriesBuilder {
    fn as_ref(&self) -> &GetChatArchivedStories {
        &self.inner
    }
}
