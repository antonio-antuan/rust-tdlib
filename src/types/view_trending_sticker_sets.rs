use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs the server that some trending sticker sets have been viewed by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewTrendingStickerSets {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of viewed trending sticker sets

    #[serde(deserialize_with = "super::_common::vec_of_i64_from_str")]
    #[serde(default)]
    sticker_set_ids: Vec<i64>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ViewTrendingStickerSets {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ViewTrendingStickerSets {}

impl ViewTrendingStickerSets {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ViewTrendingStickerSetsBuilder {
        let mut inner = ViewTrendingStickerSets::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "viewTrendingStickerSets".to_string();

        ViewTrendingStickerSetsBuilder { inner }
    }

    pub fn sticker_set_ids(&self) -> &Vec<i64> {
        &self.sticker_set_ids
    }
}

#[doc(hidden)]
pub struct ViewTrendingStickerSetsBuilder {
    inner: ViewTrendingStickerSets,
}

#[deprecated]
pub type RTDViewTrendingStickerSetsBuilder = ViewTrendingStickerSetsBuilder;

impl ViewTrendingStickerSetsBuilder {
    pub fn build(&self) -> ViewTrendingStickerSets {
        self.inner.clone()
    }

    pub fn sticker_set_ids(&mut self, sticker_set_ids: Vec<i64>) -> &mut Self {
        self.inner.sticker_set_ids = sticker_set_ids;
        self
    }
}

impl AsRef<ViewTrendingStickerSets> for ViewTrendingStickerSets {
    fn as_ref(&self) -> &ViewTrendingStickerSets {
        self
    }
}

impl AsRef<ViewTrendingStickerSets> for ViewTrendingStickerSetsBuilder {
    fn as_ref(&self) -> &ViewTrendingStickerSets {
        &self.inner
    }
}
