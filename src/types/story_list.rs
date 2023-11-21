use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a list of stories
pub trait TDStoryList: Debug + RObject {}

/// Describes a list of stories
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StoryList {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The list of stories, shown in the Arvhive chat list
    #[serde(rename = "storyListArchive")]
    Archive(StoryListArchive),
    /// The list of stories, shown in the main chat list and folder chat lists
    #[serde(rename = "storyListMain")]
    Main(StoryListMain),
}

impl RObject for StoryList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StoryList::Archive(t) => t.extra(),
            StoryList::Main(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StoryList::Archive(t) => t.client_id(),
            StoryList::Main(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StoryList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StoryList::_Default)
    }
}

impl AsRef<StoryList> for StoryList {
    fn as_ref(&self) -> &StoryList {
        self
    }
}

/// The list of stories, shown in the Arvhive chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryListArchive {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StoryListArchive {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryList for StoryListArchive {}

impl StoryListArchive {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryListArchiveBuilder {
        let mut inner = StoryListArchive::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryListArchiveBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StoryListArchiveBuilder {
    inner: StoryListArchive,
}

#[deprecated]
pub type RTDStoryListArchiveBuilder = StoryListArchiveBuilder;

impl StoryListArchiveBuilder {
    pub fn build(&self) -> StoryListArchive {
        self.inner.clone()
    }
}

impl AsRef<StoryListArchive> for StoryListArchive {
    fn as_ref(&self) -> &StoryListArchive {
        self
    }
}

impl AsRef<StoryListArchive> for StoryListArchiveBuilder {
    fn as_ref(&self) -> &StoryListArchive {
        &self.inner
    }
}

/// The list of stories, shown in the main chat list and folder chat lists
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryListMain {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StoryListMain {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryList for StoryListMain {}

impl StoryListMain {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryListMainBuilder {
        let mut inner = StoryListMain::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryListMainBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StoryListMainBuilder {
    inner: StoryListMain,
}

#[deprecated]
pub type RTDStoryListMainBuilder = StoryListMainBuilder;

impl StoryListMainBuilder {
    pub fn build(&self) -> StoryListMain {
        self.inner.clone()
    }
}

impl AsRef<StoryListMain> for StoryListMain {
    fn as_ref(&self) -> &StoryListMain {
        self
    }
}

impl AsRef<StoryListMain> for StoryListMainBuilder {
    fn as_ref(&self) -> &StoryListMain {
        &self.inner
    }
}
