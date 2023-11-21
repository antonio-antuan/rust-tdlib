use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about interactions with a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInteractionInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of times the story was viewed

    #[serde(default)]
    view_count: i32,
    /// Number of times the story was forwarded; 0 if none or unknown

    #[serde(default)]
    forward_count: i32,
    /// Number of reactions added to the story; 0 if none or unknown

    #[serde(default)]
    reaction_count: i32,
    /// Identifiers of at most 3 recent viewers of the story

    #[serde(default)]
    recent_viewer_user_ids: Vec<i64>,
}

impl RObject for StoryInteractionInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StoryInteractionInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryInteractionInfoBuilder {
        let mut inner = StoryInteractionInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryInteractionInfoBuilder { inner }
    }

    pub fn view_count(&self) -> i32 {
        self.view_count
    }

    pub fn forward_count(&self) -> i32 {
        self.forward_count
    }

    pub fn reaction_count(&self) -> i32 {
        self.reaction_count
    }

    pub fn recent_viewer_user_ids(&self) -> &Vec<i64> {
        &self.recent_viewer_user_ids
    }
}

#[doc(hidden)]
pub struct StoryInteractionInfoBuilder {
    inner: StoryInteractionInfo,
}

#[deprecated]
pub type RTDStoryInteractionInfoBuilder = StoryInteractionInfoBuilder;

impl StoryInteractionInfoBuilder {
    pub fn build(&self) -> StoryInteractionInfo {
        self.inner.clone()
    }

    pub fn view_count(&mut self, view_count: i32) -> &mut Self {
        self.inner.view_count = view_count;
        self
    }

    pub fn forward_count(&mut self, forward_count: i32) -> &mut Self {
        self.inner.forward_count = forward_count;
        self
    }

    pub fn reaction_count(&mut self, reaction_count: i32) -> &mut Self {
        self.inner.reaction_count = reaction_count;
        self
    }

    pub fn recent_viewer_user_ids(&mut self, recent_viewer_user_ids: Vec<i64>) -> &mut Self {
        self.inner.recent_viewer_user_ids = recent_viewer_user_ids;
        self
    }
}

impl AsRef<StoryInteractionInfo> for StoryInteractionInfo {
    fn as_ref(&self) -> &StoryInteractionInfo {
        self
    }
}

impl AsRef<StoryInteractionInfo> for StoryInteractionInfoBuilder {
    fn as_ref(&self) -> &StoryInteractionInfo {
        &self.inner
    }
}
