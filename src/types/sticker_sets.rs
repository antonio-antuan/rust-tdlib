use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total number of sticker sets found

    #[serde(default)]
    total_count: i32,
    /// List of sticker sets

    #[serde(default)]
    sets: Vec<StickerSetInfo>,
}

impl RObject for StickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerSetsBuilder {
        let mut inner = StickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerSetsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn sets(&self) -> &Vec<StickerSetInfo> {
        &self.sets
    }
}

#[doc(hidden)]
pub struct StickerSetsBuilder {
    inner: StickerSets,
}

#[deprecated]
pub type RTDStickerSetsBuilder = StickerSetsBuilder;

impl StickerSetsBuilder {
    pub fn build(&self) -> StickerSets {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn sets(&mut self, sets: Vec<StickerSetInfo>) -> &mut Self {
        self.inner.sets = sets;
        self
    }
}

impl AsRef<StickerSets> for StickerSets {
    fn as_ref(&self) -> &StickerSets {
        self
    }
}

impl AsRef<StickerSets> for StickerSetsBuilder {
    fn as_ref(&self) -> &StickerSets {
        &self.inner
    }
}
