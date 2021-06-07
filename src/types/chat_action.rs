use crate::errors::*;
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
    /// The user has cancelled the previous action
    #[serde(rename(serialize = "chatActionCancel", deserialize = "chatActionCancel"))]
    Cancel(ChatActionCancel),
    /// The user is picking a contact to send
    #[serde(rename(
        serialize = "chatActionChoosingContact",
        deserialize = "chatActionChoosingContact"
    ))]
    ChoosingContact(ChatActionChoosingContact),
    /// The user is picking a location or venue to send
    #[serde(rename(
        serialize = "chatActionChoosingLocation",
        deserialize = "chatActionChoosingLocation"
    ))]
    ChoosingLocation(ChatActionChoosingLocation),
    /// The user is recording a video
    #[serde(rename(
        serialize = "chatActionRecordingVideo",
        deserialize = "chatActionRecordingVideo"
    ))]
    RecordingVideo(ChatActionRecordingVideo),
    /// The user is recording a video note
    #[serde(rename(
        serialize = "chatActionRecordingVideoNote",
        deserialize = "chatActionRecordingVideoNote"
    ))]
    RecordingVideoNote(ChatActionRecordingVideoNote),
    /// The user is recording a voice note
    #[serde(rename(
        serialize = "chatActionRecordingVoiceNote",
        deserialize = "chatActionRecordingVoiceNote"
    ))]
    RecordingVoiceNote(ChatActionRecordingVoiceNote),
    /// The user has started to play a game
    #[serde(rename(
        serialize = "chatActionStartPlayingGame",
        deserialize = "chatActionStartPlayingGame"
    ))]
    StartPlayingGame(ChatActionStartPlayingGame),
    /// The user is typing a message
    #[serde(rename(serialize = "chatActionTyping", deserialize = "chatActionTyping"))]
    Typing(ChatActionTyping),
    /// The user is uploading a document
    #[serde(rename(
        serialize = "chatActionUploadingDocument",
        deserialize = "chatActionUploadingDocument"
    ))]
    UploadingDocument(ChatActionUploadingDocument),
    /// The user is uploading a photo
    #[serde(rename(
        serialize = "chatActionUploadingPhoto",
        deserialize = "chatActionUploadingPhoto"
    ))]
    UploadingPhoto(ChatActionUploadingPhoto),
    /// The user is uploading a video
    #[serde(rename(
        serialize = "chatActionUploadingVideo",
        deserialize = "chatActionUploadingVideo"
    ))]
    UploadingVideo(ChatActionUploadingVideo),
    /// The user is uploading a video note
    #[serde(rename(
        serialize = "chatActionUploadingVideoNote",
        deserialize = "chatActionUploadingVideoNote"
    ))]
    UploadingVideoNote(ChatActionUploadingVideoNote),
    /// The user is uploading a voice note
    #[serde(rename(
        serialize = "chatActionUploadingVoiceNote",
        deserialize = "chatActionUploadingVoiceNote"
    ))]
    UploadingVoiceNote(ChatActionUploadingVoiceNote),
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

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatAction::Cancel(t) => t.client_id(),
            ChatAction::ChoosingContact(t) => t.client_id(),
            ChatAction::ChoosingLocation(t) => t.client_id(),
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

            _ => None,
        }
    }
}

impl ChatAction {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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

/// The user has cancelled the previous action
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionCancelBuilder {
        let mut inner = ChatActionCancel::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionCancelBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionCancelBuilder {
    inner: ChatActionCancel,
}

impl RTDChatActionCancelBuilder {
    pub fn build(&self) -> ChatActionCancel {
        self.inner.clone()
    }
}

impl AsRef<ChatActionCancel> for ChatActionCancel {
    fn as_ref(&self) -> &ChatActionCancel {
        self
    }
}

impl AsRef<ChatActionCancel> for RTDChatActionCancelBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionChoosingContactBuilder {
        let mut inner = ChatActionChoosingContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionChoosingContactBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionChoosingContactBuilder {
    inner: ChatActionChoosingContact,
}

impl RTDChatActionChoosingContactBuilder {
    pub fn build(&self) -> ChatActionChoosingContact {
        self.inner.clone()
    }
}

impl AsRef<ChatActionChoosingContact> for ChatActionChoosingContact {
    fn as_ref(&self) -> &ChatActionChoosingContact {
        self
    }
}

impl AsRef<ChatActionChoosingContact> for RTDChatActionChoosingContactBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionChoosingLocationBuilder {
        let mut inner = ChatActionChoosingLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionChoosingLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionChoosingLocationBuilder {
    inner: ChatActionChoosingLocation,
}

impl RTDChatActionChoosingLocationBuilder {
    pub fn build(&self) -> ChatActionChoosingLocation {
        self.inner.clone()
    }
}

impl AsRef<ChatActionChoosingLocation> for ChatActionChoosingLocation {
    fn as_ref(&self) -> &ChatActionChoosingLocation {
        self
    }
}

impl AsRef<ChatActionChoosingLocation> for RTDChatActionChoosingLocationBuilder {
    fn as_ref(&self) -> &ChatActionChoosingLocation {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionRecordingVideoBuilder {
        let mut inner = ChatActionRecordingVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionRecordingVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionRecordingVideoBuilder {
    inner: ChatActionRecordingVideo,
}

impl RTDChatActionRecordingVideoBuilder {
    pub fn build(&self) -> ChatActionRecordingVideo {
        self.inner.clone()
    }
}

impl AsRef<ChatActionRecordingVideo> for ChatActionRecordingVideo {
    fn as_ref(&self) -> &ChatActionRecordingVideo {
        self
    }
}

impl AsRef<ChatActionRecordingVideo> for RTDChatActionRecordingVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionRecordingVideoNoteBuilder {
        let mut inner = ChatActionRecordingVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionRecordingVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionRecordingVideoNoteBuilder {
    inner: ChatActionRecordingVideoNote,
}

impl RTDChatActionRecordingVideoNoteBuilder {
    pub fn build(&self) -> ChatActionRecordingVideoNote {
        self.inner.clone()
    }
}

impl AsRef<ChatActionRecordingVideoNote> for ChatActionRecordingVideoNote {
    fn as_ref(&self) -> &ChatActionRecordingVideoNote {
        self
    }
}

impl AsRef<ChatActionRecordingVideoNote> for RTDChatActionRecordingVideoNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionRecordingVoiceNoteBuilder {
        let mut inner = ChatActionRecordingVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionRecordingVoiceNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionRecordingVoiceNoteBuilder {
    inner: ChatActionRecordingVoiceNote,
}

impl RTDChatActionRecordingVoiceNoteBuilder {
    pub fn build(&self) -> ChatActionRecordingVoiceNote {
        self.inner.clone()
    }
}

impl AsRef<ChatActionRecordingVoiceNote> for ChatActionRecordingVoiceNote {
    fn as_ref(&self) -> &ChatActionRecordingVoiceNote {
        self
    }
}

impl AsRef<ChatActionRecordingVoiceNote> for RTDChatActionRecordingVoiceNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionStartPlayingGameBuilder {
        let mut inner = ChatActionStartPlayingGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionStartPlayingGameBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionStartPlayingGameBuilder {
    inner: ChatActionStartPlayingGame,
}

impl RTDChatActionStartPlayingGameBuilder {
    pub fn build(&self) -> ChatActionStartPlayingGame {
        self.inner.clone()
    }
}

impl AsRef<ChatActionStartPlayingGame> for ChatActionStartPlayingGame {
    fn as_ref(&self) -> &ChatActionStartPlayingGame {
        self
    }
}

impl AsRef<ChatActionStartPlayingGame> for RTDChatActionStartPlayingGameBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionTypingBuilder {
        let mut inner = ChatActionTyping::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionTypingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDChatActionTypingBuilder {
    inner: ChatActionTyping,
}

impl RTDChatActionTypingBuilder {
    pub fn build(&self) -> ChatActionTyping {
        self.inner.clone()
    }
}

impl AsRef<ChatActionTyping> for ChatActionTyping {
    fn as_ref(&self) -> &ChatActionTyping {
        self
    }
}

impl AsRef<ChatActionTyping> for RTDChatActionTypingBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionUploadingDocumentBuilder {
        let mut inner = ChatActionUploadingDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionUploadingDocumentBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct RTDChatActionUploadingDocumentBuilder {
    inner: ChatActionUploadingDocument,
}

impl RTDChatActionUploadingDocumentBuilder {
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

impl AsRef<ChatActionUploadingDocument> for RTDChatActionUploadingDocumentBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionUploadingPhotoBuilder {
        let mut inner = ChatActionUploadingPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionUploadingPhotoBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct RTDChatActionUploadingPhotoBuilder {
    inner: ChatActionUploadingPhoto,
}

impl RTDChatActionUploadingPhotoBuilder {
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

impl AsRef<ChatActionUploadingPhoto> for RTDChatActionUploadingPhotoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionUploadingVideoBuilder {
        let mut inner = ChatActionUploadingVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionUploadingVideoBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct RTDChatActionUploadingVideoBuilder {
    inner: ChatActionUploadingVideo,
}

impl RTDChatActionUploadingVideoBuilder {
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

impl AsRef<ChatActionUploadingVideo> for RTDChatActionUploadingVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionUploadingVideoNoteBuilder {
        let mut inner = ChatActionUploadingVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionUploadingVideoNoteBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct RTDChatActionUploadingVideoNoteBuilder {
    inner: ChatActionUploadingVideoNote,
}

impl RTDChatActionUploadingVideoNoteBuilder {
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

impl AsRef<ChatActionUploadingVideoNote> for RTDChatActionUploadingVideoNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDChatActionUploadingVoiceNoteBuilder {
        let mut inner = ChatActionUploadingVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDChatActionUploadingVoiceNoteBuilder { inner }
    }

    pub fn progress(&self) -> i32 {
        self.progress
    }
}

#[doc(hidden)]
pub struct RTDChatActionUploadingVoiceNoteBuilder {
    inner: ChatActionUploadingVoiceNote,
}

impl RTDChatActionUploadingVoiceNoteBuilder {
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

impl AsRef<ChatActionUploadingVoiceNote> for RTDChatActionUploadingVoiceNoteBuilder {
    fn as_ref(&self) -> &ChatActionUploadingVoiceNote {
        &self.inner
    }
}
