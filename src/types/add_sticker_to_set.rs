use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds a new sticker to a set; for bots only. Returns the sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddStickerToSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set owner
    user_id: i32,
    /// Sticker set name
    name: String,
    /// Sticker to add to the set

    #[serde(skip_serializing_if = "InputSticker::_is_default")]
    sticker: InputSticker,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddStickerToSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddStickerToSet {}

impl AddStickerToSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddStickerToSetBuilder {
        let mut inner = AddStickerToSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addStickerToSet".to_string();

        RTDAddStickerToSetBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn sticker(&self) -> &InputSticker {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RTDAddStickerToSetBuilder {
    inner: AddStickerToSet,
}

impl RTDAddStickerToSetBuilder {
    pub fn build(&self) -> AddStickerToSet {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn sticker<T: AsRef<InputSticker>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<AddStickerToSet> for AddStickerToSet {
    fn as_ref(&self) -> &AddStickerToSet {
        self
    }
}

impl AsRef<AddStickerToSet> for RTDAddStickerToSetBuilder {
    fn as_ref(&self) -> &AddStickerToSet {
        &self.inner
    }
}
