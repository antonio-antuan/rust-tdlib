use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the different types of activity in a chat
pub trait TDChatAction: Debug + RObject {}

/// Describes the different types of activity in a chat
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatAction {
    #[doc(hidden)]
    _Default,
    /// The user has canceled the previous action
    #[serde(rename = "chatActionCancel")]
    Cancel(ChatActionCancel),
    /// The user is picking a contact to send
    #[serde(rename = "chatActionChoosingContact")]
    ChoosingContact(ChatActionChoosingContact),
    /// The user is picking a location or venue to send
    #[serde(rename = "chatActionChoosingLocation")]
    ChoosingLocation(ChatActionChoosingLocation),
    /// The user is picking a sticker to send
    #[serde(rename = "chatActionChoosingSticker")]
    ChoosingSticker(ChatActionChoosingSticker),
    /// The user is recording a video
    #[serde(rename = "chatActionRecordingVideo")]
    RecordingVideo(ChatActionRecordingVideo),
    /// The user is recording a video note
    #[serde(rename = "chatActionRecordingVideoNote")]
    RecordingVideoNote(ChatActionRecordingVideoNote),
    /// The user is recording a voice note
    #[serde(rename = "chatActionRecordingVoiceNote")]
    RecordingVoiceNote(ChatActionRecordingVoiceNote),
    /// The user has started to play a game
    #[serde(rename = "chatActionStartPlayingGame")]
    StartPlayingGame(ChatActionStartPlayingGame),
    /// The user is typing a message
    #[serde(rename = "chatActionTyping")]
    Typing(ChatActionTyping),
    /// The user is uploading a document
    #[serde(rename = "chatActionUploadingDocument")]
    UploadingDocument(ChatActionUploadingDocument),
    /// The user is uploading a photo
    #[serde(rename = "chatActionUploadingPhoto")]
    UploadingPhoto(ChatActionUploadingPhoto),
    /// The user is uploading a video
    #[serde(rename = "chatActionUploadingVideo")]
    UploadingVideo(ChatActionUploadingVideo),
    /// The user is uploading a video note
    #[serde(rename = "chatActionUploadingVideoNote")]
    UploadingVideoNote(ChatActionUploadingVideoNote),
    /// The user is uploading a voice note
    #[serde(rename = "chatActionUploadingVoiceNote")]
    UploadingVoiceNote(ChatActionUploadingVoiceNote),
    /// The user is watching animations sent by the other party by clicking on an animated emoji
    #[serde(rename = "chatActionWatchingAnimations")]
    WatchingAnimations(ChatActionWatchingAnimations),
}

impl Default for ChatAction {
    fn default() -> Self {
        ChatAction::_Default
    }
}

impl RObject for ChatAction {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatAction::Cancel(t) => t.extra(),
            ChatAction::ChoosingContact(t) => t.extra(),
            ChatAction::ChoosingLocation(t) => t.extra(),
            ChatAction::ChoosingSticker(t) => t.extra(),
            ChatAction::RecordingVideo(t) => t.extra(),
            ChatAction::RecordingVideoNote(t) => t.extra(),
            ChatAction::RecordingVoiceNote(t) => t.extra(),
            ChatAction::StartPlayingGame(t) => t.extra(),
            ChatAction::Typing(t) => t.extra(),
            ChatAction::UploadingDocument(t) => t.extra(),
            ChatAction::UploadingPhoto(t) => t.extra(),
            ChatAction::UploadingVideo(t) => t.extra(),
            ChatAction::UploadingVideoNote(t) => t.extra(),
            ChatAction::UploadingVoiceNote(t) => t.extra(),
            ChatAction::WatchingAnimations(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatAction::Cancel(t) => t.client_id(),
            ChatAction::ChoosingContact(t) => t.client_id(),
            ChatAction::ChoosingLocation(t) => t.client_id(),
            ChatAction::ChoosingSticker(t) => t.client_id(),
            ChatAction::RecordingVideo(t) => t.client_id(),
            ChatAction::RecordingVideoNote(t) => t.client_id(),
            ChatAction::RecordingVoiceNote(t) => t.client_id(),
            ChatAction::StartPlayingGame(t) => t.client_id(),
            ChatAction::Typing(t) => t.client_id(),
            ChatAction::UploadingDocument(t) => t.client_id(),
            ChatAction::UploadingPhoto(t) => t.client_id(),
            ChatAction::UploadingVideo(t) => t.client_id(),
            ChatAction::UploadingVideoNote(t) => t.client_id(),
            ChatAction::UploadingVoiceNote(t) => t.client_id(),
            ChatAction::WatchingAnimations(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatAction::_Default)
    }
}

impl AsRef<ChatAction> for ChatAction {
    fn as_ref(&self) -> &ChatAction {
        self
    }
}

/// The user has canceled the previous action
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionCancel {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionCancel {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionCancel {}

impl ChatActionCancel {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionCancelBuilder {
        let mut inner = ChatActionCancel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionCancelBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionCancelBuilder {
    inner: ChatActionCancel,
}

#[deprecated]
pub type RTDChatActionCancelBuilder = ChatActionCancelBuilder;

impl ChatActionCancelBuilder {
    pub fn build(&self) -> ChatActionCancel {
        self.inner.clone()
    }
}

impl AsRef<ChatActionCancel> for ChatActionCancel {
    fn as_ref(&self) -> &ChatActionCancel {
        self
    }
}

impl AsRef<ChatActionCancel> for ChatActionCancelBuilder {
    fn as_ref(&self) -> &ChatActionCancel {
        &self.inner
    }
}

/// The user is picking a contact to send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionChoosingContact {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionChoosingContact {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionChoosingContact {}

impl ChatActionChoosingContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionChoosingContactBuilder {
        let mut inner = ChatActionChoosingContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionChoosingContactBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionChoosingContactBuilder {
    inner: ChatActionChoosingContact,
}

#[deprecated]
pub type RTDChatActionChoosingContactBuilder = ChatActionChoosingContactBuilder;

impl ChatActionChoosingContactBuilder {
    pub fn build(&self) -> ChatActionChoosingContact {
        self.inner.clone()
    }
}

impl AsRef<ChatActionChoosingContact> for ChatActionChoosingContact {
    fn as_ref(&self) -> &ChatActionChoosingContact {
        self
    }
}

impl AsRef<ChatActionChoosingContact> for ChatActionChoosingContactBuilder {
    fn as_ref(&self) -> &ChatActionChoosingContact {
        &self.inner
    }
}

/// The user is picking a location or venue to send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionChoosingLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionChoosingLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionChoosingLocation {}

impl ChatActionChoosingLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionChoosingLocationBuilder {
        let mut inner = ChatActionChoosingLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionChoosingLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionChoosingLocationBuilder {
    inner: ChatActionChoosingLocation,
}

#[deprecated]
pub type RTDChatActionChoosingLocationBuilder = ChatActionChoosingLocationBuilder;

impl ChatActionChoosingLocationBuilder {
    pub fn build(&self) -> ChatActionChoosingLocation {
        self.inner.clone()
    }
}

impl AsRef<ChatActionChoosingLocation> for ChatActionChoosingLocation {
    fn as_ref(&self) -> &ChatActionChoosingLocation {
        self
    }
}

impl AsRef<ChatActionChoosingLocation> for ChatActionChoosingLocationBuilder {
    fn as_ref(&self) -> &ChatActionChoosingLocation {
        &self.inner
    }
}

/// The user is picking a sticker to send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionChoosingSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionChoosingSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionChoosingSticker {}

impl ChatActionChoosingSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionChoosingStickerBuilder {
        let mut inner = ChatActionChoosingSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionChoosingStickerBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionChoosingStickerBuilder {
    inner: ChatActionChoosingSticker,
}

#[deprecated]
pub type RTDChatActionChoosingStickerBuilder = ChatActionChoosingStickerBuilder;

impl ChatActionChoosingStickerBuilder {
    pub fn build(&self) -> ChatActionChoosingSticker {
        self.inner.clone()
    }
}

impl AsRef<ChatActionChoosingSticker> for ChatActionChoosingSticker {
    fn as_ref(&self) -> &ChatActionChoosingSticker {
        self
    }
}

impl AsRef<ChatActionChoosingSticker> for ChatActionChoosingStickerBuilder {
    fn as_ref(&self) -> &ChatActionChoosingSticker {
        &self.inner
    }
}

/// The user is recording a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionRecordingVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionRecordingVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionRecordingVideo {}

impl ChatActionRecordingVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionRecordingVideoBuilder {
        let mut inner = ChatActionRecordingVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionRecordingVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionRecordingVideoBuilder {
    inner: ChatActionRecordingVideo,
}

#[deprecated]
pub type RTDChatActionRecordingVideoBuilder = ChatActionRecordingVideoBuilder;

impl ChatActionRecordingVideoBuilder {
    pub fn build(&self) -> ChatActionRecordingVideo {
        self.inner.clone()
    }
}

impl AsRef<ChatActionRecordingVideo> for ChatActionRecordingVideo {
    fn as_ref(&self) -> &ChatActionRecordingVideo {
        self
    }
}

impl AsRef<ChatActionRecordingVideo> for ChatActionRecordingVideoBuilder {
    fn as_ref(&self) -> &ChatActionRecordingVideo {
        &self.inner
    }
}

/// The user is recording a video note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionRecordingVideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionRecordingVideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionRecordingVideoNote {}

impl ChatActionRecordingVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionRecordingVideoNoteBuilder {
        let mut inner = ChatActionRecordingVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionRecordingVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionRecordingVideoNoteBuilder {
    inner: ChatActionRecordingVideoNote,
}

#[deprecated]
pub type RTDChatActionRecordingVideoNoteBuilder = ChatActionRecordingVideoNoteBuilder;

impl ChatActionRecordingVideoNoteBuilder {
    pub fn build(&self) -> ChatActionRecordingVideoNote {
        self.inner.clone()
    }
}

impl AsRef<ChatActionRecordingVideoNote> for ChatActionRecordingVideoNote {
    fn as_ref(&self) -> &ChatActionRecordingVideoNote {
        self
    }
}

impl AsRef<ChatActionRecordingVideoNote> for ChatActionRecordingVideoNoteBuilder {
    fn as_ref(&self) -> &ChatActionRecordingVideoNote {
        &self.inner
    }
}

/// The user is recording a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionRecordingVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionRecordingVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionRecordingVoiceNote {}

impl ChatActionRecordingVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionRecordingVoiceNoteBuilder {
        let mut inner = ChatActionRecordingVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionRecordingVoiceNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionRecordingVoiceNoteBuilder {
    inner: ChatActionRecordingVoiceNote,
}

#[deprecated]
pub type RTDChatActionRecordingVoiceNoteBuilder = ChatActionRecordingVoiceNoteBuilder;

impl ChatActionRecordingVoiceNoteBuilder {
    pub fn build(&self) -> ChatActionRecordingVoiceNote {
        self.inner.clone()
    }
}

impl AsRef<ChatActionRecordingVoiceNote> for ChatActionRecordingVoiceNote {
    fn as_ref(&self) -> &ChatActionRecordingVoiceNote {
        self
    }
}

impl AsRef<ChatActionRecordingVoiceNote> for ChatActionRecordingVoiceNoteBuilder {
    fn as_ref(&self) -> &ChatActionRecordingVoiceNote {
        &self.inner
    }
}

/// The user has started to play a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionStartPlayingGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionStartPlayingGame {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionStartPlayingGame {}

impl ChatActionStartPlayingGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionStartPlayingGameBuilder {
        let mut inner = ChatActionStartPlayingGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionStartPlayingGameBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionStartPlayingGameBuilder {
    inner: ChatActionStartPlayingGame,
}

#[deprecated]
pub type RTDChatActionStartPlayingGameBuilder = ChatActionStartPlayingGameBuilder;

impl ChatActionStartPlayingGameBuilder {
    pub fn build(&self) -> ChatActionStartPlayingGame {
        self.inner.clone()
    }
}

impl AsRef<ChatActionStartPlayingGame> for ChatActionStartPlayingGame {
    fn as_ref(&self) -> &ChatActionStartPlayingGame {
        self
    }
}

impl AsRef<ChatActionStartPlayingGame> for ChatActionStartPlayingGameBuilder {
    fn as_ref(&self) -> &ChatActionStartPlayingGame {
        &self.inner
    }
}

/// The user is typing a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionTyping {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatActionTyping {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionTyping {}

impl ChatActionTyping {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionTypingBuilder {
        let mut inner = ChatActionTyping::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionTypingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatActionTypingBuilder {
    inner: ChatActionTyping,
}

#[deprecated]
pub type RTDChatActionTypingBuilder = ChatActionTypingBuilder;

impl ChatActionTypingBuilder {
    pub fn build(&self) -> ChatActionTyping {
        self.inner.clone()
    }
}

impl AsRef<ChatActionTyping> for ChatActionTyping {
    fn as_ref(&self) -> &ChatActionTyping {
        self
    }
}

impl AsRef<ChatActionTyping> for ChatActionTypingBuilder {
    fn as_ref(&self) -> &ChatActionTyping {
        &self.inner
    }
}

/// The user is uploading a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Upload progress, as a percentage

    #[serde(default)]
    progress: i32,
}

impl RObject for ChatActionUploadingDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionUploadingDocument {}

impl ChatActionUploadingDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionUploadingDocumentBuilder {
        let mut inner = ChatActionUploadingDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionUploadingDocumentBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct ChatActionUploadingDocumentBuilder {
    inner: ChatActionUploadingDocument,
}

#[deprecated]
pub type RTDChatActionUploadingDocumentBuilder = ChatActionUploadingDocumentBuilder;

impl ChatActionUploadingDocumentBuilder {
    pub fn build(&self) -> ChatActionUploadingDocument {
        self.inner.clone()
    }

    pub fn progress(&mut self, progress: i32) -> &mut Self {
        self.inner.progress = progress;
        self
    }
}

impl AsRef<ChatActionUploadingDocument> for ChatActionUploadingDocument {
    fn as_ref(&self) -> &ChatActionUploadingDocument {
        self
    }
}

impl AsRef<ChatActionUploadingDocument> for ChatActionUploadingDocumentBuilder {
    fn as_ref(&self) -> &ChatActionUploadingDocument {
        &self.inner
    }
}

/// The user is uploading a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Upload progress, as a percentage

    #[serde(default)]
    progress: i32,
}

impl RObject for ChatActionUploadingPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionUploadingPhoto {}

impl ChatActionUploadingPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionUploadingPhotoBuilder {
        let mut inner = ChatActionUploadingPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionUploadingPhotoBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct ChatActionUploadingPhotoBuilder {
    inner: ChatActionUploadingPhoto,
}

#[deprecated]
pub type RTDChatActionUploadingPhotoBuilder = ChatActionUploadingPhotoBuilder;

impl ChatActionUploadingPhotoBuilder {
    pub fn build(&self) -> ChatActionUploadingPhoto {
        self.inner.clone()
    }

    pub fn progress(&mut self, progress: i32) -> &mut Self {
        self.inner.progress = progress;
        self
    }
}

impl AsRef<ChatActionUploadingPhoto> for ChatActionUploadingPhoto {
    fn as_ref(&self) -> &ChatActionUploadingPhoto {
        self
    }
}

impl AsRef<ChatActionUploadingPhoto> for ChatActionUploadingPhotoBuilder {
    fn as_ref(&self) -> &ChatActionUploadingPhoto {
        &self.inner
    }
}

/// The user is uploading a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Upload progress, as a percentage

    #[serde(default)]
    progress: i32,
}

impl RObject for ChatActionUploadingVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionUploadingVideo {}

impl ChatActionUploadingVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionUploadingVideoBuilder {
        let mut inner = ChatActionUploadingVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionUploadingVideoBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct ChatActionUploadingVideoBuilder {
    inner: ChatActionUploadingVideo,
}

#[deprecated]
pub type RTDChatActionUploadingVideoBuilder = ChatActionUploadingVideoBuilder;

impl ChatActionUploadingVideoBuilder {
    pub fn build(&self) -> ChatActionUploadingVideo {
        self.inner.clone()
    }

    pub fn progress(&mut self, progress: i32) -> &mut Self {
        self.inner.progress = progress;
        self
    }
}

impl AsRef<ChatActionUploadingVideo> for ChatActionUploadingVideo {
    fn as_ref(&self) -> &ChatActionUploadingVideo {
        self
    }
}

impl AsRef<ChatActionUploadingVideo> for ChatActionUploadingVideoBuilder {
    fn as_ref(&self) -> &ChatActionUploadingVideo {
        &self.inner
    }
}

/// The user is uploading a video note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingVideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Upload progress, as a percentage

    #[serde(default)]
    progress: i32,
}

impl RObject for ChatActionUploadingVideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionUploadingVideoNote {}

impl ChatActionUploadingVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionUploadingVideoNoteBuilder {
        let mut inner = ChatActionUploadingVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionUploadingVideoNoteBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct ChatActionUploadingVideoNoteBuilder {
    inner: ChatActionUploadingVideoNote,
}

#[deprecated]
pub type RTDChatActionUploadingVideoNoteBuilder = ChatActionUploadingVideoNoteBuilder;

impl ChatActionUploadingVideoNoteBuilder {
    pub fn build(&self) -> ChatActionUploadingVideoNote {
        self.inner.clone()
    }

    pub fn progress(&mut self, progress: i32) -> &mut Self {
        self.inner.progress = progress;
        self
    }
}

impl AsRef<ChatActionUploadingVideoNote> for ChatActionUploadingVideoNote {
    fn as_ref(&self) -> &ChatActionUploadingVideoNote {
        self
    }
}

impl AsRef<ChatActionUploadingVideoNote> for ChatActionUploadingVideoNoteBuilder {
    fn as_ref(&self) -> &ChatActionUploadingVideoNote {
        &self.inner
    }
}

/// The user is uploading a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Upload progress, as a percentage

    #[serde(default)]
    progress: i32,
}

impl RObject for ChatActionUploadingVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionUploadingVoiceNote {}

impl ChatActionUploadingVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionUploadingVoiceNoteBuilder {
        let mut inner = ChatActionUploadingVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionUploadingVoiceNoteBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct ChatActionUploadingVoiceNoteBuilder {
    inner: ChatActionUploadingVoiceNote,
}

#[deprecated]
pub type RTDChatActionUploadingVoiceNoteBuilder = ChatActionUploadingVoiceNoteBuilder;

impl ChatActionUploadingVoiceNoteBuilder {
    pub fn build(&self) -> ChatActionUploadingVoiceNote {
        self.inner.clone()
    }

    pub fn progress(&mut self, progress: i32) -> &mut Self {
        self.inner.progress = progress;
        self
    }
}

impl AsRef<ChatActionUploadingVoiceNote> for ChatActionUploadingVoiceNote {
    fn as_ref(&self) -> &ChatActionUploadingVoiceNote {
        self
    }
}

impl AsRef<ChatActionUploadingVoiceNote> for ChatActionUploadingVoiceNoteBuilder {
    fn as_ref(&self) -> &ChatActionUploadingVoiceNote {
        &self.inner
    }
}

/// The user is watching animations sent by the other party by clicking on an animated emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionWatchingAnimations {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The animated emoji

    #[serde(default)]
    emoji: String,
}

impl RObject for ChatActionWatchingAnimations {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatAction for ChatActionWatchingAnimations {}

impl ChatActionWatchingAnimations {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatActionWatchingAnimationsBuilder {
        let mut inner = ChatActionWatchingAnimations::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatActionWatchingAnimationsBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }
}

#[doc(hidden)]
pub struct ChatActionWatchingAnimationsBuilder {
    inner: ChatActionWatchingAnimations,
}

#[deprecated]
pub type RTDChatActionWatchingAnimationsBuilder = ChatActionWatchingAnimationsBuilder;

impl ChatActionWatchingAnimationsBuilder {
    pub fn build(&self) -> ChatActionWatchingAnimations {
        self.inner.clone()
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }
}

impl AsRef<ChatActionWatchingAnimations> for ChatActionWatchingAnimations {
    fn as_ref(&self) -> &ChatActionWatchingAnimations {
        self
    }
}

impl AsRef<ChatActionWatchingAnimations> for ChatActionWatchingAnimationsBuilder {
    fn as_ref(&self) -> &ChatActionWatchingAnimations {
        &self.inner
    }
}
