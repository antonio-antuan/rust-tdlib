use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Creates a new sticker set. Returns the newly created sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateNewStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker set owner; ignored for regular users
    user_id: i64,
    /// Sticker set title; 1-64 characters
    title: String,
    /// Sticker set name. Can contain only English letters, digits and underscores. Must end with *"_by_<bot username>"* (*<bot_username>* is case insensitive) for bots; 1-64 characters
    name: String,
    /// List of stickers to be added to the set; must be non-empty. All stickers must have the same format. For TGS stickers, uploadStickerFile must be used before the sticker is shown
    stickers: Vec<InputSticker>,
    /// Source of the sticker set; may be empty if unknown
    source: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateNewStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateNewStickerSet {}

impl CreateNewStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateNewStickerSetBuilder {
        let mut inner = CreateNewStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createNewStickerSet".to_string();

        RTDCreateNewStickerSetBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn stickers(&self) -> &Vec<InputSticker> {
        &self.stickers
    }

    pub fn source(&self) -> &String {
        &self.source
    }
}

#[doc(hidden)]
pub struct RTDCreateNewStickerSetBuilder {
    inner: CreateNewStickerSet,
}

impl RTDCreateNewStickerSetBuilder {
    pub fn build(&self) -> CreateNewStickerSet {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn stickers(&mut self, stickers: Vec<InputSticker>) -> &mut Self {
        self.inner.stickers = stickers;
        self
    }

    pub fn source<T: AsRef<str>>(&mut self, source: T) -> &mut Self {
        self.inner.source = source.as_ref().to_string();
        self
    }
}

impl AsRef<CreateNewStickerSet> for CreateNewStickerSet {
    fn as_ref(&self) -> &CreateNewStickerSet {
        self
    }
}

impl AsRef<CreateNewStickerSet> for RTDCreateNewStickerSetBuilder {
    fn as_ref(&self) -> &CreateNewStickerSet {
        &self.inner
    }
}
