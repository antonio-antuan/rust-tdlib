use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of trending sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrendingStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Approximate total number of trending sticker sets
    total_count: i32,
    /// List of trending sticker sets
    sets: Vec<StickerSetInfo>,
    /// True, if the list contains sticker sets with premium stickers
    is_premium: bool,
}

impl RObject for TrendingStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TrendingStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTrendingStickerSetsBuilder {
        let mut inner = TrendingStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTrendingStickerSetsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn sets(&self) -> &Vec<StickerSetInfo> {
        &self.sets
    }

    pub fn is_premium(&self) -> bool {
        self.is_premium
    }
}

#[doc(hidden)]
pub struct RTDTrendingStickerSetsBuilder {
    inner: TrendingStickerSets,
}

impl RTDTrendingStickerSetsBuilder {
    pub fn build(&self) -> TrendingStickerSets {
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

    pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
        self.inner.is_premium = is_premium;
        self
    }
}

impl AsRef<TrendingStickerSets> for TrendingStickerSets {
    fn as_ref(&self) -> &TrendingStickerSets {
        self
    }
}

impl AsRef<TrendingStickerSets> for RTDTrendingStickerSetsBuilder {
    fn as_ref(&self) -> &TrendingStickerSets {
        &self.inner
    }
}
