use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Loads more active stories from a story list. The loaded stories will be sent through updates. Active stories are sorted by the pair (active_stories.order, active_stories.story_sender_chat_id) in descending order. Returns a 404 error if all active stories have been loaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoadActiveStories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The story list in which to load active stories

    #[serde(skip_serializing_if = "StoryList::_is_default")]
    story_list: StoryList,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for LoadActiveStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for LoadActiveStories {}

impl LoadActiveStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LoadActiveStoriesBuilder {
        let mut inner = LoadActiveStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "loadActiveStories".to_string();

        LoadActiveStoriesBuilder { inner }
    }

    pub fn story_list(&self) -> &StoryList {
        &self.story_list
    }
}

#[doc(hidden)]
pub struct LoadActiveStoriesBuilder {
    inner: LoadActiveStories,
}

#[deprecated]
pub type RTDLoadActiveStoriesBuilder = LoadActiveStoriesBuilder;

impl LoadActiveStoriesBuilder {
    pub fn build(&self) -> LoadActiveStories {
        self.inner.clone()
    }

    pub fn story_list<T: AsRef<StoryList>>(&mut self, story_list: T) -> &mut Self {
        self.inner.story_list = story_list.as_ref().clone();
        self
    }
}

impl AsRef<LoadActiveStories> for LoadActiveStories {
    fn as_ref(&self) -> &LoadActiveStories {
        self
    }
}

impl AsRef<LoadActiveStories> for LoadActiveStoriesBuilder {
    fn as_ref(&self) -> &LoadActiveStories {
        &self.inner
    }
}
