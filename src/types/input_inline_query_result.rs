use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a single result of an inline query; for bots only
pub trait TDInputInlineQueryResult: Debug + RObject {}

/// Represents a single result of an inline query; for bots only
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputInlineQueryResult {
    #[doc(hidden)]
    _Default(()),
    /// Represents a link to an animated GIF or an animated (i.e. without sound) H.264/MPEG-4 AVC video
    Animation(InputInlineQueryResultAnimation),
    /// Represents a link to an article or web page
    Article(InputInlineQueryResultArticle),
    /// Represents a link to an MP3 audio file
    Audio(InputInlineQueryResultAudio),
    /// Represents a user contact
    Contact(InputInlineQueryResultContact),
    /// Represents a link to a file
    Document(InputInlineQueryResultDocument),
    /// Represents a game
    Game(InputInlineQueryResultGame),
    /// Represents a point on the map
    Location(InputInlineQueryResultLocation),
    /// Represents link to a JPEG image
    Photo(InputInlineQueryResultPhoto),
    /// Represents a link to a WEBP or TGS sticker
    Sticker(InputInlineQueryResultSticker),
    /// Represents information about a venue
    Venue(InputInlineQueryResultVenue),
    /// Represents a link to a page containing an embedded video player or a video file
    Video(InputInlineQueryResultVideo),
    /// Represents a link to an opus-encoded audio file within an OGG container, single channel audio
    VoiceNote(InputInlineQueryResultVoiceNote),
}

impl Default for InputInlineQueryResult {
    fn default() -> Self {
        InputInlineQueryResult::_Default(())
    }
}

impl<'de> Deserialize<'de> for InputInlineQueryResult {
    fn deserialize<D>(deserializer: D) -> Result<InputInlineQueryResult, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          InputInlineQueryResult,
          (inputInlineQueryResultAnimation, Animation);
          (inputInlineQueryResultArticle, Article);
          (inputInlineQueryResultAudio, Audio);
          (inputInlineQueryResultContact, Contact);
          (inputInlineQueryResultDocument, Document);
          (inputInlineQueryResultGame, Game);
          (inputInlineQueryResultLocation, Location);
          (inputInlineQueryResultPhoto, Photo);
          (inputInlineQueryResultSticker, Sticker);
          (inputInlineQueryResultVenue, Venue);
          (inputInlineQueryResultVideo, Video);
          (inputInlineQueryResultVoiceNote, VoiceNote);

        )(deserializer)
    }
}

impl RObject for InputInlineQueryResult {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            InputInlineQueryResult::Animation(t) => t.td_name(),
            InputInlineQueryResult::Article(t) => t.td_name(),
            InputInlineQueryResult::Audio(t) => t.td_name(),
            InputInlineQueryResult::Contact(t) => t.td_name(),
            InputInlineQueryResult::Document(t) => t.td_name(),
            InputInlineQueryResult::Game(t) => t.td_name(),
            InputInlineQueryResult::Location(t) => t.td_name(),
            InputInlineQueryResult::Photo(t) => t.td_name(),
            InputInlineQueryResult::Sticker(t) => t.td_name(),
            InputInlineQueryResult::Venue(t) => t.td_name(),
            InputInlineQueryResult::Video(t) => t.td_name(),
            InputInlineQueryResult::VoiceNote(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            InputInlineQueryResult::Animation(t) => t.extra(),
            InputInlineQueryResult::Article(t) => t.extra(),
            InputInlineQueryResult::Audio(t) => t.extra(),
            InputInlineQueryResult::Contact(t) => t.extra(),
            InputInlineQueryResult::Document(t) => t.extra(),
            InputInlineQueryResult::Game(t) => t.extra(),
            InputInlineQueryResult::Location(t) => t.extra(),
            InputInlineQueryResult::Photo(t) => t.extra(),
            InputInlineQueryResult::Sticker(t) => t.extra(),
            InputInlineQueryResult::Venue(t) => t.extra(),
            InputInlineQueryResult::Video(t) => t.extra(),
            InputInlineQueryResult::VoiceNote(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl InputInlineQueryResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputInlineQueryResult::_Default(_))
    }
}

impl AsRef<InputInlineQueryResult> for InputInlineQueryResult {
    fn as_ref(&self) -> &InputInlineQueryResult {
        self
    }
}

/// Represents a link to an animated GIF or an animated (i.e. without sound) H.264/MPEG-4 AVC video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Title of the query result
    title: String,
    /// URL of the result thumbnail (JPEG, GIF, or MPEG4), if it exists
    thumbnail_url: String,
    /// MIME type of the video thumbnail. If non-empty, must be one of "image/jpeg", "image/gif" and "video/mp4"
    thumbnail_mime_type: String,
    /// The URL of the video file (file size must not exceed 1MB)
    video_url: String,
    /// MIME type of the video file. Must be one of "image/gif" and "video/mp4"
    video_mime_type: String,
    /// Duration of the video, in seconds
    video_duration: i32,
    /// Width of the video
    video_width: i32,
    /// Height of the video
    video_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAnimation, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultAnimation {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultAnimation"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultAnimation {}

impl InputInlineQueryResultAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultAnimationBuilder {
        let mut inner = InputInlineQueryResultAnimation::default();
        inner.td_name = "inputInlineQueryResultAnimation".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultAnimationBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn thumbnail_mime_type(&self) -> &String {
        &self.thumbnail_mime_type
    }

    pub fn video_url(&self) -> &String {
        &self.video_url
    }

    pub fn video_mime_type(&self) -> &String {
        &self.video_mime_type
    }

    pub fn video_duration(&self) -> i32 {
        self.video_duration
    }

    pub fn video_width(&self) -> i32 {
        self.video_width
    }

    pub fn video_height(&self) -> i32 {
        self.video_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultAnimationBuilder {
    inner: InputInlineQueryResultAnimation,
}

impl RTDInputInlineQueryResultAnimationBuilder {
    pub fn build(&self) -> InputInlineQueryResultAnimation {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn thumbnail_mime_type<T: AsRef<str>>(&mut self, thumbnail_mime_type: T) -> &mut Self {
        self.inner.thumbnail_mime_type = thumbnail_mime_type.as_ref().to_string();
        self
    }

    pub fn video_url<T: AsRef<str>>(&mut self, video_url: T) -> &mut Self {
        self.inner.video_url = video_url.as_ref().to_string();
        self
    }

    pub fn video_mime_type<T: AsRef<str>>(&mut self, video_mime_type: T) -> &mut Self {
        self.inner.video_mime_type = video_mime_type.as_ref().to_string();
        self
    }

    pub fn video_duration(&mut self, video_duration: i32) -> &mut Self {
        self.inner.video_duration = video_duration;
        self
    }

    pub fn video_width(&mut self, video_width: i32) -> &mut Self {
        self.inner.video_width = video_width;
        self
    }

    pub fn video_height(&mut self, video_height: i32) -> &mut Self {
        self.inner.video_height = video_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultAnimation> for InputInlineQueryResultAnimation {
    fn as_ref(&self) -> &InputInlineQueryResultAnimation {
        self
    }
}

impl AsRef<InputInlineQueryResultAnimation> for RTDInputInlineQueryResultAnimationBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultAnimation {
        &self.inner
    }
}

/// Represents a link to an article or web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultArticle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// URL of the result, if it exists
    url: String,
    /// True, if the URL must be not shown
    hide_url: bool,
    /// Title of the result
    title: String,
    /// Represents a link to an article or web page
    description: String,
    /// URL of the result thumbnail, if it exists
    thumbnail_url: String,
    /// Thumbnail width, if known
    thumbnail_width: i32,
    /// Thumbnail height, if known
    thumbnail_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultArticle {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultArticle"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultArticle {}

impl InputInlineQueryResultArticle {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultArticleBuilder {
        let mut inner = InputInlineQueryResultArticle::default();
        inner.td_name = "inputInlineQueryResultArticle".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultArticleBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn hide_url(&self) -> bool {
        self.hide_url
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn thumbnail_width(&self) -> i32 {
        self.thumbnail_width
    }

    pub fn thumbnail_height(&self) -> i32 {
        self.thumbnail_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultArticleBuilder {
    inner: InputInlineQueryResultArticle,
}

impl RTDInputInlineQueryResultArticleBuilder {
    pub fn build(&self) -> InputInlineQueryResultArticle {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn hide_url(&mut self, hide_url: bool) -> &mut Self {
        self.inner.hide_url = hide_url;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn thumbnail_width(&mut self, thumbnail_width: i32) -> &mut Self {
        self.inner.thumbnail_width = thumbnail_width;
        self
    }

    pub fn thumbnail_height(&mut self, thumbnail_height: i32) -> &mut Self {
        self.inner.thumbnail_height = thumbnail_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultArticle> for InputInlineQueryResultArticle {
    fn as_ref(&self) -> &InputInlineQueryResultArticle {
        self
    }
}

impl AsRef<InputInlineQueryResultArticle> for RTDInputInlineQueryResultArticleBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultArticle {
        &self.inner
    }
}

/// Represents a link to an MP3 audio file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultAudio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Title of the audio file
    title: String,
    /// Performer of the audio file
    performer: String,
    /// The URL of the audio file
    audio_url: String,
    /// Audio file duration, in seconds
    audio_duration: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageAudio, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultAudio {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultAudio"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultAudio {}

impl InputInlineQueryResultAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultAudioBuilder {
        let mut inner = InputInlineQueryResultAudio::default();
        inner.td_name = "inputInlineQueryResultAudio".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultAudioBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn performer(&self) -> &String {
        &self.performer
    }

    pub fn audio_url(&self) -> &String {
        &self.audio_url
    }

    pub fn audio_duration(&self) -> i32 {
        self.audio_duration
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultAudioBuilder {
    inner: InputInlineQueryResultAudio,
}

impl RTDInputInlineQueryResultAudioBuilder {
    pub fn build(&self) -> InputInlineQueryResultAudio {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn performer<T: AsRef<str>>(&mut self, performer: T) -> &mut Self {
        self.inner.performer = performer.as_ref().to_string();
        self
    }

    pub fn audio_url<T: AsRef<str>>(&mut self, audio_url: T) -> &mut Self {
        self.inner.audio_url = audio_url.as_ref().to_string();
        self
    }

    pub fn audio_duration(&mut self, audio_duration: i32) -> &mut Self {
        self.inner.audio_duration = audio_duration;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultAudio> for InputInlineQueryResultAudio {
    fn as_ref(&self) -> &InputInlineQueryResultAudio {
        self
    }
}

impl AsRef<InputInlineQueryResultAudio> for RTDInputInlineQueryResultAudioBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultAudio {
        &self.inner
    }
}

/// Represents a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultContact {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// User contact
    contact: Contact,
    /// URL of the result thumbnail, if it exists
    thumbnail_url: String,
    /// Thumbnail width, if known
    thumbnail_width: i32,
    /// Thumbnail height, if known
    thumbnail_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultContact {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultContact"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultContact {}

impl InputInlineQueryResultContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultContactBuilder {
        let mut inner = InputInlineQueryResultContact::default();
        inner.td_name = "inputInlineQueryResultContact".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultContactBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn contact(&self) -> &Contact {
        &self.contact
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn thumbnail_width(&self) -> i32 {
        self.thumbnail_width
    }

    pub fn thumbnail_height(&self) -> i32 {
        self.thumbnail_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultContactBuilder {
    inner: InputInlineQueryResultContact,
}

impl RTDInputInlineQueryResultContactBuilder {
    pub fn build(&self) -> InputInlineQueryResultContact {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
        self.inner.contact = contact.as_ref().clone();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn thumbnail_width(&mut self, thumbnail_width: i32) -> &mut Self {
        self.inner.thumbnail_width = thumbnail_width;
        self
    }

    pub fn thumbnail_height(&mut self, thumbnail_height: i32) -> &mut Self {
        self.inner.thumbnail_height = thumbnail_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultContact> for InputInlineQueryResultContact {
    fn as_ref(&self) -> &InputInlineQueryResultContact {
        self
    }
}

impl AsRef<InputInlineQueryResultContact> for RTDInputInlineQueryResultContactBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultContact {
        &self.inner
    }
}

/// Represents a link to a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Title of the resulting file
    title: String,
    /// Represents a link to a file
    description: String,
    /// URL of the file
    document_url: String,
    /// MIME type of the file content; only "application/pdf" and "application/zip" are currently allowed
    mime_type: String,
    /// The URL of the file thumbnail, if it exists
    thumbnail_url: String,
    /// Width of the thumbnail
    thumbnail_width: i32,
    /// Height of the thumbnail
    thumbnail_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageDocument, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultDocument {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultDocument"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultDocument {}

impl InputInlineQueryResultDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultDocumentBuilder {
        let mut inner = InputInlineQueryResultDocument::default();
        inner.td_name = "inputInlineQueryResultDocument".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultDocumentBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn document_url(&self) -> &String {
        &self.document_url
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn thumbnail_width(&self) -> i32 {
        self.thumbnail_width
    }

    pub fn thumbnail_height(&self) -> i32 {
        self.thumbnail_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultDocumentBuilder {
    inner: InputInlineQueryResultDocument,
}

impl RTDInputInlineQueryResultDocumentBuilder {
    pub fn build(&self) -> InputInlineQueryResultDocument {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn document_url<T: AsRef<str>>(&mut self, document_url: T) -> &mut Self {
        self.inner.document_url = document_url.as_ref().to_string();
        self
    }

    pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
        self.inner.mime_type = mime_type.as_ref().to_string();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn thumbnail_width(&mut self, thumbnail_width: i32) -> &mut Self {
        self.inner.thumbnail_width = thumbnail_width;
        self
    }

    pub fn thumbnail_height(&mut self, thumbnail_height: i32) -> &mut Self {
        self.inner.thumbnail_height = thumbnail_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultDocument> for InputInlineQueryResultDocument {
    fn as_ref(&self) -> &InputInlineQueryResultDocument {
        self
    }
}

impl AsRef<InputInlineQueryResultDocument> for RTDInputInlineQueryResultDocumentBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultDocument {
        &self.inner
    }
}

/// Represents a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Short name of the game
    game_short_name: String,
    /// Message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
}

impl RObject for InputInlineQueryResultGame {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultGame"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultGame {}

impl InputInlineQueryResultGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultGameBuilder {
        let mut inner = InputInlineQueryResultGame::default();
        inner.td_name = "inputInlineQueryResultGame".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultGameBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn game_short_name(&self) -> &String {
        &self.game_short_name
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultGameBuilder {
    inner: InputInlineQueryResultGame,
}

impl RTDInputInlineQueryResultGameBuilder {
    pub fn build(&self) -> InputInlineQueryResultGame {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
        self.inner.game_short_name = game_short_name.as_ref().to_string();
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultGame> for InputInlineQueryResultGame {
    fn as_ref(&self) -> &InputInlineQueryResultGame {
        self
    }
}

impl AsRef<InputInlineQueryResultGame> for RTDInputInlineQueryResultGameBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultGame {
        &self.inner
    }
}

/// Represents a point on the map
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Location result
    location: Location,
    /// Amount of time relative to the message sent time until the location can be updated, in seconds
    live_period: i32,
    /// Title of the result
    title: String,
    /// URL of the result thumbnail, if it exists
    thumbnail_url: String,
    /// Thumbnail width, if known
    thumbnail_width: i32,
    /// Thumbnail height, if known
    thumbnail_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultLocation {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultLocation"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultLocation {}

impl InputInlineQueryResultLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultLocationBuilder {
        let mut inner = InputInlineQueryResultLocation::default();
        inner.td_name = "inputInlineQueryResultLocation".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultLocationBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn live_period(&self) -> i32 {
        self.live_period
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn thumbnail_width(&self) -> i32 {
        self.thumbnail_width
    }

    pub fn thumbnail_height(&self) -> i32 {
        self.thumbnail_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultLocationBuilder {
    inner: InputInlineQueryResultLocation,
}

impl RTDInputInlineQueryResultLocationBuilder {
    pub fn build(&self) -> InputInlineQueryResultLocation {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }

    pub fn live_period(&mut self, live_period: i32) -> &mut Self {
        self.inner.live_period = live_period;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn thumbnail_width(&mut self, thumbnail_width: i32) -> &mut Self {
        self.inner.thumbnail_width = thumbnail_width;
        self
    }

    pub fn thumbnail_height(&mut self, thumbnail_height: i32) -> &mut Self {
        self.inner.thumbnail_height = thumbnail_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultLocation> for InputInlineQueryResultLocation {
    fn as_ref(&self) -> &InputInlineQueryResultLocation {
        self
    }
}

impl AsRef<InputInlineQueryResultLocation> for RTDInputInlineQueryResultLocationBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultLocation {
        &self.inner
    }
}

/// Represents link to a JPEG image
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Title of the result, if known
    title: String,
    /// Represents link to a JPEG image
    description: String,
    /// URL of the photo thumbnail, if it exists
    thumbnail_url: String,
    /// The URL of the JPEG photo (photo size must not exceed 5MB)
    photo_url: String,
    /// Width of the photo
    photo_width: i32,
    /// Height of the photo
    photo_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessagePhoto, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultPhoto {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultPhoto"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultPhoto {}

impl InputInlineQueryResultPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultPhotoBuilder {
        let mut inner = InputInlineQueryResultPhoto::default();
        inner.td_name = "inputInlineQueryResultPhoto".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultPhotoBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn photo_url(&self) -> &String {
        &self.photo_url
    }

    pub fn photo_width(&self) -> i32 {
        self.photo_width
    }

    pub fn photo_height(&self) -> i32 {
        self.photo_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultPhotoBuilder {
    inner: InputInlineQueryResultPhoto,
}

impl RTDInputInlineQueryResultPhotoBuilder {
    pub fn build(&self) -> InputInlineQueryResultPhoto {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn photo_url<T: AsRef<str>>(&mut self, photo_url: T) -> &mut Self {
        self.inner.photo_url = photo_url.as_ref().to_string();
        self
    }

    pub fn photo_width(&mut self, photo_width: i32) -> &mut Self {
        self.inner.photo_width = photo_width;
        self
    }

    pub fn photo_height(&mut self, photo_height: i32) -> &mut Self {
        self.inner.photo_height = photo_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultPhoto> for InputInlineQueryResultPhoto {
    fn as_ref(&self) -> &InputInlineQueryResultPhoto {
        self
    }
}

impl AsRef<InputInlineQueryResultPhoto> for RTDInputInlineQueryResultPhotoBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultPhoto {
        &self.inner
    }
}

/// Represents a link to a WEBP or TGS sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// URL of the sticker thumbnail, if it exists
    thumbnail_url: String,
    /// The URL of the WEBP or TGS sticker (sticker file size must not exceed 5MB)
    sticker_url: String,
    /// Width of the sticker
    sticker_width: i32,
    /// Height of the sticker
    sticker_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, inputMessageSticker, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultSticker {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultSticker"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultSticker {}

impl InputInlineQueryResultSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultStickerBuilder {
        let mut inner = InputInlineQueryResultSticker::default();
        inner.td_name = "inputInlineQueryResultSticker".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultStickerBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn sticker_url(&self) -> &String {
        &self.sticker_url
    }

    pub fn sticker_width(&self) -> i32 {
        self.sticker_width
    }

    pub fn sticker_height(&self) -> i32 {
        self.sticker_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultStickerBuilder {
    inner: InputInlineQueryResultSticker,
}

impl RTDInputInlineQueryResultStickerBuilder {
    pub fn build(&self) -> InputInlineQueryResultSticker {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn sticker_url<T: AsRef<str>>(&mut self, sticker_url: T) -> &mut Self {
        self.inner.sticker_url = sticker_url.as_ref().to_string();
        self
    }

    pub fn sticker_width(&mut self, sticker_width: i32) -> &mut Self {
        self.inner.sticker_width = sticker_width;
        self
    }

    pub fn sticker_height(&mut self, sticker_height: i32) -> &mut Self {
        self.inner.sticker_height = sticker_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultSticker> for InputInlineQueryResultSticker {
    fn as_ref(&self) -> &InputInlineQueryResultSticker {
        self
    }
}

impl AsRef<InputInlineQueryResultSticker> for RTDInputInlineQueryResultStickerBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultSticker {
        &self.inner
    }
}

/// Represents information about a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultVenue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Venue result
    venue: Venue,
    /// URL of the result thumbnail, if it exists
    thumbnail_url: String,
    /// Thumbnail width, if known
    thumbnail_width: i32,
    /// Thumbnail height, if known
    thumbnail_height: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultVenue {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultVenue"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultVenue {}

impl InputInlineQueryResultVenue {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultVenueBuilder {
        let mut inner = InputInlineQueryResultVenue::default();
        inner.td_name = "inputInlineQueryResultVenue".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultVenueBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn venue(&self) -> &Venue {
        &self.venue
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn thumbnail_width(&self) -> i32 {
        self.thumbnail_width
    }

    pub fn thumbnail_height(&self) -> i32 {
        self.thumbnail_height
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultVenueBuilder {
    inner: InputInlineQueryResultVenue,
}

impl RTDInputInlineQueryResultVenueBuilder {
    pub fn build(&self) -> InputInlineQueryResultVenue {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
        self.inner.venue = venue.as_ref().clone();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn thumbnail_width(&mut self, thumbnail_width: i32) -> &mut Self {
        self.inner.thumbnail_width = thumbnail_width;
        self
    }

    pub fn thumbnail_height(&mut self, thumbnail_height: i32) -> &mut Self {
        self.inner.thumbnail_height = thumbnail_height;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultVenue> for InputInlineQueryResultVenue {
    fn as_ref(&self) -> &InputInlineQueryResultVenue {
        self
    }
}

impl AsRef<InputInlineQueryResultVenue> for RTDInputInlineQueryResultVenueBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultVenue {
        &self.inner
    }
}

/// Represents a link to a page containing an embedded video player or a video file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Title of the result
    title: String,
    /// Represents a link to a page containing an embedded video player or a video file
    description: String,
    /// The URL of the video thumbnail (JPEG), if it exists
    thumbnail_url: String,
    /// URL of the embedded video player or video file
    video_url: String,
    /// MIME type of the content of the video URL, only "text/html" or "video/mp4" are currently supported
    mime_type: String,
    /// Width of the video
    video_width: i32,
    /// Height of the video
    video_height: i32,
    /// Video duration, in seconds
    video_duration: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVideo, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultVideo {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultVideo"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultVideo {}

impl InputInlineQueryResultVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultVideoBuilder {
        let mut inner = InputInlineQueryResultVideo::default();
        inner.td_name = "inputInlineQueryResultVideo".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultVideoBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn thumbnail_url(&self) -> &String {
        &self.thumbnail_url
    }

    pub fn video_url(&self) -> &String {
        &self.video_url
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }

    pub fn video_width(&self) -> i32 {
        self.video_width
    }

    pub fn video_height(&self) -> i32 {
        self.video_height
    }

    pub fn video_duration(&self) -> i32 {
        self.video_duration
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultVideoBuilder {
    inner: InputInlineQueryResultVideo,
}

impl RTDInputInlineQueryResultVideoBuilder {
    pub fn build(&self) -> InputInlineQueryResultVideo {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }

    pub fn thumbnail_url<T: AsRef<str>>(&mut self, thumbnail_url: T) -> &mut Self {
        self.inner.thumbnail_url = thumbnail_url.as_ref().to_string();
        self
    }

    pub fn video_url<T: AsRef<str>>(&mut self, video_url: T) -> &mut Self {
        self.inner.video_url = video_url.as_ref().to_string();
        self
    }

    pub fn mime_type<T: AsRef<str>>(&mut self, mime_type: T) -> &mut Self {
        self.inner.mime_type = mime_type.as_ref().to_string();
        self
    }

    pub fn video_width(&mut self, video_width: i32) -> &mut Self {
        self.inner.video_width = video_width;
        self
    }

    pub fn video_height(&mut self, video_height: i32) -> &mut Self {
        self.inner.video_height = video_height;
        self
    }

    pub fn video_duration(&mut self, video_duration: i32) -> &mut Self {
        self.inner.video_duration = video_duration;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultVideo> for InputInlineQueryResultVideo {
    fn as_ref(&self) -> &InputInlineQueryResultVideo {
        self
    }
}

impl AsRef<InputInlineQueryResultVideo> for RTDInputInlineQueryResultVideoBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultVideo {
        &self.inner
    }
}

/// Represents a link to an opus-encoded audio file within an OGG container, single channel audio
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputInlineQueryResultVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// Unique identifier of the query result
    id: String,
    /// Title of the voice note
    title: String,
    /// The URL of the voice note file
    voice_note_url: String,
    /// Duration of the voice note, in seconds
    voice_note_duration: i32,
    /// The message reply markup. Must be of type replyMarkupInlineKeyboard or null
    reply_markup: ReplyMarkup,
    /// The content of the message to be sent. Must be one of the following types: InputMessageText, InputMessageVoiceNote, InputMessageLocation, InputMessageVenue or InputMessageContact
    input_message_content: InputMessageContent,
}

impl RObject for InputInlineQueryResultVoiceNote {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "inputInlineQueryResultVoiceNote"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDInputInlineQueryResult for InputInlineQueryResultVoiceNote {}

impl InputInlineQueryResultVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputInlineQueryResultVoiceNoteBuilder {
        let mut inner = InputInlineQueryResultVoiceNote::default();
        inner.td_name = "inputInlineQueryResultVoiceNote".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDInputInlineQueryResultVoiceNoteBuilder { inner }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn voice_note_url(&self) -> &String {
        &self.voice_note_url
    }

    pub fn voice_note_duration(&self) -> i32 {
        self.voice_note_duration
    }

    pub fn reply_markup(&self) -> &ReplyMarkup {
        &self.reply_markup
    }

    pub fn input_message_content(&self) -> &InputMessageContent {
        &self.input_message_content
    }
}

#[doc(hidden)]
pub struct RTDInputInlineQueryResultVoiceNoteBuilder {
    inner: InputInlineQueryResultVoiceNote,
}

impl RTDInputInlineQueryResultVoiceNoteBuilder {
    pub fn build(&self) -> InputInlineQueryResultVoiceNote {
        self.inner.clone()
    }

    pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
        self.inner.id = id.as_ref().to_string();
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn voice_note_url<T: AsRef<str>>(&mut self, voice_note_url: T) -> &mut Self {
        self.inner.voice_note_url = voice_note_url.as_ref().to_string();
        self
    }

    pub fn voice_note_duration(&mut self, voice_note_duration: i32) -> &mut Self {
        self.inner.voice_note_duration = voice_note_duration;
        self
    }

    pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
        self.inner.reply_markup = reply_markup.as_ref().clone();
        self
    }

    pub fn input_message_content<T: AsRef<InputMessageContent>>(
        &mut self,
        input_message_content: T,
    ) -> &mut Self {
        self.inner.input_message_content = input_message_content.as_ref().clone();
        self
    }
}

impl AsRef<InputInlineQueryResultVoiceNote> for InputInlineQueryResultVoiceNote {
    fn as_ref(&self) -> &InputInlineQueryResultVoiceNote {
        self
    }
}

impl AsRef<InputInlineQueryResultVoiceNote> for RTDInputInlineQueryResultVoiceNoteBuilder {
    fn as_ref(&self) -> &InputInlineQueryResultVoiceNote {
        &self.inner
    }
}
