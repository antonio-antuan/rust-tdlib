use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents the type of a file
pub trait TDFileType: Debug + RObject {}

/// Represents the type of a file
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum FileType {
    #[doc(hidden)]
    _Default(()),
    /// The file is an animation
    Animation(FileTypeAnimation),
    /// The file is an audio file
    Audio(FileTypeAudio),
    /// The file is a document
    Document(FileTypeDocument),
    /// The data is not a file
    None(FileTypeNone),
    /// The file is a photo
    Photo(FileTypePhoto),
    /// The file is a profile photo
    ProfilePhoto(FileTypeProfilePhoto),
    /// The file was sent to a secret chat (the file type is not known to the server)
    Secret(FileTypeSecret),
    /// The file is a thumbnail of a file from a secret chat
    SecretThumbnail(FileTypeSecretThumbnail),
    /// The file is a file from Secure storage used for storing Telegram Passport files
    Secure(FileTypeSecure),
    /// The file is a sticker
    Sticker(FileTypeSticker),
    /// The file is a thumbnail of another file
    Thumbnail(FileTypeThumbnail),
    /// The file type is not yet known
    Unknown(FileTypeUnknown),
    /// The file is a video
    Video(FileTypeVideo),
    /// The file is a video note
    VideoNote(FileTypeVideoNote),
    /// The file is a voice note
    VoiceNote(FileTypeVoiceNote),
    /// The file is a wallpaper or a background pattern
    Wallpaper(FileTypeWallpaper),
}

impl Default for FileType {
    fn default() -> Self {
        FileType::_Default(())
    }
}

impl<'de> Deserialize<'de> for FileType {
    fn deserialize<D>(deserializer: D) -> Result<FileType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          FileType,
          (fileTypeAnimation, Animation);
          (fileTypeAudio, Audio);
          (fileTypeDocument, Document);
          (fileTypeNone, None);
          (fileTypePhoto, Photo);
          (fileTypeProfilePhoto, ProfilePhoto);
          (fileTypeSecret, Secret);
          (fileTypeSecretThumbnail, SecretThumbnail);
          (fileTypeSecure, Secure);
          (fileTypeSticker, Sticker);
          (fileTypeThumbnail, Thumbnail);
          (fileTypeUnknown, Unknown);
          (fileTypeVideo, Video);
          (fileTypeVideoNote, VideoNote);
          (fileTypeVoiceNote, VoiceNote);
          (fileTypeWallpaper, Wallpaper);

        )(deserializer)
    }
}

impl RObject for FileType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            FileType::Animation(t) => t.td_name(),
            FileType::Audio(t) => t.td_name(),
            FileType::Document(t) => t.td_name(),
            FileType::None(t) => t.td_name(),
            FileType::Photo(t) => t.td_name(),
            FileType::ProfilePhoto(t) => t.td_name(),
            FileType::Secret(t) => t.td_name(),
            FileType::SecretThumbnail(t) => t.td_name(),
            FileType::Secure(t) => t.td_name(),
            FileType::Sticker(t) => t.td_name(),
            FileType::Thumbnail(t) => t.td_name(),
            FileType::Unknown(t) => t.td_name(),
            FileType::Video(t) => t.td_name(),
            FileType::VideoNote(t) => t.td_name(),
            FileType::VoiceNote(t) => t.td_name(),
            FileType::Wallpaper(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
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
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
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
        matches!(self, FileType::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeAnimation {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeAnimation"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeAnimation {}

impl FileTypeAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeAnimationBuilder {
        let mut inner = FileTypeAnimation::default();
        inner.td_name = "fileTypeAnimation".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeAudio {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeAudio"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeAudio {}

impl FileTypeAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeAudioBuilder {
        let mut inner = FileTypeAudio::default();
        inner.td_name = "fileTypeAudio".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeDocument {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeDocument"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeDocument {}

impl FileTypeDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeDocumentBuilder {
        let mut inner = FileTypeDocument::default();
        inner.td_name = "fileTypeDocument".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeNone {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeNone"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeNone {}

impl FileTypeNone {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeNoneBuilder {
        let mut inner = FileTypeNone::default();
        inner.td_name = "fileTypeNone".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypePhoto {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypePhoto"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypePhoto {}

impl FileTypePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypePhotoBuilder {
        let mut inner = FileTypePhoto::default();
        inner.td_name = "fileTypePhoto".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeProfilePhoto {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeProfilePhoto"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeProfilePhoto {}

impl FileTypeProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeProfilePhotoBuilder {
        let mut inner = FileTypeProfilePhoto::default();
        inner.td_name = "fileTypeProfilePhoto".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSecret {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeSecret"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeSecret {}

impl FileTypeSecret {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeSecretBuilder {
        let mut inner = FileTypeSecret::default();
        inner.td_name = "fileTypeSecret".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSecretThumbnail {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeSecretThumbnail"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeSecretThumbnail {}

impl FileTypeSecretThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeSecretThumbnailBuilder {
        let mut inner = FileTypeSecretThumbnail::default();
        inner.td_name = "fileTypeSecretThumbnail".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSecure {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeSecure"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeSecure {}

impl FileTypeSecure {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeSecureBuilder {
        let mut inner = FileTypeSecure::default();
        inner.td_name = "fileTypeSecure".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeSticker {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeSticker"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeSticker {}

impl FileTypeSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeStickerBuilder {
        let mut inner = FileTypeSticker::default();
        inner.td_name = "fileTypeSticker".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeThumbnail {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeThumbnail"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeThumbnail {}

impl FileTypeThumbnail {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeThumbnailBuilder {
        let mut inner = FileTypeThumbnail::default();
        inner.td_name = "fileTypeThumbnail".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeUnknown {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeUnknown"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeUnknown {}

impl FileTypeUnknown {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeUnknownBuilder {
        let mut inner = FileTypeUnknown::default();
        inner.td_name = "fileTypeUnknown".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeVideo {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeVideo"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeVideo {}

impl FileTypeVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeVideoBuilder {
        let mut inner = FileTypeVideo::default();
        inner.td_name = "fileTypeVideo".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeVideoNote {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeVideoNote"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeVideoNote {}

impl FileTypeVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeVideoNoteBuilder {
        let mut inner = FileTypeVideoNote::default();
        inner.td_name = "fileTypeVideoNote".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeVoiceNote {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeVoiceNote"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeVoiceNote {}

impl FileTypeVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeVoiceNoteBuilder {
        let mut inner = FileTypeVoiceNote::default();
        inner.td_name = "fileTypeVoiceNote".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for FileTypeWallpaper {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "fileTypeWallpaper"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDFileType for FileTypeWallpaper {}

impl FileTypeWallpaper {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDFileTypeWallpaperBuilder {
        let mut inner = FileTypeWallpaper::default();
        inner.td_name = "fileTypeWallpaper".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
