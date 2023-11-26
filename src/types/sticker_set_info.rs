use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents short information about a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerSetInfo {
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
    id: i64,
    /// Title of the sticker set

    #[serde(default)]
    title: String,
    /// Name of the sticker set

    #[serde(default)]
    name: String,
    /// Sticker set thumbnail in WEBP, TGS, or WEBM format with width and height 100; may be null
    thumbnail: Option<Thumbnail>,
    /// Sticker set thumbnail's outline represented as a list of closed vector paths; may be empty. The coordinate system origin is in the upper-left corner

    #[serde(default)]
    thumbnail_outline: Vec<ClosedVectorPath>,
    /// True, if the sticker set has been installed by the current user

    #[serde(default)]
    is_installed: bool,
    /// True, if the sticker set has been archived. A sticker set can't be installed and archived simultaneously

    #[serde(default)]
    is_archived: bool,
    /// True, if the sticker set is official

    #[serde(default)]
    is_official: bool,
    /// Format of the stickers in the set

    #[serde(skip_serializing_if = "StickerFormat::_is_default")]
    sticker_format: StickerFormat,
    /// Type of the stickers in the set

    #[serde(skip_serializing_if = "StickerType::_is_default")]
    sticker_type: StickerType,
    /// True, if stickers in the sticker set are custom emoji that must be repainted; for custom emoji sticker sets only

    #[serde(default)]
    needs_repainting: bool,
    /// True for already viewed trending sticker sets

    #[serde(default)]
    is_viewed: bool,
    /// Total number of stickers in the set

    #[serde(default)]
    size: i32,
    /// Up to the first 5 stickers from the set, depending on the context. If the application needs more stickers the full sticker set needs to be requested

    #[serde(default)]
    covers: Vec<Sticker>,
}

impl RObject for StickerSetInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StickerSetInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerSetInfoBuilder {
        let mut inner = StickerSetInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerSetInfoBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }

    pub fn thumbnail_outline(&self) -> &Vec<ClosedVectorPath> {
        &self.thumbnail_outline
    }

    pub fn is_installed(&self) -> bool {
        self.is_installed
    }

    pub fn is_archived(&self) -> bool {
        self.is_archived
    }

    pub fn is_official(&self) -> bool {
        self.is_official
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

    pub fn is_viewed(&self) -> bool {
        self.is_viewed
    }

    pub fn size(&self) -> i32 {
        self.size
    }

    pub fn covers(&self) -> &Vec<Sticker> {
        &self.covers
    }
}

#[doc(hidden)]
pub struct StickerSetInfoBuilder {
    inner: StickerSetInfo,
}

#[deprecated]
pub type RTDStickerSetInfoBuilder = StickerSetInfoBuilder;

impl StickerSetInfoBuilder {
    pub fn build(&self) -> StickerSetInfo {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
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

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }

    pub fn thumbnail_outline(&mut self, thumbnail_outline: Vec<ClosedVectorPath>) -> &mut Self {
        self.inner.thumbnail_outline = thumbnail_outline;
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

    pub fn is_official(&mut self, is_official: bool) -> &mut Self {
        self.inner.is_official = is_official;
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

    pub fn is_viewed(&mut self, is_viewed: bool) -> &mut Self {
        self.inner.is_viewed = is_viewed;
        self
    }

    pub fn size(&mut self, size: i32) -> &mut Self {
        self.inner.size = size;
        self
    }

    pub fn covers(&mut self, covers: Vec<Sticker>) -> &mut Self {
        self.inner.covers = covers;
        self
    }
}

impl AsRef<StickerSetInfo> for StickerSetInfo {
    fn as_ref(&self) -> &StickerSetInfo {
        self
    }
}

impl AsRef<StickerSetInfo> for StickerSetInfoBuilder {
    fn as_ref(&self) -> &StickerSetInfo {
        &self.inner
    }
}
