use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique story identifier among stories of the given sender

    #[serde(default)]
    story_id: i32,
    /// Point in time (Unix timestamp) when the story was published

    #[serde(default)]
    date: i32,
    /// True, if the story is available only to close friends

    #[serde(default)]
    is_for_close_friends: bool,
}

impl RObject for StoryInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StoryInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryInfoBuilder {
        let mut inner = StoryInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryInfoBuilder { inner }
    }

    pub fn story_id(&self) -> i32 {
        self.story_id
    }

    pub fn date(&self) -> i32 {
        self.date
    }

    pub fn is_for_close_friends(&self) -> bool {
        self.is_for_close_friends
    }
}

#[doc(hidden)]
pub struct StoryInfoBuilder {
    inner: StoryInfo,
}

#[deprecated]
pub type RTDStoryInfoBuilder = StoryInfoBuilder;

impl StoryInfoBuilder {
    pub fn build(&self) -> StoryInfo {
        self.inner.clone()
    }

    pub fn story_id(&mut self, story_id: i32) -> &mut Self {
        self.inner.story_id = story_id;
        self
    }

    pub fn date(&mut self, date: i32) -> &mut Self {
        self.inner.date = date;
        self
    }

    pub fn is_for_close_friends(&mut self, is_for_close_friends: bool) -> &mut Self {
        self.inner.is_for_close_friends = is_for_close_friends;
        self
    }
}

impl AsRef<StoryInfo> for StoryInfo {
    fn as_ref(&self) -> &StoryInfo {
        self
    }
}

impl AsRef<StoryInfo> for StoryInfoBuilder {
    fn as_ref(&self) -> &StoryInfo {
        &self.inner
    }
}
