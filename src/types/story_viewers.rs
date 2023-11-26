use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of story viewers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryViewers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total number of story viewers found

    #[serde(default)]
    total_count: i32,
    /// Approximate total number of reactions set by found story viewers

    #[serde(default)]
    total_reaction_count: i32,
    /// List of story viewers

    #[serde(default)]
    viewers: Vec<StoryViewer>,
    /// The offset for the next request. If empty, there are no more results

    #[serde(default)]
    next_offset: String,
}

impl RObject for StoryViewers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StoryViewers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryViewersBuilder {
        let mut inner = StoryViewers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryViewersBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn total_reaction_count(&self) -> i32 {
        self.total_reaction_count
    }

    pub fn viewers(&self) -> &Vec<StoryViewer> {
        &self.viewers
    }

    pub fn next_offset(&self) -> &String {
        &self.next_offset
    }
}

#[doc(hidden)]
pub struct StoryViewersBuilder {
    inner: StoryViewers,
}

#[deprecated]
pub type RTDStoryViewersBuilder = StoryViewersBuilder;

impl StoryViewersBuilder {
    pub fn build(&self) -> StoryViewers {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn total_reaction_count(&mut self, total_reaction_count: i32) -> &mut Self {
        self.inner.total_reaction_count = total_reaction_count;
        self
    }

    pub fn viewers(&mut self, viewers: Vec<StoryViewer>) -> &mut Self {
        self.inner.viewers = viewers;
        self
    }

    pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
        self.inner.next_offset = next_offset.as_ref().to_string();
        self
    }
}

impl AsRef<StoryViewers> for StoryViewers {
    fn as_ref(&self) -> &StoryViewers {
        self
    }
}

impl AsRef<StoryViewers> for StoryViewersBuilder {
    fn as_ref(&self) -> &StoryViewers {
        &self.inner
    }
}
