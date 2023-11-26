use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Deleted a sticker set; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set name

    #[serde(default)]
    name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteStickerSet {}

impl DeleteStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DeleteStickerSetBuilder {
        let mut inner = DeleteStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteStickerSet".to_string();

        DeleteStickerSetBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct DeleteStickerSetBuilder {
    inner: DeleteStickerSet,
}

#[deprecated]
pub type RTDDeleteStickerSetBuilder = DeleteStickerSetBuilder;

impl DeleteStickerSetBuilder {
    pub fn build(&self) -> DeleteStickerSet {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<DeleteStickerSet> for DeleteStickerSet {
    fn as_ref(&self) -> &DeleteStickerSet {
        self
    }
}

impl AsRef<DeleteStickerSet> for DeleteStickerSetBuilder {
    fn as_ref(&self) -> &DeleteStickerSet {
        &self.inner
    }
}
