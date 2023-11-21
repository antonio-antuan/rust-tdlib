use crate::errors::Result;
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

    #[serde(default)]
    user_id: i64,
    /// Sticker set title; 1-64 characters

    #[serde(default)]
    title: String,
    /// Sticker set name. Can contain only English letters, digits and underscores. Must end with *"_by_<bot username>"* (*<bot_username>* is case insensitive) for bots; 1-64 characters

    #[serde(default)]
    name: String,
    /// Format of the stickers in the set

    #[serde(skip_serializing_if = "StickerFormat::_is_default")]
    sticker_format: StickerFormat,
    /// Type of the stickers in the set

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,
    /// Pass true if stickers in the sticker set must be repainted; for custom emoji sticker sets only

    #[serde(default)]
    needs_repainting: bool,
    /// List of stickers to be added to the set; must be non-empty. All stickers must have the same format. For TGS stickers, uploadStickerFile must be used before the sticker is shown

    #[serde(default)]
    stickers: Vec<InputSticker>,
    /// Source of the sticker set; may be empty if unknown

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateNewStickerSetBuilder {
        let mut inner = CreateNewStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createNewStickerSet".to_string();

        CreateNewStickerSetBuilder { inner }
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

    pub fn sticker_format(&self) -> &StickerFormat {
        &self.sticker_format
    }

    pub fn sticker_type(&self) -> &StickerType {
        &self.sticker_type
    }

    pub fn needs_repainting(&self) -> bool {
        self.needs_repainting
    }

    pub fn stickers(&self) -> &Vec<InputSticker> {
        &self.stickers
    }

    pub fn source(&self) -> &String {
        &self.source
    }
}

#[doc(hidden)]
pub struct CreateNewStickerSetBuilder {
    inner: CreateNewStickerSet,
}

#[deprecated]
pub type RTDCreateNewStickerSetBuilder = CreateNewStickerSetBuilder;

impl CreateNewStickerSetBuilder {
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

    pub fn sticker_format<T: AsRef<StickerFormat>>(&mut self, sticker_format: T) -> &mut Self {
        self.inner.sticker_format = sticker_format.as_ref().clone();
        self
    }

    pub fn sticker_type<T: AsRef<StickerType>>(&mut self, sticker_type: T) -> &mut Self {
        self.inner.sticker_type = sticker_type.as_ref().clone();
        self
    }

    pub fn needs_repainting(&mut self, needs_repainting: bool) -> &mut Self {
        self.inner.needs_repainting = needs_repainting;
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

impl AsRef<CreateNewStickerSet> for CreateNewStickerSetBuilder {
    fn as_ref(&self) -> &CreateNewStickerSet {
        &self.inner
    }
}
