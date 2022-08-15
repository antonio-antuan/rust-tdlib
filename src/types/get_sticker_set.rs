use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a sticker set by its identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the sticker set

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    set_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetStickerSet {}

impl GetStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetStickerSetBuilder {
        let mut inner = GetStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStickerSet".to_string();

        GetStickerSetBuilder { inner }
    }

    pub fn set_id(&self) -> i64 {
        self.set_id
    }
}

#[doc(hidden)]
pub struct GetStickerSetBuilder {
    inner: GetStickerSet,
}

#[deprecated]
pub type RTDGetStickerSetBuilder = GetStickerSetBuilder;

impl GetStickerSetBuilder {
    pub fn build(&self) -> GetStickerSet {
        self.inner.clone()
    }

    pub fn set_id(&mut self, set_id: i64) -> &mut Self {
        self.inner.set_id = set_id;
        self
    }
}

impl AsRef<GetStickerSet> for GetStickerSet {
    fn as_ref(&self) -> &GetStickerSet {
        self
    }
}

impl AsRef<GetStickerSet> for GetStickerSetBuilder {
    fn as_ref(&self) -> &GetStickerSet {
        &self.inner
    }
}
