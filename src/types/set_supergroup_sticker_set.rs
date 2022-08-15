use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Changes the sticker set of a supergroup; requires can_change_info administrator right
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetSupergroupStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup

    #[serde(default)]
    supergroup_id: i64,
    /// New value of the supergroup sticker set identifier. Use 0 to remove the supergroup sticker set

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    sticker_set_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetSupergroupStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetSupergroupStickerSet {}

impl SetSupergroupStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetSupergroupStickerSetBuilder {
        let mut inner = SetSupergroupStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setSupergroupStickerSet".to_string();

        SetSupergroupStickerSetBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn sticker_set_id(&self) -> i64 {
        self.sticker_set_id
    }
}

#[doc(hidden)]
pub struct SetSupergroupStickerSetBuilder {
    inner: SetSupergroupStickerSet,
}

#[deprecated]
pub type RTDSetSupergroupStickerSetBuilder = SetSupergroupStickerSetBuilder;

impl SetSupergroupStickerSetBuilder {
    pub fn build(&self) -> SetSupergroupStickerSet {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self {
        self.inner.sticker_set_id = sticker_set_id;
        self
    }
}

impl AsRef<SetSupergroupStickerSet> for SetSupergroupStickerSet {
    fn as_ref(&self) -> &SetSupergroupStickerSet {
        self
    }
}

impl AsRef<SetSupergroupStickerSet> for SetSupergroupStickerSetBuilder {
    fn as_ref(&self) -> &SetSupergroupStickerSet {
        &self.inner
    }
}
