use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents the type of a file
pub trait TDFileType: Debug + RObject {}

/// Represents the type of a file
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum FileType {
    #[doc(hidden)]
    _Default,
    /// The file is an animation
    #[serde(rename = "fileTypeAnimation")]
    Animation(FileTypeAnimation),
    /// The file is an audio file
    #[serde(rename = "fileTypeAudio")]
    Audio(FileTypeAudio),
    /// The file is a document
    #[serde(rename = "fileTypeDocument")]
    Document(FileTypeDocument),
    /// The data is not a file
    #[serde(rename = "fileTypeNone")]
    None(FileTypeNone),
    /// The file is a photo
    #[serde(rename = "fileTypePhoto")]
    Photo(FileTypePhoto),
    /// The file is a profile photo
    #[serde(rename = "fileTypeProfilePhoto")]
    ProfilePhoto(FileTypeProfilePhoto),
    /// The file was sent to a secret chat (the file type is not known to the server)
    #[serde(rename = "fileTypeSecret")]
    Secret(FileTypeSecret),
    /// The file is a thumbnail of a file from a secret chat
    #[serde(rename = "fileTypeSecretThumbnail")]
    SecretThumbnail(FileTypeSecretThumbnail),
    /// The file is a file from Secure storage used for storing Telegram Passport files
    #[serde(rename = "fileTypeSecure")]
    Secure(FileTypeSecure),
    /// The file is a sticker
    #[serde(rename = "fileTypeSticker")]
    Sticker(FileTypeSticker),
    /// The file is a thumbnail of another file
    #[serde(rename = "fileTypeThumbnail")]
    Thumbnail(FileTypeThumbnail),
    /// The file type is not yet known
    #[serde(rename = "fileTypeUnknown")]
    Unknown(FileTypeUnknown),
    /// The file is a video
    #[serde(rename = "fileTypeVideo")]
    Video(FileTypeVideo),
    /// The file is a video note
    #[serde(rename = "fileTypeVideoNote")]
    VideoNote(FileTypeVideoNote),
    /// The file is a voice note
    #[serde(rename = "fileTypeVoiceNote")]
    VoiceNote(FileTypeVoiceNote),
    /// The file is a wallpaper or a background pattern
    #[serde(rename = "fileTypeWallpaper")]
    Wallpaper(FileTypeWallpaper),
}

impl Default for FileType {
    fn default() -> Self {
        FileType::_Default
    }
}

impl RObject for FileType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            FileType::Animation(t) => t.extra(),
            FileType::Audio(t) => t.extra(),
            FileType::Document(t) => t.extra(),
            FileType::None(t) => t.extra(),
            FileType::Photo(t) => t.extra(),
            FileType::ProfilePhoto(t) => t.extra(),
            FileType::Secret(t) => t.extra(),
            FileType::SecretThumbnail(t) => t.extra(),
            FileType::Secure(t) => t.extra(),
            FileType::Sticker(t) => t.extra(),
            FileType::Thumbnail(t) => t.extra(),
            FileType::Unknown(t) => t.extra(),
            FileType::Video(t) => t.extra(),
            FileType::VideoNote(t) => t.extra(),
            FileType::VoiceNote(t) => t.extra(),
            FileType::Wallpaper(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            FileType::Animation(t) => t.client_id(),
            FileType::Audio(t) => t.client_id(),
            FileType::Document(t) => t.client_id(),
            FileType::None(t) => t.client_id(),
            FileType::Photo(t) => t.client_id(),
            FileType::ProfilePhoto(t) => t.client_id(),
            FileType::Secret(t) => t.client_id(),
            FileType::SecretThumbnail(t) => t.client_id(),
            FileType::Secure(t) => t.client_id(),
            FileType::Sticker(t) => t.client_id(),
            FileType::Thumbnail(t) => t.client_id(),
            FileType::Unknown(t) => t.client_id(),
            FileType::Video(t) => t.client_id(),
            FileType::VideoNote(t) => t.client_id(),
            FileType::VoiceNote(t) => t.client_id(),
            FileType::Wallpaper(t) => t.client_id(),

            _ => None,
        }
    }
}

impl FileType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, FileType::_Default)
    }
}

impl AsRef<FileType> for FileType {
    fn as_ref(&self) -> &FileType {
        self
    }
}

/// The file is an animation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeAnimation {}

impl FileTypeAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeAnimationBuilder {
        let mut inner = FileTypeAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeAnimationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeAnimationBuilder {
    inner: FileTypeAnimation,
}

#[deprecated]
pub type RTDFileTypeAnimationBuilder = FileTypeAnimationBuilder;

impl FileTypeAnimationBuilder {
    pub fn build(&self) -> FileTypeAnimation {
        self.inner.clone()
    }
}

impl AsRef<FileTypeAnimation> for FileTypeAnimation {
    fn as_ref(&self) -> &FileTypeAnimation {
        self
    }
}

impl AsRef<FileTypeAnimation> for FileTypeAnimationBuilder {
    fn as_ref(&self) -> &FileTypeAnimation {
        &self.inner
    }
}

/// The file is an audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeAudio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeAudio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeAudio {}

impl FileTypeAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeAudioBuilder {
        let mut inner = FileTypeAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeAudioBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeAudioBuilder {
    inner: FileTypeAudio,
}

#[deprecated]
pub type RTDFileTypeAudioBuilder = FileTypeAudioBuilder;

impl FileTypeAudioBuilder {
    pub fn build(&self) -> FileTypeAudio {
        self.inner.clone()
    }
}

impl AsRef<FileTypeAudio> for FileTypeAudio {
    fn as_ref(&self) -> &FileTypeAudio {
        self
    }
}

impl AsRef<FileTypeAudio> for FileTypeAudioBuilder {
    fn as_ref(&self) -> &FileTypeAudio {
        &self.inner
    }
}

/// The file is a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeDocument {}

impl FileTypeDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeDocumentBuilder {
        let mut inner = FileTypeDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeDocumentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeDocumentBuilder {
    inner: FileTypeDocument,
}

#[deprecated]
pub type RTDFileTypeDocumentBuilder = FileTypeDocumentBuilder;

impl FileTypeDocumentBuilder {
    pub fn build(&self) -> FileTypeDocument {
        self.inner.clone()
    }
}

impl AsRef<FileTypeDocument> for FileTypeDocument {
    fn as_ref(&self) -> &FileTypeDocument {
        self
    }
}

impl AsRef<FileTypeDocument> for FileTypeDocumentBuilder {
    fn as_ref(&self) -> &FileTypeDocument {
        &self.inner
    }
}

/// The data is not a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeNone {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeNone {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeNone {}

impl FileTypeNone {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeNoneBuilder {
        let mut inner = FileTypeNone::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeNoneBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeNoneBuilder {
    inner: FileTypeNone,
}

#[deprecated]
pub type RTDFileTypeNoneBuilder = FileTypeNoneBuilder;

impl FileTypeNoneBuilder {
    pub fn build(&self) -> FileTypeNone {
        self.inner.clone()
    }
}

impl AsRef<FileTypeNone> for FileTypeNone {
    fn as_ref(&self) -> &FileTypeNone {
        self
    }
}

impl AsRef<FileTypeNone> for FileTypeNoneBuilder {
    fn as_ref(&self) -> &FileTypeNone {
        &self.inner
    }
}

/// The file is a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypePhoto {}

impl FileTypePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypePhotoBuilder {
        let mut inner = FileTypePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypePhotoBuilder {
    inner: FileTypePhoto,
}

#[deprecated]
pub type RTDFileTypePhotoBuilder = FileTypePhotoBuilder;

impl FileTypePhotoBuilder {
    pub fn build(&self) -> FileTypePhoto {
        self.inner.clone()
    }
}

impl AsRef<FileTypePhoto> for FileTypePhoto {
    fn as_ref(&self) -> &FileTypePhoto {
        self
    }
}

impl AsRef<FileTypePhoto> for FileTypePhotoBuilder {
    fn as_ref(&self) -> &FileTypePhoto {
        &self.inner
    }
}

/// The file is a profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeProfilePhoto {}

impl FileTypeProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeProfilePhotoBuilder {
        let mut inner = FileTypeProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeProfilePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeProfilePhotoBuilder {
    inner: FileTypeProfilePhoto,
}

#[deprecated]
pub type RTDFileTypeProfilePhotoBuilder = FileTypeProfilePhotoBuilder;

impl FileTypeProfilePhotoBuilder {
    pub fn build(&self) -> FileTypeProfilePhoto {
        self.inner.clone()
    }
}

impl AsRef<FileTypeProfilePhoto> for FileTypeProfilePhoto {
    fn as_ref(&self) -> &FileTypeProfilePhoto {
        self
    }
}

impl AsRef<FileTypeProfilePhoto> for FileTypeProfilePhotoBuilder {
    fn as_ref(&self) -> &FileTypeProfilePhoto {
        &self.inner
    }
}

/// The file was sent to a secret chat (the file type is not known to the server)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecret {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSecret {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeSecret {}

impl FileTypeSecret {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeSecretBuilder {
        let mut inner = FileTypeSecret::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeSecretBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeSecretBuilder {
    inner: FileTypeSecret,
}

#[deprecated]
pub type RTDFileTypeSecretBuilder = FileTypeSecretBuilder;

impl FileTypeSecretBuilder {
    pub fn build(&self) -> FileTypeSecret {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSecret> for FileTypeSecret {
    fn as_ref(&self) -> &FileTypeSecret {
        self
    }
}

impl AsRef<FileTypeSecret> for FileTypeSecretBuilder {
    fn as_ref(&self) -> &FileTypeSecret {
        &self.inner
    }
}

/// The file is a thumbnail of a file from a secret chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecretThumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSecretThumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeSecretThumbnail {}

impl FileTypeSecretThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeSecretThumbnailBuilder {
        let mut inner = FileTypeSecretThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeSecretThumbnailBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeSecretThumbnailBuilder {
    inner: FileTypeSecretThumbnail,
}

#[deprecated]
pub type RTDFileTypeSecretThumbnailBuilder = FileTypeSecretThumbnailBuilder;

impl FileTypeSecretThumbnailBuilder {
    pub fn build(&self) -> FileTypeSecretThumbnail {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSecretThumbnail> for FileTypeSecretThumbnail {
    fn as_ref(&self) -> &FileTypeSecretThumbnail {
        self
    }
}

impl AsRef<FileTypeSecretThumbnail> for FileTypeSecretThumbnailBuilder {
    fn as_ref(&self) -> &FileTypeSecretThumbnail {
        &self.inner
    }
}

/// The file is a file from Secure storage used for storing Telegram Passport files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSecure {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSecure {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeSecure {}

impl FileTypeSecure {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeSecureBuilder {
        let mut inner = FileTypeSecure::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeSecureBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeSecureBuilder {
    inner: FileTypeSecure,
}

#[deprecated]
pub type RTDFileTypeSecureBuilder = FileTypeSecureBuilder;

impl FileTypeSecureBuilder {
    pub fn build(&self) -> FileTypeSecure {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSecure> for FileTypeSecure {
    fn as_ref(&self) -> &FileTypeSecure {
        self
    }
}

impl AsRef<FileTypeSecure> for FileTypeSecureBuilder {
    fn as_ref(&self) -> &FileTypeSecure {
        &self.inner
    }
}

/// The file is a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeSticker {}

impl FileTypeSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeStickerBuilder {
        let mut inner = FileTypeSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeStickerBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeStickerBuilder {
    inner: FileTypeSticker,
}

#[deprecated]
pub type RTDFileTypeStickerBuilder = FileTypeStickerBuilder;

impl FileTypeStickerBuilder {
    pub fn build(&self) -> FileTypeSticker {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSticker> for FileTypeSticker {
    fn as_ref(&self) -> &FileTypeSticker {
        self
    }
}

impl AsRef<FileTypeSticker> for FileTypeStickerBuilder {
    fn as_ref(&self) -> &FileTypeSticker {
        &self.inner
    }
}

/// The file is a thumbnail of another file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeThumbnail {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeThumbnail {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeThumbnail {}

impl FileTypeThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeThumbnailBuilder {
        let mut inner = FileTypeThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeThumbnailBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeThumbnailBuilder {
    inner: FileTypeThumbnail,
}

#[deprecated]
pub type RTDFileTypeThumbnailBuilder = FileTypeThumbnailBuilder;

impl FileTypeThumbnailBuilder {
    pub fn build(&self) -> FileTypeThumbnail {
        self.inner.clone()
    }
}

impl AsRef<FileTypeThumbnail> for FileTypeThumbnail {
    fn as_ref(&self) -> &FileTypeThumbnail {
        self
    }
}

impl AsRef<FileTypeThumbnail> for FileTypeThumbnailBuilder {
    fn as_ref(&self) -> &FileTypeThumbnail {
        &self.inner
    }
}

/// The file type is not yet known
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeUnknown {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeUnknown {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeUnknown {}

impl FileTypeUnknown {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeUnknownBuilder {
        let mut inner = FileTypeUnknown::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeUnknownBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeUnknownBuilder {
    inner: FileTypeUnknown,
}

#[deprecated]
pub type RTDFileTypeUnknownBuilder = FileTypeUnknownBuilder;

impl FileTypeUnknownBuilder {
    pub fn build(&self) -> FileTypeUnknown {
        self.inner.clone()
    }
}

impl AsRef<FileTypeUnknown> for FileTypeUnknown {
    fn as_ref(&self) -> &FileTypeUnknown {
        self
    }
}

impl AsRef<FileTypeUnknown> for FileTypeUnknownBuilder {
    fn as_ref(&self) -> &FileTypeUnknown {
        &self.inner
    }
}

/// The file is a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeVideo {}

impl FileTypeVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeVideoBuilder {
        let mut inner = FileTypeVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeVideoBuilder {
    inner: FileTypeVideo,
}

#[deprecated]
pub type RTDFileTypeVideoBuilder = FileTypeVideoBuilder;

impl FileTypeVideoBuilder {
    pub fn build(&self) -> FileTypeVideo {
        self.inner.clone()
    }
}

impl AsRef<FileTypeVideo> for FileTypeVideo {
    fn as_ref(&self) -> &FileTypeVideo {
        self
    }
}

impl AsRef<FileTypeVideo> for FileTypeVideoBuilder {
    fn as_ref(&self) -> &FileTypeVideo {
        &self.inner
    }
}

/// The file is a video note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeVideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeVideoNote {}

impl FileTypeVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeVideoNoteBuilder {
        let mut inner = FileTypeVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeVideoNoteBuilder {
    inner: FileTypeVideoNote,
}

#[deprecated]
pub type RTDFileTypeVideoNoteBuilder = FileTypeVideoNoteBuilder;

impl FileTypeVideoNoteBuilder {
    pub fn build(&self) -> FileTypeVideoNote {
        self.inner.clone()
    }
}

impl AsRef<FileTypeVideoNote> for FileTypeVideoNote {
    fn as_ref(&self) -> &FileTypeVideoNote {
        self
    }
}

impl AsRef<FileTypeVideoNote> for FileTypeVideoNoteBuilder {
    fn as_ref(&self) -> &FileTypeVideoNote {
        &self.inner
    }
}

/// The file is a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeVoiceNote {}

impl FileTypeVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeVoiceNoteBuilder {
        let mut inner = FileTypeVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeVoiceNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeVoiceNoteBuilder {
    inner: FileTypeVoiceNote,
}

#[deprecated]
pub type RTDFileTypeVoiceNoteBuilder = FileTypeVoiceNoteBuilder;

impl FileTypeVoiceNoteBuilder {
    pub fn build(&self) -> FileTypeVoiceNote {
        self.inner.clone()
    }
}

impl AsRef<FileTypeVoiceNote> for FileTypeVoiceNote {
    fn as_ref(&self) -> &FileTypeVoiceNote {
        self
    }
}

impl AsRef<FileTypeVoiceNote> for FileTypeVoiceNoteBuilder {
    fn as_ref(&self) -> &FileTypeVoiceNote {
        &self.inner
    }
}

/// The file is a wallpaper or a background pattern
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileTypeWallpaper {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeWallpaper {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDFileType for FileTypeWallpaper {}

impl FileTypeWallpaper {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> FileTypeWallpaperBuilder {
        let mut inner = FileTypeWallpaper::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        FileTypeWallpaperBuilder { inner }
    }
}

#[doc(hidden)]
pub struct FileTypeWallpaperBuilder {
    inner: FileTypeWallpaper,
}

#[deprecated]
pub type RTDFileTypeWallpaperBuilder = FileTypeWallpaperBuilder;

impl FileTypeWallpaperBuilder {
    pub fn build(&self) -> FileTypeWallpaper {
        self.inner.clone()
    }
}

impl AsRef<FileTypeWallpaper> for FileTypeWallpaper {
    fn as_ref(&self) -> &FileTypeWallpaper {
        self
    }
}

impl AsRef<FileTypeWallpaper> for FileTypeWallpaperBuilder {
    fn as_ref(&self) -> &FileTypeWallpaper {
        &self.inner
    }
}
