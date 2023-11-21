use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a type of a block list
pub trait TDBlockList: Debug + RObject {}

/// Describes a type of a block list
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum BlockList {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The main block list that disallows writing messages to the current user, receiving their status and photo, viewing of stories, and some other actions
    #[serde(rename = "blockListMain")]
    Main(BlockListMain),
    /// The block list that disallows viewing of stories of the current user
    #[serde(rename = "blockListStories")]
    Stories(BlockListStories),
}

impl RObject for BlockList {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            BlockList::Main(t) => t.extra(),
            BlockList::Stories(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            BlockList::Main(t) => t.client_id(),
            BlockList::Stories(t) => t.client_id(),

            _ => None,
        }
    }
}

impl BlockList {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, BlockList::_Default)
    }
}

impl AsRef<BlockList> for BlockList {
    fn as_ref(&self) -> &BlockList {
        self
    }
}

/// The main block list that disallows writing messages to the current user, receiving their status and photo, viewing of stories, and some other actions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockListMain {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BlockListMain {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBlockList for BlockListMain {}

impl BlockListMain {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BlockListMainBuilder {
        let mut inner = BlockListMain::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BlockListMainBuilder { inner }
    }
}

#[doc(hidden)]
pub struct BlockListMainBuilder {
    inner: BlockListMain,
}

#[deprecated]
pub type RTDBlockListMainBuilder = BlockListMainBuilder;

impl BlockListMainBuilder {
    pub fn build(&self) -> BlockListMain {
        self.inner.clone()
    }
}

impl AsRef<BlockListMain> for BlockListMain {
    fn as_ref(&self) -> &BlockListMain {
        self
    }
}

impl AsRef<BlockListMain> for BlockListMainBuilder {
    fn as_ref(&self) -> &BlockListMain {
        &self.inner
    }
}

/// The block list that disallows viewing of stories of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockListStories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for BlockListStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDBlockList for BlockListStories {}

impl BlockListStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> BlockListStoriesBuilder {
        let mut inner = BlockListStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        BlockListStoriesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct BlockListStoriesBuilder {
    inner: BlockListStories,
}

#[deprecated]
pub type RTDBlockListStoriesBuilder = BlockListStoriesBuilder;

impl BlockListStoriesBuilder {
    pub fn build(&self) -> BlockListStories {
        self.inner.clone()
    }
}

impl AsRef<BlockListStories> for BlockListStories {
    fn as_ref(&self) -> &BlockListStories {
        self
    }
}

impl AsRef<BlockListStories> for BlockListStoriesBuilder {
    fn as_ref(&self) -> &BlockListStories {
        &self.inner
    }
}
