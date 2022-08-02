use crate::errors::Result;
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
    #[serde(rename = "searchMessagesFilterAnimation")]
    Animation(SearchMessagesFilterAnimation),
    /// Returns only audio messages
    #[serde(rename = "searchMessagesFilterAudio")]
    Audio(SearchMessagesFilterAudio),
    /// Returns only messages containing chat photos
    #[serde(rename = "searchMessagesFilterChatPhoto")]
    ChatPhoto(SearchMessagesFilterChatPhoto),
    /// Returns only document messages
    #[serde(rename = "searchMessagesFilterDocument")]
    Document(SearchMessagesFilterDocument),
    /// Returns all found messages, no filter is applied
    #[serde(rename = "searchMessagesFilterEmpty")]
    Empty(SearchMessagesFilterEmpty),
    /// Returns only failed to send messages. This filter can be used only if the message database is used
    #[serde(rename = "searchMessagesFilterFailedToSend")]
    FailedToSend(SearchMessagesFilterFailedToSend),
    /// Returns only messages with mentions of the current user, or messages that are replies to their messages
    #[serde(rename = "searchMessagesFilterMention")]
    Mention(SearchMessagesFilterMention),
    /// Returns only photo messages
    #[serde(rename = "searchMessagesFilterPhoto")]
    Photo(SearchMessagesFilterPhoto),
    /// Returns only photo and video messages
    #[serde(rename = "searchMessagesFilterPhotoAndVideo")]
    PhotoAndVideo(SearchMessagesFilterPhotoAndVideo),
    /// Returns only pinned messages
    #[serde(rename = "searchMessagesFilterPinned")]
    Pinned(SearchMessagesFilterPinned),
    /// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query, a message thread or by the sending user
    #[serde(rename = "searchMessagesFilterUnreadMention")]
    UnreadMention(SearchMessagesFilterUnreadMention),
    /// Returns only messages containing URLs
    #[serde(rename = "searchMessagesFilterUrl")]
    Url(SearchMessagesFilterUrl),
    /// Returns only video messages
    #[serde(rename = "searchMessagesFilterVideo")]
    Video(SearchMessagesFilterVideo),
    /// Returns only video note messages
    #[serde(rename = "searchMessagesFilterVideoNote")]
    VideoNote(SearchMessagesFilterVideoNote),
    /// Returns only voice and video note messages
    #[serde(rename = "searchMessagesFilterVoiceAndVideoNote")]
    VoiceAndVideoNote(SearchMessagesFilterVoiceAndVideoNote),
    /// Returns only voice note messages
    #[serde(rename = "searchMessagesFilterVoiceNote")]
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
            SearchMessagesFilter::ChatPhoto(t) => t.extra(),
            SearchMessagesFilter::Document(t) => t.extra(),
            SearchMessagesFilter::Empty(t) => t.extra(),
            SearchMessagesFilter::FailedToSend(t) => t.extra(),
            SearchMessagesFilter::Mention(t) => t.extra(),
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
            SearchMessagesFilter::ChatPhoto(t) => t.client_id(),
            SearchMessagesFilter::Document(t) => t.client_id(),
            SearchMessagesFilter::Empty(t) => t.client_id(),
            SearchMessagesFilter::FailedToSend(t) => t.client_id(),
            SearchMessagesFilter::Mention(t) => t.client_id(),
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterAnimationBuilder {
        let mut inner = SearchMessagesFilterAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterAnimationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterAnimationBuilder {
    inner: SearchMessagesFilterAnimation,
}

#[deprecated]
pub type RTDSearchMessagesFilterAnimationBuilder = SearchMessagesFilterAnimationBuilder;

impl SearchMessagesFilterAnimationBuilder {
    pub fn build(&self) -> SearchMessagesFilterAnimation {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterAnimation> for SearchMessagesFilterAnimation {
    fn as_ref(&self) -> &SearchMessagesFilterAnimation {
        self
    }
}

impl AsRef<SearchMessagesFilterAnimation> for SearchMessagesFilterAnimationBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterAudioBuilder {
        let mut inner = SearchMessagesFilterAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterAudioBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterAudioBuilder {
    inner: SearchMessagesFilterAudio,
}

#[deprecated]
pub type RTDSearchMessagesFilterAudioBuilder = SearchMessagesFilterAudioBuilder;

impl SearchMessagesFilterAudioBuilder {
    pub fn build(&self) -> SearchMessagesFilterAudio {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterAudio> for SearchMessagesFilterAudio {
    fn as_ref(&self) -> &SearchMessagesFilterAudio {
        self
    }
}

impl AsRef<SearchMessagesFilterAudio> for SearchMessagesFilterAudioBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterAudio {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterChatPhotoBuilder {
        let mut inner = SearchMessagesFilterChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterChatPhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterChatPhotoBuilder {
    inner: SearchMessagesFilterChatPhoto,
}

#[deprecated]
pub type RTDSearchMessagesFilterChatPhotoBuilder = SearchMessagesFilterChatPhotoBuilder;

impl SearchMessagesFilterChatPhotoBuilder {
    pub fn build(&self) -> SearchMessagesFilterChatPhoto {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterChatPhoto> for SearchMessagesFilterChatPhoto {
    fn as_ref(&self) -> &SearchMessagesFilterChatPhoto {
        self
    }
}

impl AsRef<SearchMessagesFilterChatPhoto> for SearchMessagesFilterChatPhotoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterDocumentBuilder {
        let mut inner = SearchMessagesFilterDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterDocumentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterDocumentBuilder {
    inner: SearchMessagesFilterDocument,
}

#[deprecated]
pub type RTDSearchMessagesFilterDocumentBuilder = SearchMessagesFilterDocumentBuilder;

impl SearchMessagesFilterDocumentBuilder {
    pub fn build(&self) -> SearchMessagesFilterDocument {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterDocument> for SearchMessagesFilterDocument {
    fn as_ref(&self) -> &SearchMessagesFilterDocument {
        self
    }
}

impl AsRef<SearchMessagesFilterDocument> for SearchMessagesFilterDocumentBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterEmptyBuilder {
        let mut inner = SearchMessagesFilterEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterEmptyBuilder {
    inner: SearchMessagesFilterEmpty,
}

#[deprecated]
pub type RTDSearchMessagesFilterEmptyBuilder = SearchMessagesFilterEmptyBuilder;

impl SearchMessagesFilterEmptyBuilder {
    pub fn build(&self) -> SearchMessagesFilterEmpty {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterEmpty> for SearchMessagesFilterEmpty {
    fn as_ref(&self) -> &SearchMessagesFilterEmpty {
        self
    }
}

impl AsRef<SearchMessagesFilterEmpty> for SearchMessagesFilterEmptyBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterFailedToSendBuilder {
        let mut inner = SearchMessagesFilterFailedToSend::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterFailedToSendBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterFailedToSendBuilder {
    inner: SearchMessagesFilterFailedToSend,
}

#[deprecated]
pub type RTDSearchMessagesFilterFailedToSendBuilder = SearchMessagesFilterFailedToSendBuilder;

impl SearchMessagesFilterFailedToSendBuilder {
    pub fn build(&self) -> SearchMessagesFilterFailedToSend {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterFailedToSend> for SearchMessagesFilterFailedToSend {
    fn as_ref(&self) -> &SearchMessagesFilterFailedToSend {
        self
    }
}

impl AsRef<SearchMessagesFilterFailedToSend> for SearchMessagesFilterFailedToSendBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterMentionBuilder {
        let mut inner = SearchMessagesFilterMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterMentionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterMentionBuilder {
    inner: SearchMessagesFilterMention,
}

#[deprecated]
pub type RTDSearchMessagesFilterMentionBuilder = SearchMessagesFilterMentionBuilder;

impl SearchMessagesFilterMentionBuilder {
    pub fn build(&self) -> SearchMessagesFilterMention {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterMention> for SearchMessagesFilterMention {
    fn as_ref(&self) -> &SearchMessagesFilterMention {
        self
    }
}

impl AsRef<SearchMessagesFilterMention> for SearchMessagesFilterMentionBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterMention {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterPhotoBuilder {
        let mut inner = SearchMessagesFilterPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterPhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterPhotoBuilder {
    inner: SearchMessagesFilterPhoto,
}

#[deprecated]
pub type RTDSearchMessagesFilterPhotoBuilder = SearchMessagesFilterPhotoBuilder;

impl SearchMessagesFilterPhotoBuilder {
    pub fn build(&self) -> SearchMessagesFilterPhoto {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterPhoto> for SearchMessagesFilterPhoto {
    fn as_ref(&self) -> &SearchMessagesFilterPhoto {
        self
    }
}

impl AsRef<SearchMessagesFilterPhoto> for SearchMessagesFilterPhotoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterPhotoAndVideoBuilder {
        let mut inner = SearchMessagesFilterPhotoAndVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterPhotoAndVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterPhotoAndVideoBuilder {
    inner: SearchMessagesFilterPhotoAndVideo,
}

#[deprecated]
pub type RTDSearchMessagesFilterPhotoAndVideoBuilder = SearchMessagesFilterPhotoAndVideoBuilder;

impl SearchMessagesFilterPhotoAndVideoBuilder {
    pub fn build(&self) -> SearchMessagesFilterPhotoAndVideo {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterPhotoAndVideo> for SearchMessagesFilterPhotoAndVideo {
    fn as_ref(&self) -> &SearchMessagesFilterPhotoAndVideo {
        self
    }
}

impl AsRef<SearchMessagesFilterPhotoAndVideo> for SearchMessagesFilterPhotoAndVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterPinnedBuilder {
        let mut inner = SearchMessagesFilterPinned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterPinnedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterPinnedBuilder {
    inner: SearchMessagesFilterPinned,
}

#[deprecated]
pub type RTDSearchMessagesFilterPinnedBuilder = SearchMessagesFilterPinnedBuilder;

impl SearchMessagesFilterPinnedBuilder {
    pub fn build(&self) -> SearchMessagesFilterPinned {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterPinned> for SearchMessagesFilterPinned {
    fn as_ref(&self) -> &SearchMessagesFilterPinned {
        self
    }
}

impl AsRef<SearchMessagesFilterPinned> for SearchMessagesFilterPinnedBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterUnreadMentionBuilder {
        let mut inner = SearchMessagesFilterUnreadMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterUnreadMentionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterUnreadMentionBuilder {
    inner: SearchMessagesFilterUnreadMention,
}

#[deprecated]
pub type RTDSearchMessagesFilterUnreadMentionBuilder = SearchMessagesFilterUnreadMentionBuilder;

impl SearchMessagesFilterUnreadMentionBuilder {
    pub fn build(&self) -> SearchMessagesFilterUnreadMention {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterUnreadMention> for SearchMessagesFilterUnreadMention {
    fn as_ref(&self) -> &SearchMessagesFilterUnreadMention {
        self
    }
}

impl AsRef<SearchMessagesFilterUnreadMention> for SearchMessagesFilterUnreadMentionBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterUrlBuilder {
        let mut inner = SearchMessagesFilterUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterUrlBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterUrlBuilder {
    inner: SearchMessagesFilterUrl,
}

#[deprecated]
pub type RTDSearchMessagesFilterUrlBuilder = SearchMessagesFilterUrlBuilder;

impl SearchMessagesFilterUrlBuilder {
    pub fn build(&self) -> SearchMessagesFilterUrl {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterUrl> for SearchMessagesFilterUrl {
    fn as_ref(&self) -> &SearchMessagesFilterUrl {
        self
    }
}

impl AsRef<SearchMessagesFilterUrl> for SearchMessagesFilterUrlBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterVideoBuilder {
        let mut inner = SearchMessagesFilterVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterVideoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterVideoBuilder {
    inner: SearchMessagesFilterVideo,
}

#[deprecated]
pub type RTDSearchMessagesFilterVideoBuilder = SearchMessagesFilterVideoBuilder;

impl SearchMessagesFilterVideoBuilder {
    pub fn build(&self) -> SearchMessagesFilterVideo {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVideo> for SearchMessagesFilterVideo {
    fn as_ref(&self) -> &SearchMessagesFilterVideo {
        self
    }
}

impl AsRef<SearchMessagesFilterVideo> for SearchMessagesFilterVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterVideoNoteBuilder {
        let mut inner = SearchMessagesFilterVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterVideoNoteBuilder {
    inner: SearchMessagesFilterVideoNote,
}

#[deprecated]
pub type RTDSearchMessagesFilterVideoNoteBuilder = SearchMessagesFilterVideoNoteBuilder;

impl SearchMessagesFilterVideoNoteBuilder {
    pub fn build(&self) -> SearchMessagesFilterVideoNote {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVideoNote> for SearchMessagesFilterVideoNote {
    fn as_ref(&self) -> &SearchMessagesFilterVideoNote {
        self
    }
}

impl AsRef<SearchMessagesFilterVideoNote> for SearchMessagesFilterVideoNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterVoiceAndVideoNoteBuilder {
        let mut inner = SearchMessagesFilterVoiceAndVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterVoiceAndVideoNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterVoiceAndVideoNoteBuilder {
    inner: SearchMessagesFilterVoiceAndVideoNote,
}

#[deprecated]
pub type RTDSearchMessagesFilterVoiceAndVideoNoteBuilder =
    SearchMessagesFilterVoiceAndVideoNoteBuilder;

impl SearchMessagesFilterVoiceAndVideoNoteBuilder {
    pub fn build(&self) -> SearchMessagesFilterVoiceAndVideoNote {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVoiceAndVideoNote> for SearchMessagesFilterVoiceAndVideoNote {
    fn as_ref(&self) -> &SearchMessagesFilterVoiceAndVideoNote {
        self
    }
}

impl AsRef<SearchMessagesFilterVoiceAndVideoNote> for SearchMessagesFilterVoiceAndVideoNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SearchMessagesFilterVoiceNoteBuilder {
        let mut inner = SearchMessagesFilterVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SearchMessagesFilterVoiceNoteBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SearchMessagesFilterVoiceNoteBuilder {
    inner: SearchMessagesFilterVoiceNote,
}

#[deprecated]
pub type RTDSearchMessagesFilterVoiceNoteBuilder = SearchMessagesFilterVoiceNoteBuilder;

impl SearchMessagesFilterVoiceNoteBuilder {
    pub fn build(&self) -> SearchMessagesFilterVoiceNote {
        self.inner.clone()
    }
}

impl AsRef<SearchMessagesFilterVoiceNote> for SearchMessagesFilterVoiceNote {
    fn as_ref(&self) -> &SearchMessagesFilterVoiceNote {
        self
    }
}

impl AsRef<SearchMessagesFilterVoiceNote> for SearchMessagesFilterVoiceNoteBuilder {
    fn as_ref(&self) -> &SearchMessagesFilterVoiceNote {
        &self.inner
    }
}
