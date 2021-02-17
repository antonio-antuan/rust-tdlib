use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Installs/uninstalls or activates/archives a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChangeStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the sticker set

    #[serde(deserialize_with = "super::_common::number_from_string")]
    set_id: i64,
    /// The new value of is_installed
    is_installed: bool,
    /// The new value of is_archived. A sticker set can't be installed and archived simultaneously
    is_archived: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ChangeStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ChangeStickerSet {}

impl ChangeStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChangeStickerSetBuilder {
        let mut inner = ChangeStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "changeStickerSet".to_string();

        RTDChangeStickerSetBuilder { inner }
    }

    pub fn set_id(&self) -> i64 {
        self.set_id
    }

    pub fn is_installed(&self) -> bool {
        self.is_installed
    }

    pub fn is_archived(&self) -> bool {
        self.is_archived
    }
}

#[doc(hidden)]
pub struct RTDChangeStickerSetBuilder {
    inner: ChangeStickerSet,
}

impl RTDChangeStickerSetBuilder {
    pub fn build(&self) -> ChangeStickerSet {
        self.inner.clone()
    }

    pub fn set_id(&mut self, set_id: i64) -> &mut Self {
        self.inner.set_id = set_id;
        self
    }

    pub fn is_installed(&mut self, is_installed: bool) -> &mut Self {
        self.inner.is_installed = is_installed;
        self
    }

    pub fn is_archived(&mut self, is_archived: bool) -> &mut Self {
        self.inner.is_archived = is_archived;
        self
    }
}

impl AsRef<ChangeStickerSet> for ChangeStickerSet {
    fn as_ref(&self) -> &ChangeStickerSet {
        self
    }
}

impl AsRef<ChangeStickerSet> for RTDChangeStickerSetBuilder {
    fn as_ref(&self) -> &ChangeStickerSet {
        &self.inner
    }
}
