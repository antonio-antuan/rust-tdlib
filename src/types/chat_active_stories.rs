use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes active stories posted by a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActiveStories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat that posted the stories

    #[serde(default)]
    chat_id: i64,
    /// Identifier of the story list in which the stories are shown; may be null if the stories aren't shown in a story list
    list: Option<StoryList>,
    /// A parameter used to determine order of the stories in the story list; 0 if the stories doesn't need to be shown in the story list. Stories must be sorted by the pair (order, story_sender_chat_id) in descending order

    #[serde(default)]
    order: i64,
    /// Identifier of the last read active story

    #[serde(default)]
    max_read_story_id: i32,
    /// Basic information about the stories; use getStory to get full information about the stories. The stories are in a chronological order (i.e., in order of increasing story identifiers)

    #[serde(default)]
    stories: Vec<StoryInfo>,
}

impl RObject for ChatActiveStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatActiveStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActiveStoriesBuilder {
        let mut inner = ChatActiveStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActiveStoriesBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn list(&self) -> &Option<StoryList> {
        &self.list
    }

    pub fn order(&self) -> i64 {
        self.order
    }

    pub fn max_read_story_id(&self) -> i32 {
        self.max_read_story_id
    }

    pub fn stories(&self) -> &Vec<StoryInfo> {
        &self.stories
    }
}

#[doc(hidden)]
pub struct ChatActiveStoriesBuilder {
    inner: ChatActiveStories,
}

#[deprecated]
pub type RTDChatActiveStoriesBuilder = ChatActiveStoriesBuilder;

impl ChatActiveStoriesBuilder {
    pub fn build(&self) -> ChatActiveStories {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn list<T: AsRef<StoryList>>(&mut self, list: T) -> &mut Self {
        self.inner.list = Some(list.as_ref().clone());
        self
    }

    pub fn order(&mut self, order: i64) -> &mut Self {
        self.inner.order = order;
        self
    }

    pub fn max_read_story_id(&mut self, max_read_story_id: i32) -> &mut Self {
        self.inner.max_read_story_id = max_read_story_id;
        self
    }

    pub fn stories(&mut self, stories: Vec<StoryInfo>) -> &mut Self {
        self.inner.stories = stories;
        self
    }
}

impl AsRef<ChatActiveStories> for ChatActiveStories {
    fn as_ref(&self) -> &ChatActiveStories {
        self
    }
}

impl AsRef<ChatActiveStories> for ChatActiveStoriesBuilder {
    fn as_ref(&self) -> &ChatActiveStories {
        &self.inner
    }
}
