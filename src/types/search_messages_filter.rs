use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a filter for message search results
pub trait TDSearchMessagesFilter: Debug + RObject {}

/// Represents a filter for message search results
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SearchMessagesFilter {
    #[doc(hidden)]
    _Default,
    /// Returns only animation messages
    #[serde(rename(
        serialize = "searchMessagesFilterAnimation",
        deserialize = "searchMessagesFilterAnimation"
    ))]
    Animation(SearchMessagesFilterAnimation),
    /// Returns only audio messages
    #[serde(rename(
        serialize = "searchMessagesFilterAudio",
        deserialize = "searchMessagesFilterAudio"
    ))]
    Audio(SearchMessagesFilterAudio),
    /// Returns only call messages
    #[serde(rename(
        serialize = "searchMessagesFilterCall",
        deserialize = "searchMessagesFilterCall"
    ))]
    Call(SearchMessagesFilterCall),
    /// Returns only messages containing chat photos
    #[serde(rename(
        serialize = "searchMessagesFilterChatPhoto",
        deserialize = "searchMessagesFilterChatPhoto"
    ))]
    ChatPhoto(SearchMessagesFilterChatPhoto),
    /// Returns only document messages
    #[serde(rename(
        serialize = "searchMessagesFilterDocument",
        deserialize = "searchMessagesFilterDocument"
    ))]
    Document(SearchMessagesFilterDocument),
    /// Returns all found messages, no filter is applied
    #[serde(rename(
        serialize = "searchMessagesFilterEmpty",
        deserialize = "searchMessagesFilterEmpty"
    ))]
    Empty(SearchMessagesFilterEmpty),
    /// Returns only failed to send messages. This filter can be used only if the message database is used
    #[serde(rename(
        serialize = "searchMessagesFilterFailedToSend",
        deserialize = "searchMessagesFilterFailedToSend"
    ))]
    FailedToSend(SearchMessagesFilterFailedToSend),
    /// Returns only messages with mentions of the current user, or messages that are replies to their messages
    #[serde(rename(
        serialize = "searchMessagesFilterMention",
        deserialize = "searchMessagesFilterMention"
    ))]
    Mention(SearchMessagesFilterMention),
    /// Returns only incoming call messages with missed/declined discard reasons
    #[serde(rename(
        serialize = "searchMessagesFilterMissedCall",
        deserialize = "searchMessagesFilterMissedCall"
    ))]
    MissedCall(SearchMessagesFilterMissedCall),
    /// Returns only photo messages
    #[serde(rename(
        serialize = "searchMessagesFilterPhoto",
        deserialize = "searchMessagesFilterPhoto"
    ))]
    Photo(SearchMessagesFilterPhoto),
    /// Returns only photo and video messages
    #[serde(rename(
        serialize = "searchMessagesFilterPhotoAndVideo",
        deserialize = "searchMessagesFilterPhotoAndVideo"
    ))]
    PhotoAndVideo(SearchMessagesFilterPhotoAndVideo),
    /// Returns only pinned messages
    #[serde(rename(
        serialize = "searchMessagesFilterPinned",
        deserialize = "searchMessagesFilterPinned"
    ))]
    Pinned(SearchMessagesFilterPinned),
    /// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query, a message thread or by the sending user
    #[serde(rename(
        serialize = "searchMessagesFilterUnreadMention",
        deserialize = "searchMessagesFilterUnreadMention"
    ))]
    UnreadMention(SearchMessagesFilterUnreadMention),
    /// Returns only messages containing URLs
    #[serde(rename(
        serialize = "searchMessagesFilterUrl",
        deserialize = "searchMessagesFilterUrl"
    ))]
    Url(SearchMessagesFilterUrl),
    /// Returns only video messages
    #[serde(rename(
        serialize = "searchMessagesFilterVideo",
        deserialize = "searchMessagesFilterVideo"
    ))]
    Video(SearchMessagesFilterVideo),
    /// Returns only video note messages
    #[serde(rename(
        serialize = "searchMessagesFilterVideoNote",
        deserialize = "searchMessagesFilterVideoNote"
    ))]
    VideoNote(SearchMessagesFilterVideoNote),
    /// Returns only voice and video note messages
    #[serde(rename(
        serialize = "searchMessagesFilterVoiceAndVideoNote",
        deserialize = "searchMessagesFilterVoiceAndVideoNote"
    ))]
    VoiceAndVideoNote(SearchMessagesFilterVoiceAndVideoNote),
    /// Returns only voice note messages
    #[serde(rename(
        serialize = "searchMessagesFilterVoiceNote",
        deserialize = "searchMessagesFilterVoiceNote"
    ))]
    VoiceNote(SearchMessagesFilterVoiceNote),
}

impl Default for SearchMessagesFilter {
    fn default() -> Self {
        SearchMessagesFilter::_Default
    }
}

impl RObject for SearchMessagesFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            SearchMessagesFilter::Animation(t) => t.extra(),
            SearchMessagesFilter::Audio(t) => t.extra(),
            SearchMessagesFilter::Call(t) => t.extra(),
            SearchMessagesFilter::ChatPhoto(t) => t.extra(),
            SearchMessagesFilter::Document(t) => t.extra(),
            SearchMessagesFilter::Empty(t) => t.extra(),
            SearchMessagesFilter::FailedToSend(t) => t.extra(),
            SearchMessagesFilter::Mention(t) => t.extra(),
            SearchMessagesFilter::MissedCall(t) => t.extra(),
            SearchMessagesFilter::Photo(t) => t.extra(),
            SearchMessagesFilter::PhotoAndVideo(t) => t.extra(),
            SearchMessagesFilter::Pinned(t) => t.extra(),
            SearchMessagesFilter::UnreadMention(t) => t.extra(),
            SearchMessagesFilter::Url(t) => t.extra(),
            SearchMessagesFilter::Video(t) => t.extra(),
            SearchMessagesFilter::VideoNote(t) => t.extra(),
            SearchMessagesFilter::VoiceAndVideoNote(t) => t.extra(),
            SearchMessagesFilter::VoiceNote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            SearchMessagesFilter::Animation(t) => t.client_id(),
            SearchMessagesFilter::Audio(t) => t.client_id(),
            SearchMessagesFilter::Call(t) => t.client_id(),
            SearchMessagesFilter::ChatPhoto(t) => t.client_id(),
            SearchMessagesFilter::Document(t) => t.client_id(),
            SearchMessagesFilter::Empty(t) => t.client_id(),
            SearchMessagesFilter::FailedToSend(t) => t.client_id(),
            SearchMessagesFilter::Mention(t) => t.client_id(),
            SearchMessagesFilter::MissedCall(t) => t.client_id(),
            SearchMessagesFilter::Photo(t) => t.client_id(),
            SearchMessagesFilter::PhotoAndVideo(t) => t.client_id(),
            SearchMessagesFilter::Pinned(t) => t.client_id(),
            SearchMessagesFilter::UnreadMention(t) => t.client_id(),
            SearchMessagesFilter::Url(t) => t.client_id(),
            SearchMessagesFilter::Video(t) => t.client_id(),
            SearchMessagesFilter::VideoNote(t) => t.client_id(),
            SearchMessagesFilter::VoiceAndVideoNote(t) => t.client_id(),
            SearchMessagesFilter::VoiceNote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl SearchMessagesFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SearchMessagesFilter::_Default)
    }
}

impl AsRef<SearchMessagesFilter> for SearchMessagesFilter {
    fn as_ref(&self) -> &SearchMessagesFilter {
        self
    }
}

/// Returns only animation messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterAnimation {}

impl SearchMessagesFilterAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterAnimationBuilder {
        let mut inner = SearchMessagesFilterAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterAnimationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterAnimationBuilder {
    inner: SearchMessagesFilterAnimation,
}

impl RTDSearchMessagesFilterAnimationBuilder {
    pub fn build(&self) -> SearchMessagesFilterAnimation {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterAnimation> for SearchMessagesFilterAnimation {
    fn as_ref(&self) -> &SearchMessagesFilterAnimation {
        self
    }
}

impl AsRef<SearchMessagesFilterAnimation> for RTDSearchMessagesFilterAnimationBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterAnimation {
        &self.inner
    }
}

/// Returns only audio messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterAudio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterAudio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterAudio {}

impl SearchMessagesFilterAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterAudioBuilder {
        let mut inner = SearchMessagesFilterAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterAudioBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterAudioBuilder {
    inner: SearchMessagesFilterAudio,
}

impl RTDSearchMessagesFilterAudioBuilder {
    pub fn build(&self) -> SearchMessagesFilterAudio {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterAudio> for SearchMessagesFilterAudio {
    fn as_ref(&self) -> &SearchMessagesFilterAudio {
        self
    }
}

impl AsRef<SearchMessagesFilterAudio> for RTDSearchMessagesFilterAudioBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterAudio {
        &self.inner
    }
}

/// Returns only call messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterCall {}

impl SearchMessagesFilterCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterCallBuilder {
        let mut inner = SearchMessagesFilterCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterCallBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterCallBuilder {
    inner: SearchMessagesFilterCall,
}

impl RTDSearchMessagesFilterCallBuilder {
    pub fn build(&self) -> SearchMessagesFilterCall {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterCall> for SearchMessagesFilterCall {
    fn as_ref(&self) -> &SearchMessagesFilterCall {
        self
    }
}

impl AsRef<SearchMessagesFilterCall> for RTDSearchMessagesFilterCallBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterCall {
        &self.inner
    }
}

/// Returns only messages containing chat photos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterChatPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterChatPhoto {}

impl SearchMessagesFilterChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterChatPhotoBuilder {
        let mut inner = SearchMessagesFilterChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterChatPhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterChatPhotoBuilder {
    inner: SearchMessagesFilterChatPhoto,
}

impl RTDSearchMessagesFilterChatPhotoBuilder {
    pub fn build(&self) -> SearchMessagesFilterChatPhoto {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterChatPhoto> for SearchMessagesFilterChatPhoto {
    fn as_ref(&self) -> &SearchMessagesFilterChatPhoto {
        self
    }
}

impl AsRef<SearchMessagesFilterChatPhoto> for RTDSearchMessagesFilterChatPhotoBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterChatPhoto {
        &self.inner
    }
}

/// Returns only document messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterDocument {}

impl SearchMessagesFilterDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterDocumentBuilder {
        let mut inner = SearchMessagesFilterDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterDocumentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterDocumentBuilder {
    inner: SearchMessagesFilterDocument,
}

impl RTDSearchMessagesFilterDocumentBuilder {
    pub fn build(&self) -> SearchMessagesFilterDocument {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterDocument> for SearchMessagesFilterDocument {
    fn as_ref(&self) -> &SearchMessagesFilterDocument {
        self
    }
}

impl AsRef<SearchMessagesFilterDocument> for RTDSearchMessagesFilterDocumentBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterDocument {
        &self.inner
    }
}

/// Returns all found messages, no filter is applied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterEmpty {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterEmpty {}

impl SearchMessagesFilterEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterEmptyBuilder {
        let mut inner = SearchMessagesFilterEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterEmptyBuilder {
    inner: SearchMessagesFilterEmpty,
}

impl RTDSearchMessagesFilterEmptyBuilder {
    pub fn build(&self) -> SearchMessagesFilterEmpty {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterEmpty> for SearchMessagesFilterEmpty {
    fn as_ref(&self) -> &SearchMessagesFilterEmpty {
        self
    }
}

impl AsRef<SearchMessagesFilterEmpty> for RTDSearchMessagesFilterEmptyBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterEmpty {
        &self.inner
    }
}

/// Returns only failed to send messages. This filter can be used only if the message database is used
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterFailedToSend {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterFailedToSend {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterFailedToSend {}

impl SearchMessagesFilterFailedToSend {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterFailedToSendBuilder {
        let mut inner = SearchMessagesFilterFailedToSend::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterFailedToSendBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterFailedToSendBuilder {
    inner: SearchMessagesFilterFailedToSend,
}

impl RTDSearchMessagesFilterFailedToSendBuilder {
    pub fn build(&self) -> SearchMessagesFilterFailedToSend {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterFailedToSend> for SearchMessagesFilterFailedToSend {
    fn as_ref(&self) -> &SearchMessagesFilterFailedToSend {
        self
    }
}

impl AsRef<SearchMessagesFilterFailedToSend> for RTDSearchMessagesFilterFailedToSendBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterFailedToSend {
        &self.inner
    }
}

/// Returns only messages with mentions of the current user, or messages that are replies to their messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterMention {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterMention {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterMention {}

impl SearchMessagesFilterMention {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterMentionBuilder {
        let mut inner = SearchMessagesFilterMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterMentionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterMentionBuilder {
    inner: SearchMessagesFilterMention,
}

impl RTDSearchMessagesFilterMentionBuilder {
    pub fn build(&self) -> SearchMessagesFilterMention {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterMention> for SearchMessagesFilterMention {
    fn as_ref(&self) -> &SearchMessagesFilterMention {
        self
    }
}

impl AsRef<SearchMessagesFilterMention> for RTDSearchMessagesFilterMentionBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterMention {
        &self.inner
    }
}

/// Returns only incoming call messages with missed/declined discard reasons
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterMissedCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterMissedCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterMissedCall {}

impl SearchMessagesFilterMissedCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterMissedCallBuilder {
        let mut inner = SearchMessagesFilterMissedCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterMissedCallBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterMissedCallBuilder {
    inner: SearchMessagesFilterMissedCall,
}

impl RTDSearchMessagesFilterMissedCallBuilder {
    pub fn build(&self) -> SearchMessagesFilterMissedCall {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterMissedCall> for SearchMessagesFilterMissedCall {
    fn as_ref(&self) -> &SearchMessagesFilterMissedCall {
        self
    }
}

impl AsRef<SearchMessagesFilterMissedCall> for RTDSearchMessagesFilterMissedCallBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterMissedCall {
        &self.inner
    }
}

/// Returns only photo messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterPhoto {}

impl SearchMessagesFilterPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterPhotoBuilder {
        let mut inner = SearchMessagesFilterPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterPhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterPhotoBuilder {
    inner: SearchMessagesFilterPhoto,
}

impl RTDSearchMessagesFilterPhotoBuilder {
    pub fn build(&self) -> SearchMessagesFilterPhoto {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterPhoto> for SearchMessagesFilterPhoto {
    fn as_ref(&self) -> &SearchMessagesFilterPhoto {
        self
    }
}

impl AsRef<SearchMessagesFilterPhoto> for RTDSearchMessagesFilterPhotoBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterPhoto {
        &self.inner
    }
}

/// Returns only photo and video messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterPhotoAndVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterPhotoAndVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterPhotoAndVideo {}

impl SearchMessagesFilterPhotoAndVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterPhotoAndVideoBuilder {
        let mut inner = SearchMessagesFilterPhotoAndVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterPhotoAndVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterPhotoAndVideoBuilder {
    inner: SearchMessagesFilterPhotoAndVideo,
}

impl RTDSearchMessagesFilterPhotoAndVideoBuilder {
    pub fn build(&self) -> SearchMessagesFilterPhotoAndVideo {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterPhotoAndVideo> for SearchMessagesFilterPhotoAndVideo {
    fn as_ref(&self) -> &SearchMessagesFilterPhotoAndVideo {
        self
    }
}

impl AsRef<SearchMessagesFilterPhotoAndVideo> for RTDSearchMessagesFilterPhotoAndVideoBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterPhotoAndVideo {
        &self.inner
    }
}

/// Returns only pinned messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterPinned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterPinned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterPinned {}

impl SearchMessagesFilterPinned {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterPinnedBuilder {
        let mut inner = SearchMessagesFilterPinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterPinnedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterPinnedBuilder {
    inner: SearchMessagesFilterPinned,
}

impl RTDSearchMessagesFilterPinnedBuilder {
    pub fn build(&self) -> SearchMessagesFilterPinned {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterPinned> for SearchMessagesFilterPinned {
    fn as_ref(&self) -> &SearchMessagesFilterPinned {
        self
    }
}

impl AsRef<SearchMessagesFilterPinned> for RTDSearchMessagesFilterPinnedBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterPinned {
        &self.inner
    }
}

/// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query, a message thread or by the sending user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterUnreadMention {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterUnreadMention {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterUnreadMention {}

impl SearchMessagesFilterUnreadMention {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterUnreadMentionBuilder {
        let mut inner = SearchMessagesFilterUnreadMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterUnreadMentionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterUnreadMentionBuilder {
    inner: SearchMessagesFilterUnreadMention,
}

impl RTDSearchMessagesFilterUnreadMentionBuilder {
    pub fn build(&self) -> SearchMessagesFilterUnreadMention {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterUnreadMention> for SearchMessagesFilterUnreadMention {
    fn as_ref(&self) -> &SearchMessagesFilterUnreadMention {
        self
    }
}

impl AsRef<SearchMessagesFilterUnreadMention> for RTDSearchMessagesFilterUnreadMentionBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterUnreadMention {
        &self.inner
    }
}

/// Returns only messages containing URLs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterUrl {}

impl SearchMessagesFilterUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterUrlBuilder {
        let mut inner = SearchMessagesFilterUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterUrlBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterUrlBuilder {
    inner: SearchMessagesFilterUrl,
}

impl RTDSearchMessagesFilterUrlBuilder {
    pub fn build(&self) -> SearchMessagesFilterUrl {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterUrl> for SearchMessagesFilterUrl {
    fn as_ref(&self) -> &SearchMessagesFilterUrl {
        self
    }
}

impl AsRef<SearchMessagesFilterUrl> for RTDSearchMessagesFilterUrlBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterUrl {
        &self.inner
    }
}

/// Returns only video messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterVideo {}

impl SearchMessagesFilterVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterVideoBuilder {
        let mut inner = SearchMessagesFilterVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVideoBuilder {
    inner: SearchMessagesFilterVideo,
}

impl RTDSearchMessagesFilterVideoBuilder {
    pub fn build(&self) -> SearchMessagesFilterVideo {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVideo> for SearchMessagesFilterVideo {
    fn as_ref(&self) -> &SearchMessagesFilterVideo {
        self
    }
}

impl AsRef<SearchMessagesFilterVideo> for RTDSearchMessagesFilterVideoBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterVideo {
        &self.inner
    }
}

/// Returns only video note messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterVideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterVideoNote {}

impl SearchMessagesFilterVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterVideoNoteBuilder {
        let mut inner = SearchMessagesFilterVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVideoNoteBuilder {
    inner: SearchMessagesFilterVideoNote,
}

impl RTDSearchMessagesFilterVideoNoteBuilder {
    pub fn build(&self) -> SearchMessagesFilterVideoNote {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVideoNote> for SearchMessagesFilterVideoNote {
    fn as_ref(&self) -> &SearchMessagesFilterVideoNote {
        self
    }
}

impl AsRef<SearchMessagesFilterVideoNote> for RTDSearchMessagesFilterVideoNoteBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterVideoNote {
        &self.inner
    }
}

/// Returns only voice and video note messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVoiceAndVideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterVoiceAndVideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterVoiceAndVideoNote {}

impl SearchMessagesFilterVoiceAndVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterVoiceAndVideoNoteBuilder {
        let mut inner = SearchMessagesFilterVoiceAndVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterVoiceAndVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVoiceAndVideoNoteBuilder {
    inner: SearchMessagesFilterVoiceAndVideoNote,
}

impl RTDSearchMessagesFilterVoiceAndVideoNoteBuilder {
    pub fn build(&self) -> SearchMessagesFilterVoiceAndVideoNote {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVoiceAndVideoNote> for SearchMessagesFilterVoiceAndVideoNote {
    fn as_ref(&self) -> &SearchMessagesFilterVoiceAndVideoNote {
        self
    }
}

impl AsRef<SearchMessagesFilterVoiceAndVideoNote>
    for RTDSearchMessagesFilterVoiceAndVideoNoteBuilder
{
    fn as_ref(&self) -> &SearchMessagesFilterVoiceAndVideoNote {
        &self.inner
    }
}

/// Returns only voice note messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchMessagesFilterVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SearchMessagesFilterVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSearchMessagesFilter for SearchMessagesFilterVoiceNote {}

impl SearchMessagesFilterVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSearchMessagesFilterVoiceNoteBuilder {
        let mut inner = SearchMessagesFilterVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSearchMessagesFilterVoiceNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSearchMessagesFilterVoiceNoteBuilder {
    inner: SearchMessagesFilterVoiceNote,
}

impl RTDSearchMessagesFilterVoiceNoteBuilder {
    pub fn build(&self) -> SearchMessagesFilterVoiceNote {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVoiceNote> for SearchMessagesFilterVoiceNote {
    fn as_ref(&self) -> &SearchMessagesFilterVoiceNote {
        self
    }
}

impl AsRef<SearchMessagesFilterVoiceNote> for RTDSearchMessagesFilterVoiceNoteBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterVoiceNote {
        &self.inner
    }
}
