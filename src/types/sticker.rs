use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique sticker identifier within the set; 0 if none

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Identifier of the sticker set to which the sticker belongs; 0 if none

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    set_id: i64,
    /// Sticker width; as defined by the sender

    #[serde(default)]
    width: i32,
    /// Sticker height; as defined by the sender

    #[serde(default)]
    height: i32,
    /// Emoji corresponding to the sticker

    #[serde(default)]
    emoji: String,
    /// Sticker format

    #[serde(skip_serializing_if = "StickerFormat::_is_default")]
    format: StickerFormat,
    /// Sticker's full type

    #[serde(skip_serializing_if = "StickerFullType::_is_default")]
    full_type: StickerFullType,
    /// Sticker's outline represented as a list of closed vector paths; may be empty. The coordinate system origin is in the upper-left corner

    #[serde(default)]
    outline: Vec<ClosedVectorPath>,
    /// Sticker thumbnail in WEBP or JPEG format; may be null
    thumbnail: Option<Thumbnail>,
    /// File containing the sticker
    sticker: File,
}

impl RObject for Sticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Sticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StickerBuilder {
        let mut inner = Sticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StickerBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn set_id(&self) -> i64 {
        self.set_id
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }

    pub fn format(&self) -> &StickerFormat {
        &self.format
    }

    pub fn full_type(&self) -> &StickerFullType {
        &self.full_type
    }

    pub fn outline(&self) -> &Vec<ClosedVectorPath> {
        &self.outline
    }

    pub fn thumbnail(&self) -> &Option<Thumbnail> {
        &self.thumbnail
    }

    pub fn sticker(&self) -> &File {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct StickerBuilder {
    inner: Sticker,
}

#[deprecated]
pub type RTDStickerBuilder = StickerBuilder;

impl StickerBuilder {
    pub fn build(&self) -> Sticker {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn set_id(&mut self, set_id: i64) -> &mut Self {
        self.inner.set_id = set_id;
        self
    }

    pub fn width(&mut self, width: i32) -> &mut Self {
        self.inner.width = width;
        self
    }

    pub fn height(&mut self, height: i32) -> &mut Self {
        self.inner.height = height;
        self
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }

    pub fn format<T: AsRef<StickerFormat>>(&mut self, format: T) -> &mut Self {
        self.inner.format = format.as_ref().clone();
        self
    }

    pub fn full_type<T: AsRef<StickerFullType>>(&mut self, full_type: T) -> &mut Self {
        self.inner.full_type = full_type.as_ref().clone();
        self
    }

    pub fn outline(&mut self, outline: Vec<ClosedVectorPath>) -> &mut Self {
        self.inner.outline = outline;
        self
    }

    pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = Some(thumbnail.as_ref().clone());
        self
    }

    pub fn sticker<T: AsRef<File>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<Sticker> for Sticker {
    fn as_ref(&self) -> &Sticker {
        self
    }
}

impl AsRef<Sticker> for StickerBuilder {
    fn as_ref(&self) -> &Sticker {
        &self.inner
    }
}
