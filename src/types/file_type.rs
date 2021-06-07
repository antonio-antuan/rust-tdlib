use crate::errors::*;
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
    #[serde(rename(serialize = "fileTypeAnimation", deserialize = "fileTypeAnimation"))]
    Animation(FileTypeAnimation),
    /// The file is an audio file
    #[serde(rename(serialize = "fileTypeAudio", deserialize = "fileTypeAudio"))]
    Audio(FileTypeAudio),
    /// The file is a document
    #[serde(rename(serialize = "fileTypeDocument", deserialize = "fileTypeDocument"))]
    Document(FileTypeDocument),
    /// The data is not a file
    #[serde(rename(serialize = "fileTypeNone", deserialize = "fileTypeNone"))]
    None(FileTypeNone),
    /// The file is a photo
    #[serde(rename(serialize = "fileTypePhoto", deserialize = "fileTypePhoto"))]
    Photo(FileTypePhoto),
    /// The file is a profile photo
    #[serde(rename(
        serialize = "fileTypeProfilePhoto",
        deserialize = "fileTypeProfilePhoto"
    ))]
    ProfilePhoto(FileTypeProfilePhoto),
    /// The file was sent to a secret chat (the file type is not known to the server)
    #[serde(rename(serialize = "fileTypeSecret", deserialize = "fileTypeSecret"))]
    Secret(FileTypeSecret),
    /// The file is a thumbnail of a file from a secret chat
    #[serde(rename(
        serialize = "fileTypeSecretThumbnail",
        deserialize = "fileTypeSecretThumbnail"
    ))]
    SecretThumbnail(FileTypeSecretThumbnail),
    /// The file is a file from Secure storage used for storing Telegram Passport files
    #[serde(rename(serialize = "fileTypeSecure", deserialize = "fileTypeSecure"))]
    Secure(FileTypeSecure),
    /// The file is a sticker
    #[serde(rename(serialize = "fileTypeSticker", deserialize = "fileTypeSticker"))]
    Sticker(FileTypeSticker),
    /// The file is a thumbnail of another file
    #[serde(rename(serialize = "fileTypeThumbnail", deserialize = "fileTypeThumbnail"))]
    Thumbnail(FileTypeThumbnail),
    /// The file type is not yet known
    #[serde(rename(serialize = "fileTypeUnknown", deserialize = "fileTypeUnknown"))]
    Unknown(FileTypeUnknown),
    /// The file is a video
    #[serde(rename(serialize = "fileTypeVideo", deserialize = "fileTypeVideo"))]
    Video(FileTypeVideo),
    /// The file is a video note
    #[serde(rename(serialize = "fileTypeVideoNote", deserialize = "fileTypeVideoNote"))]
    VideoNote(FileTypeVideoNote),
    /// The file is a voice note
    #[serde(rename(serialize = "fileTypeVoiceNote", deserialize = "fileTypeVoiceNote"))]
    VoiceNote(FileTypeVoiceNote),
    /// The file is a wallpaper or a background pattern
    #[serde(rename(serialize = "fileTypeWallpaper", deserialize = "fileTypeWallpaper"))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeAnimationBuilder {
        let mut inner = FileTypeAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeAnimationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeAnimationBuilder {
    inner: FileTypeAnimation,
}

impl RTDFileTypeAnimationBuilder {
    pub fn build(&self) -> FileTypeAnimation {
        self.inner.clone()
    }
}

impl AsRef<FileTypeAnimation> for FileTypeAnimation {
    fn as_ref(&self) -> &FileTypeAnimation {
        self
    }
}

impl AsRef<FileTypeAnimation> for RTDFileTypeAnimationBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeAudioBuilder {
        let mut inner = FileTypeAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeAudioBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeAudioBuilder {
    inner: FileTypeAudio,
}

impl RTDFileTypeAudioBuilder {
    pub fn build(&self) -> FileTypeAudio {
        self.inner.clone()
    }
}

impl AsRef<FileTypeAudio> for FileTypeAudio {
    fn as_ref(&self) -> &FileTypeAudio {
        self
    }
}

impl AsRef<FileTypeAudio> for RTDFileTypeAudioBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeDocumentBuilder {
        let mut inner = FileTypeDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeDocumentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeDocumentBuilder {
    inner: FileTypeDocument,
}

impl RTDFileTypeDocumentBuilder {
    pub fn build(&self) -> FileTypeDocument {
        self.inner.clone()
    }
}

impl AsRef<FileTypeDocument> for FileTypeDocument {
    fn as_ref(&self) -> &FileTypeDocument {
        self
    }
}

impl AsRef<FileTypeDocument> for RTDFileTypeDocumentBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeNoneBuilder {
        let mut inner = FileTypeNone::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeNoneBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeNoneBuilder {
    inner: FileTypeNone,
}

impl RTDFileTypeNoneBuilder {
    pub fn build(&self) -> FileTypeNone {
        self.inner.clone()
    }
}

impl AsRef<FileTypeNone> for FileTypeNone {
    fn as_ref(&self) -> &FileTypeNone {
        self
    }
}

impl AsRef<FileTypeNone> for RTDFileTypeNoneBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypePhotoBuilder {
        let mut inner = FileTypePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypePhotoBuilder {
    inner: FileTypePhoto,
}

impl RTDFileTypePhotoBuilder {
    pub fn build(&self) -> FileTypePhoto {
        self.inner.clone()
    }
}

impl AsRef<FileTypePhoto> for FileTypePhoto {
    fn as_ref(&self) -> &FileTypePhoto {
        self
    }
}

impl AsRef<FileTypePhoto> for RTDFileTypePhotoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeProfilePhotoBuilder {
        let mut inner = FileTypeProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeProfilePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeProfilePhotoBuilder {
    inner: FileTypeProfilePhoto,
}

impl RTDFileTypeProfilePhotoBuilder {
    pub fn build(&self) -> FileTypeProfilePhoto {
        self.inner.clone()
    }
}

impl AsRef<FileTypeProfilePhoto> for FileTypeProfilePhoto {
    fn as_ref(&self) -> &FileTypeProfilePhoto {
        self
    }
}

impl AsRef<FileTypeProfilePhoto> for RTDFileTypeProfilePhotoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeSecretBuilder {
        let mut inner = FileTypeSecret::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeSecretBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeSecretBuilder {
    inner: FileTypeSecret,
}

impl RTDFileTypeSecretBuilder {
    pub fn build(&self) -> FileTypeSecret {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSecret> for FileTypeSecret {
    fn as_ref(&self) -> &FileTypeSecret {
        self
    }
}

impl AsRef<FileTypeSecret> for RTDFileTypeSecretBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeSecretThumbnailBuilder {
        let mut inner = FileTypeSecretThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeSecretThumbnailBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeSecretThumbnailBuilder {
    inner: FileTypeSecretThumbnail,
}

impl RTDFileTypeSecretThumbnailBuilder {
    pub fn build(&self) -> FileTypeSecretThumbnail {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSecretThumbnail> for FileTypeSecretThumbnail {
    fn as_ref(&self) -> &FileTypeSecretThumbnail {
        self
    }
}

impl AsRef<FileTypeSecretThumbnail> for RTDFileTypeSecretThumbnailBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeSecureBuilder {
        let mut inner = FileTypeSecure::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeSecureBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeSecureBuilder {
    inner: FileTypeSecure,
}

impl RTDFileTypeSecureBuilder {
    pub fn build(&self) -> FileTypeSecure {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSecure> for FileTypeSecure {
    fn as_ref(&self) -> &FileTypeSecure {
        self
    }
}

impl AsRef<FileTypeSecure> for RTDFileTypeSecureBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeStickerBuilder {
        let mut inner = FileTypeSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeStickerBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeStickerBuilder {
    inner: FileTypeSticker,
}

impl RTDFileTypeStickerBuilder {
    pub fn build(&self) -> FileTypeSticker {
        self.inner.clone()
    }
}

impl AsRef<FileTypeSticker> for FileTypeSticker {
    fn as_ref(&self) -> &FileTypeSticker {
        self
    }
}

impl AsRef<FileTypeSticker> for RTDFileTypeStickerBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeThumbnailBuilder {
        let mut inner = FileTypeThumbnail::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeThumbnailBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeThumbnailBuilder {
    inner: FileTypeThumbnail,
}

impl RTDFileTypeThumbnailBuilder {
    pub fn build(&self) -> FileTypeThumbnail {
        self.inner.clone()
    }
}

impl AsRef<FileTypeThumbnail> for FileTypeThumbnail {
    fn as_ref(&self) -> &FileTypeThumbnail {
        self
    }
}

impl AsRef<FileTypeThumbnail> for RTDFileTypeThumbnailBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeUnknownBuilder {
        let mut inner = FileTypeUnknown::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeUnknownBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeUnknownBuilder {
    inner: FileTypeUnknown,
}

impl RTDFileTypeUnknownBuilder {
    pub fn build(&self) -> FileTypeUnknown {
        self.inner.clone()
    }
}

impl AsRef<FileTypeUnknown> for FileTypeUnknown {
    fn as_ref(&self) -> &FileTypeUnknown {
        self
    }
}

impl AsRef<FileTypeUnknown> for RTDFileTypeUnknownBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeVideoBuilder {
        let mut inner = FileTypeVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeVideoBuilder {
    inner: FileTypeVideo,
}

impl RTDFileTypeVideoBuilder {
    pub fn build(&self) -> FileTypeVideo {
        self.inner.clone()
    }
}

impl AsRef<FileTypeVideo> for FileTypeVideo {
    fn as_ref(&self) -> &FileTypeVideo {
        self
    }
}

impl AsRef<FileTypeVideo> for RTDFileTypeVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeVideoNoteBuilder {
        let mut inner = FileTypeVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeVideoNoteBuilder {
    inner: FileTypeVideoNote,
}

impl RTDFileTypeVideoNoteBuilder {
    pub fn build(&self) -> FileTypeVideoNote {
        self.inner.clone()
    }
}

impl AsRef<FileTypeVideoNote> for FileTypeVideoNote {
    fn as_ref(&self) -> &FileTypeVideoNote {
        self
    }
}

impl AsRef<FileTypeVideoNote> for RTDFileTypeVideoNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeVoiceNoteBuilder {
        let mut inner = FileTypeVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeVoiceNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeVoiceNoteBuilder {
    inner: FileTypeVoiceNote,
}

impl RTDFileTypeVoiceNoteBuilder {
    pub fn build(&self) -> FileTypeVoiceNote {
        self.inner.clone()
    }
}

impl AsRef<FileTypeVoiceNote> for FileTypeVoiceNote {
    fn as_ref(&self) -> &FileTypeVoiceNote {
        self
    }
}

impl AsRef<FileTypeVoiceNote> for RTDFileTypeVoiceNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeWallpaperBuilder {
        let mut inner = FileTypeWallpaper::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDFileTypeWallpaperBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDFileTypeWallpaperBuilder {
    inner: FileTypeWallpaper,
}

impl RTDFileTypeWallpaperBuilder {
    pub fn build(&self) -> FileTypeWallpaper {
        self.inner.clone()
    }
}

impl AsRef<FileTypeWallpaper> for FileTypeWallpaper {
    fn as_ref(&self) -> &FileTypeWallpaper {
        self
    }
}

impl AsRef<FileTypeWallpaper> for RTDFileTypeWallpaperBuilder {
    fn as_ref(&self) -> &FileTypeWallpaper {
        &self.inner
    }
}
