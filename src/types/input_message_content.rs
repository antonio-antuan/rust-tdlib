use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// The content of a message to send
pub trait TDInputMessageContent: Debug + RObject {}

/// The content of a message to send
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum InputMessageContent {
    #[doc(hidden)]
    _Default,
    /// An animation message (GIF-style).
    #[serde(rename(
        serialize = "inputMessageAnimation",
        deserialize = "inputMessageAnimation"
    ))]
    InputMessageAnimation(InputMessageAnimation),
    /// An audio message
    #[serde(rename(serialize = "inputMessageAudio", deserialize = "inputMessageAudio"))]
    InputMessageAudio(InputMessageAudio),
    /// A message containing a user contact
    #[serde(rename(serialize = "inputMessageContact", deserialize = "inputMessageContact"))]
    InputMessageContact(InputMessageContact),
    /// A dice message
    #[serde(rename(serialize = "inputMessageDice", deserialize = "inputMessageDice"))]
    InputMessageDice(InputMessageDice),
    /// A document message (general file)
    #[serde(rename(
        serialize = "inputMessageDocument",
        deserialize = "inputMessageDocument"
    ))]
    InputMessageDocument(InputMessageDocument),
    /// A forwarded message
    #[serde(rename(
        serialize = "inputMessageForwarded",
        deserialize = "inputMessageForwarded"
    ))]
    InputMessageForwarded(InputMessageForwarded),
    /// A message with a game; not supported for channels or secret chats
    #[serde(rename(serialize = "inputMessageGame", deserialize = "inputMessageGame"))]
    InputMessageGame(InputMessageGame),
    /// A message with an invoice; can be used only by bots and only in private chats
    #[serde(rename(serialize = "inputMessageInvoice", deserialize = "inputMessageInvoice"))]
    InputMessageInvoice(InputMessageInvoice),
    /// A message with a location
    #[serde(rename(
        serialize = "inputMessageLocation",
        deserialize = "inputMessageLocation"
    ))]
    InputMessageLocation(InputMessageLocation),
    /// A photo message
    #[serde(rename(serialize = "inputMessagePhoto", deserialize = "inputMessagePhoto"))]
    InputMessagePhoto(InputMessagePhoto),
    /// A message with a poll. Polls can't be sent to secret chats. Polls can be sent only to a private chat with a bot
    #[serde(rename(serialize = "inputMessagePoll", deserialize = "inputMessagePoll"))]
    InputMessagePoll(InputMessagePoll),
    /// A sticker message
    #[serde(rename(serialize = "inputMessageSticker", deserialize = "inputMessageSticker"))]
    InputMessageSticker(InputMessageSticker),
    /// A text message
    #[serde(rename(serialize = "inputMessageText", deserialize = "inputMessageText"))]
    InputMessageText(InputMessageText),
    /// A message with information about a venue
    #[serde(rename(serialize = "inputMessageVenue", deserialize = "inputMessageVenue"))]
    InputMessageVenue(InputMessageVenue),
    /// A video message
    #[serde(rename(serialize = "inputMessageVideo", deserialize = "inputMessageVideo"))]
    InputMessageVideo(InputMessageVideo),
    /// A video note message
    #[serde(rename(
        serialize = "inputMessageVideoNote",
        deserialize = "inputMessageVideoNote"
    ))]
    InputMessageVideoNote(InputMessageVideoNote),
    /// A voice note message
    #[serde(rename(
        serialize = "inputMessageVoiceNote",
        deserialize = "inputMessageVoiceNote"
    ))]
    InputMessageVoiceNote(InputMessageVoiceNote),
}

impl Default for InputMessageContent {
    fn default() -> Self {
        InputMessageContent::_Default
    }
}

impl RObject for InputMessageContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            InputMessageContent::InputMessageAnimation(t) => t.extra(),
            InputMessageContent::InputMessageAudio(t) => t.extra(),
            InputMessageContent::InputMessageContact(t) => t.extra(),
            InputMessageContent::InputMessageDice(t) => t.extra(),
            InputMessageContent::InputMessageDocument(t) => t.extra(),
            InputMessageContent::InputMessageForwarded(t) => t.extra(),
            InputMessageContent::InputMessageGame(t) => t.extra(),
            InputMessageContent::InputMessageInvoice(t) => t.extra(),
            InputMessageContent::InputMessageLocation(t) => t.extra(),
            InputMessageContent::InputMessagePhoto(t) => t.extra(),
            InputMessageContent::InputMessagePoll(t) => t.extra(),
            InputMessageContent::InputMessageSticker(t) => t.extra(),
            InputMessageContent::InputMessageText(t) => t.extra(),
            InputMessageContent::InputMessageVenue(t) => t.extra(),
            InputMessageContent::InputMessageVideo(t) => t.extra(),
            InputMessageContent::InputMessageVideoNote(t) => t.extra(),
            InputMessageContent::InputMessageVoiceNote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            InputMessageContent::InputMessageAnimation(t) => t.client_id(),
            InputMessageContent::InputMessageAudio(t) => t.client_id(),
            InputMessageContent::InputMessageContact(t) => t.client_id(),
            InputMessageContent::InputMessageDice(t) => t.client_id(),
            InputMessageContent::InputMessageDocument(t) => t.client_id(),
            InputMessageContent::InputMessageForwarded(t) => t.client_id(),
            InputMessageContent::InputMessageGame(t) => t.client_id(),
            InputMessageContent::InputMessageInvoice(t) => t.client_id(),
            InputMessageContent::InputMessageLocation(t) => t.client_id(),
            InputMessageContent::InputMessagePhoto(t) => t.client_id(),
            InputMessageContent::InputMessagePoll(t) => t.client_id(),
            InputMessageContent::InputMessageSticker(t) => t.client_id(),
            InputMessageContent::InputMessageText(t) => t.client_id(),
            InputMessageContent::InputMessageVenue(t) => t.client_id(),
            InputMessageContent::InputMessageVideo(t) => t.client_id(),
            InputMessageContent::InputMessageVideoNote(t) => t.client_id(),
            InputMessageContent::InputMessageVoiceNote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl InputMessageContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, InputMessageContent::_Default)
    }
}

impl AsRef<InputMessageContent> for InputMessageContent {
    fn as_ref(&self) -> &InputMessageContent {
        self
    }
}

/// An animation message (GIF-style).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Animation file to be sent

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    animation: InputFile,
    /// Animation thumbnail, if available
    thumbnail: InputThumbnail,
    /// File identifiers of the stickers added to the animation, if applicable
    added_sticker_file_ids: Vec<i32>,
    /// Duration of the animation, in seconds
    duration: i32,
    /// Width of the animation; may be replaced by the server
    width: i32,
    /// Height of the animation; may be replaced by the server
    height: i32,
    /// Animation caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
}

impl RObject for InputMessageAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageAnimation {}

impl InputMessageAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageAnimationBuilder {
        let mut inner = InputMessageAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageAnimationBuilder { inner }
    }

    pub fn animation(&self) -> &InputFile {
        &self.animation
    }

    pub fn thumbnail(&self) -> &InputThumbnail {
        &self.thumbnail
    }

    pub fn added_sticker_file_ids(&self) -> &Vec<i32> {
        &self.added_sticker_file_ids
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDInputMessageAnimationBuilder {
    inner: InputMessageAnimation,
}

impl RTDInputMessageAnimationBuilder {
    pub fn build(&self) -> InputMessageAnimation {
        self.inner.clone()
    }

    pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = animation.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
        self
    }

    pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self {
        self.inner.added_sticker_file_ids = added_sticker_file_ids;
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
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

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageAnimation> for InputMessageAnimation {
    fn as_ref(&self) -> &InputMessageAnimation {
        self
    }
}

impl AsRef<InputMessageAnimation> for RTDInputMessageAnimationBuilder {
    fn as_ref(&self) -> &InputMessageAnimation {
        &self.inner
    }
}

/// An audio message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageAudio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Audio file to be sent

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    audio: InputFile,
    /// Thumbnail of the cover for the album, if available
    album_cover_thumbnail: InputThumbnail,
    /// Duration of the audio, in seconds; may be replaced by the server
    duration: i32,
    /// Title of the audio; 0-64 characters; may be replaced by the server
    title: String,
    /// Performer of the audio; 0-64 characters, may be replaced by the server
    performer: String,
    /// Audio caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
}

impl RObject for InputMessageAudio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageAudio {}

impl InputMessageAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageAudioBuilder {
        let mut inner = InputMessageAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageAudioBuilder { inner }
    }

    pub fn audio(&self) -> &InputFile {
        &self.audio
    }

    pub fn album_cover_thumbnail(&self) -> &InputThumbnail {
        &self.album_cover_thumbnail
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn performer(&self) -> &String {
        &self.performer
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDInputMessageAudioBuilder {
    inner: InputMessageAudio,
}

impl RTDInputMessageAudioBuilder {
    pub fn build(&self) -> InputMessageAudio {
        self.inner.clone()
    }

    pub fn audio<T: AsRef<InputFile>>(&mut self, audio: T) -> &mut Self {
        self.inner.audio = audio.as_ref().clone();
        self
    }

    pub fn album_cover_thumbnail<T: AsRef<InputThumbnail>>(
        &mut self,
        album_cover_thumbnail: T,
    ) -> &mut Self {
        self.inner.album_cover_thumbnail = album_cover_thumbnail.as_ref().clone();
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
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

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageAudio> for InputMessageAudio {
    fn as_ref(&self) -> &InputMessageAudio {
        self
    }
}

impl AsRef<InputMessageAudio> for RTDInputMessageAudioBuilder {
    fn as_ref(&self) -> &InputMessageAudio {
        &self.inner
    }
}

/// A message containing a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageContact {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Contact to send
    contact: Contact,
}

impl RObject for InputMessageContact {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageContact {}

impl InputMessageContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageContactBuilder {
        let mut inner = InputMessageContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageContactBuilder { inner }
    }

    pub fn contact(&self) -> &Contact {
        &self.contact
    }
}

#[doc(hidden)]
pub struct RTDInputMessageContactBuilder {
    inner: InputMessageContact,
}

impl RTDInputMessageContactBuilder {
    pub fn build(&self) -> InputMessageContact {
        self.inner.clone()
    }

    pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
        self.inner.contact = contact.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageContact> for InputMessageContact {
    fn as_ref(&self) -> &InputMessageContact {
        self
    }
}

impl AsRef<InputMessageContact> for RTDInputMessageContactBuilder {
    fn as_ref(&self) -> &InputMessageContact {
        &self.inner
    }
}

/// A dice message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageDice {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Emoji on which the dice throw animation is based
    emoji: String,
    /// True, if a chat message draft should be deleted
    clear_draft: bool,
}

impl RObject for InputMessageDice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageDice {}

impl InputMessageDice {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageDiceBuilder {
        let mut inner = InputMessageDice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageDiceBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }

    pub fn clear_draft(&self) -> bool {
        self.clear_draft
    }
}

#[doc(hidden)]
pub struct RTDInputMessageDiceBuilder {
    inner: InputMessageDice,
}

impl RTDInputMessageDiceBuilder {
    pub fn build(&self) -> InputMessageDice {
        self.inner.clone()
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }

    pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
        self.inner.clear_draft = clear_draft;
        self
    }
}

impl AsRef<InputMessageDice> for InputMessageDice {
    fn as_ref(&self) -> &InputMessageDice {
        self
    }
}

impl AsRef<InputMessageDice> for RTDInputMessageDiceBuilder {
    fn as_ref(&self) -> &InputMessageDice {
        &self.inner
    }
}

/// A document message (general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Document to be sent

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    document: InputFile,
    /// Document thumbnail, if available
    thumbnail: InputThumbnail,
    /// If true, automatic file type detection will be disabled and the document will be always sent as file. Always true for files sent to secret chats
    disable_content_type_detection: bool,
    /// Document caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
}

impl RObject for InputMessageDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageDocument {}

impl InputMessageDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageDocumentBuilder {
        let mut inner = InputMessageDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageDocumentBuilder { inner }
    }

    pub fn document(&self) -> &InputFile {
        &self.document
    }

    pub fn thumbnail(&self) -> &InputThumbnail {
        &self.thumbnail
    }

    pub fn disable_content_type_detection(&self) -> bool {
        self.disable_content_type_detection
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDInputMessageDocumentBuilder {
    inner: InputMessageDocument,
}

impl RTDInputMessageDocumentBuilder {
    pub fn build(&self) -> InputMessageDocument {
        self.inner.clone()
    }

    pub fn document<T: AsRef<InputFile>>(&mut self, document: T) -> &mut Self {
        self.inner.document = document.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
        self
    }

    pub fn disable_content_type_detection(
        &mut self,
        disable_content_type_detection: bool,
    ) -> &mut Self {
        self.inner.disable_content_type_detection = disable_content_type_detection;
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageDocument> for InputMessageDocument {
    fn as_ref(&self) -> &InputMessageDocument {
        self
    }
}

impl AsRef<InputMessageDocument> for RTDInputMessageDocumentBuilder {
    fn as_ref(&self) -> &InputMessageDocument {
        &self.inner
    }
}

/// A forwarded message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageForwarded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier for the chat this forwarded message came from
    from_chat_id: i64,
    /// Identifier of the message to forward
    message_id: i64,
    /// True, if a game message should be shared within a launched game; applies only to game messages
    in_game_share: bool,
    /// Options to be used to copy content of the message without a link to the original message
    copy_options: MessageCopyOptions,
}

impl RObject for InputMessageForwarded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageForwarded {}

impl InputMessageForwarded {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageForwardedBuilder {
        let mut inner = InputMessageForwarded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageForwardedBuilder { inner }
    }

    pub fn from_chat_id(&self) -> i64 {
        self.from_chat_id
    }

    pub fn message_id(&self) -> i64 {
        self.message_id
    }

    pub fn in_game_share(&self) -> bool {
        self.in_game_share
    }

    pub fn copy_options(&self) -> &MessageCopyOptions {
        &self.copy_options
    }
}

#[doc(hidden)]
pub struct RTDInputMessageForwardedBuilder {
    inner: InputMessageForwarded,
}

impl RTDInputMessageForwardedBuilder {
    pub fn build(&self) -> InputMessageForwarded {
        self.inner.clone()
    }

    pub fn from_chat_id(&mut self, from_chat_id: i64) -> &mut Self {
        self.inner.from_chat_id = from_chat_id;
        self
    }

    pub fn message_id(&mut self, message_id: i64) -> &mut Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn in_game_share(&mut self, in_game_share: bool) -> &mut Self {
        self.inner.in_game_share = in_game_share;
        self
    }

    pub fn copy_options<T: AsRef<MessageCopyOptions>>(&mut self, copy_options: T) -> &mut Self {
        self.inner.copy_options = copy_options.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageForwarded> for InputMessageForwarded {
    fn as_ref(&self) -> &InputMessageForwarded {
        self
    }
}

impl AsRef<InputMessageForwarded> for RTDInputMessageForwardedBuilder {
    fn as_ref(&self) -> &InputMessageForwarded {
        &self.inner
    }
}

/// A message with a game; not supported for channels or secret chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier of the bot that owns the game
    bot_user_id: i32,
    /// Short name of the game
    game_short_name: String,
}

impl RObject for InputMessageGame {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageGame {}

impl InputMessageGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageGameBuilder {
        let mut inner = InputMessageGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageGameBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i32 {
        self.bot_user_id
    }

    pub fn game_short_name(&self) -> &String {
        &self.game_short_name
    }
}

#[doc(hidden)]
pub struct RTDInputMessageGameBuilder {
    inner: InputMessageGame,
}

impl RTDInputMessageGameBuilder {
    pub fn build(&self) -> InputMessageGame {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i32) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn game_short_name<T: AsRef<str>>(&mut self, game_short_name: T) -> &mut Self {
        self.inner.game_short_name = game_short_name.as_ref().to_string();
        self
    }
}

impl AsRef<InputMessageGame> for InputMessageGame {
    fn as_ref(&self) -> &InputMessageGame {
        self
    }
}

impl AsRef<InputMessageGame> for RTDInputMessageGameBuilder {
    fn as_ref(&self) -> &InputMessageGame {
        &self.inner
    }
}

/// A message with an invoice; can be used only by bots and only in private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageInvoice {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Invoice
    invoice: Invoice,
    /// Product title; 1-32 characters
    title: String,
    /// A message with an invoice; can be used only by bots and only in private chats
    description: String,
    /// Product photo URL; optional
    photo_url: String,
    /// Product photo size
    photo_size: i32,
    /// Product photo width
    photo_width: i32,
    /// Product photo height
    photo_height: i32,
    /// The invoice payload
    payload: String,
    /// Payment provider token
    provider_token: String,
    /// JSON-encoded data about the invoice, which will be shared with the payment provider
    provider_data: String,
    /// Unique invoice bot start_parameter for the generation of this invoice
    start_parameter: String,
}

impl RObject for InputMessageInvoice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageInvoice {}

impl InputMessageInvoice {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageInvoiceBuilder {
        let mut inner = InputMessageInvoice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageInvoiceBuilder { inner }
    }

    pub fn invoice(&self) -> &Invoice {
        &self.invoice
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn photo_url(&self) -> &String {
        &self.photo_url
    }

    pub fn photo_size(&self) -> i32 {
        self.photo_size
    }

    pub fn photo_width(&self) -> i32 {
        self.photo_width
    }

    pub fn photo_height(&self) -> i32 {
        self.photo_height
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }

    pub fn provider_token(&self) -> &String {
        &self.provider_token
    }

    pub fn provider_data(&self) -> &String {
        &self.provider_data
    }

    pub fn start_parameter(&self) -> &String {
        &self.start_parameter
    }
}

#[doc(hidden)]
pub struct RTDInputMessageInvoiceBuilder {
    inner: InputMessageInvoice,
}

impl RTDInputMessageInvoiceBuilder {
    pub fn build(&self) -> InputMessageInvoice {
        self.inner.clone()
    }

    pub fn invoice<T: AsRef<Invoice>>(&mut self, invoice: T) -> &mut Self {
        self.inner.invoice = invoice.as_ref().clone();
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

    pub fn photo_url<T: AsRef<str>>(&mut self, photo_url: T) -> &mut Self {
        self.inner.photo_url = photo_url.as_ref().to_string();
        self
    }

    pub fn photo_size(&mut self, photo_size: i32) -> &mut Self {
        self.inner.photo_size = photo_size;
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

    pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().to_string();
        self
    }

    pub fn provider_token<T: AsRef<str>>(&mut self, provider_token: T) -> &mut Self {
        self.inner.provider_token = provider_token.as_ref().to_string();
        self
    }

    pub fn provider_data<T: AsRef<str>>(&mut self, provider_data: T) -> &mut Self {
        self.inner.provider_data = provider_data.as_ref().to_string();
        self
    }

    pub fn start_parameter<T: AsRef<str>>(&mut self, start_parameter: T) -> &mut Self {
        self.inner.start_parameter = start_parameter.as_ref().to_string();
        self
    }
}

impl AsRef<InputMessageInvoice> for InputMessageInvoice {
    fn as_ref(&self) -> &InputMessageInvoice {
        self
    }
}

impl AsRef<InputMessageInvoice> for RTDInputMessageInvoiceBuilder {
    fn as_ref(&self) -> &InputMessageInvoice {
        &self.inner
    }
}

/// A message with a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Location to be sent
    location: Location,
    /// Period for which the location can be updated, in seconds; should be between 60 and 86400 for a live location and 0 otherwise
    live_period: i32,
    /// For live locations, a direction in which the location moves, in degrees; 1-360. Pass 0 if unknown
    heading: i32,
    /// For live locations, a maximum distance to another chat member for proximity alerts, in meters (0-100000). Pass 0 if the notification is disabled. Can't be enabled in channels and Saved Messages
    proximity_alert_radius: i32,
}

impl RObject for InputMessageLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageLocation {}

impl InputMessageLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageLocationBuilder {
        let mut inner = InputMessageLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageLocationBuilder { inner }
    }

    pub fn location(&self) -> &Location {
        &self.location
    }

    pub fn live_period(&self) -> i32 {
        self.live_period
    }

    pub fn heading(&self) -> i32 {
        self.heading
    }

    pub fn proximity_alert_radius(&self) -> i32 {
        self.proximity_alert_radius
    }
}

#[doc(hidden)]
pub struct RTDInputMessageLocationBuilder {
    inner: InputMessageLocation,
}

impl RTDInputMessageLocationBuilder {
    pub fn build(&self) -> InputMessageLocation {
        self.inner.clone()
    }

    pub fn location<T: AsRef<Location>>(&mut self, location: T) -> &mut Self {
        self.inner.location = location.as_ref().clone();
        self
    }

    pub fn live_period(&mut self, live_period: i32) -> &mut Self {
        self.inner.live_period = live_period;
        self
    }

    pub fn heading(&mut self, heading: i32) -> &mut Self {
        self.inner.heading = heading;
        self
    }

    pub fn proximity_alert_radius(&mut self, proximity_alert_radius: i32) -> &mut Self {
        self.inner.proximity_alert_radius = proximity_alert_radius;
        self
    }
}

impl AsRef<InputMessageLocation> for InputMessageLocation {
    fn as_ref(&self) -> &InputMessageLocation {
        self
    }
}

impl AsRef<InputMessageLocation> for RTDInputMessageLocationBuilder {
    fn as_ref(&self) -> &InputMessageLocation {
        &self.inner
    }
}

/// A photo message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessagePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Photo to send

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    photo: InputFile,
    /// Photo thumbnail to be sent, this is sent to the other party in secret chats only
    thumbnail: InputThumbnail,
    /// File identifiers of the stickers added to the photo, if applicable
    added_sticker_file_ids: Vec<i32>,
    /// Photo width
    width: i32,
    /// Photo height
    height: i32,
    /// Photo caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
    /// Photo TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats
    ttl: i32,
}

impl RObject for InputMessagePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessagePhoto {}

impl InputMessagePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessagePhotoBuilder {
        let mut inner = InputMessagePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessagePhotoBuilder { inner }
    }

    pub fn photo(&self) -> &InputFile {
        &self.photo
    }

    pub fn thumbnail(&self) -> &InputThumbnail {
        &self.thumbnail
    }

    pub fn added_sticker_file_ids(&self) -> &Vec<i32> {
        &self.added_sticker_file_ids
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }
}

#[doc(hidden)]
pub struct RTDInputMessagePhotoBuilder {
    inner: InputMessagePhoto,
}

impl RTDInputMessagePhotoBuilder {
    pub fn build(&self) -> InputMessagePhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = photo.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
        self
    }

    pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self {
        self.inner.added_sticker_file_ids = added_sticker_file_ids;
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

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }
}

impl AsRef<InputMessagePhoto> for InputMessagePhoto {
    fn as_ref(&self) -> &InputMessagePhoto {
        self
    }
}

impl AsRef<InputMessagePhoto> for RTDInputMessagePhotoBuilder {
    fn as_ref(&self) -> &InputMessagePhoto {
        &self.inner
    }
}

/// A message with a poll. Polls can't be sent to secret chats. Polls can be sent only to a private chat with a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessagePoll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Poll question, 1-255 characters (up to 300 characters for bots)
    question: String,
    /// List of poll answer options, 2-10 strings 1-100 characters each
    options: Vec<String>,
    /// True, if the poll voters are anonymous. Non-anonymous polls can't be sent or forwarded to channels
    is_anonymous: bool,
    /// Type of the poll

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PollType::_is_default")]
    type_: PollType,
    /// Amount of time the poll will be active after creation, in seconds; for bots only
    open_period: i32,
    /// Point in time (Unix timestamp) when the poll will be automatically closed; for bots only
    close_date: i32,
    /// True, if the poll needs to be sent already closed; for bots only
    is_closed: bool,
}

impl RObject for InputMessagePoll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessagePoll {}

impl InputMessagePoll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessagePollBuilder {
        let mut inner = InputMessagePoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessagePollBuilder { inner }
    }

    pub fn question(&self) -> &String {
        &self.question
    }

    pub fn options(&self) -> &Vec<String> {
        &self.options
    }

    pub fn is_anonymous(&self) -> bool {
        self.is_anonymous
    }

    pub fn type_(&self) -> &PollType {
        &self.type_
    }

    pub fn open_period(&self) -> i32 {
        self.open_period
    }

    pub fn close_date(&self) -> i32 {
        self.close_date
    }

    pub fn is_closed(&self) -> bool {
        self.is_closed
    }
}

#[doc(hidden)]
pub struct RTDInputMessagePollBuilder {
    inner: InputMessagePoll,
}

impl RTDInputMessagePollBuilder {
    pub fn build(&self) -> InputMessagePoll {
        self.inner.clone()
    }

    pub fn question<T: AsRef<str>>(&mut self, question: T) -> &mut Self {
        self.inner.question = question.as_ref().to_string();
        self
    }

    pub fn options(&mut self, options: Vec<String>) -> &mut Self {
        self.inner.options = options;
        self
    }

    pub fn is_anonymous(&mut self, is_anonymous: bool) -> &mut Self {
        self.inner.is_anonymous = is_anonymous;
        self
    }

    pub fn type_<T: AsRef<PollType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn open_period(&mut self, open_period: i32) -> &mut Self {
        self.inner.open_period = open_period;
        self
    }

    pub fn close_date(&mut self, close_date: i32) -> &mut Self {
        self.inner.close_date = close_date;
        self
    }

    pub fn is_closed(&mut self, is_closed: bool) -> &mut Self {
        self.inner.is_closed = is_closed;
        self
    }
}

impl AsRef<InputMessagePoll> for InputMessagePoll {
    fn as_ref(&self) -> &InputMessagePoll {
        self
    }
}

impl AsRef<InputMessagePoll> for RTDInputMessagePollBuilder {
    fn as_ref(&self) -> &InputMessagePoll {
        &self.inner
    }
}

/// A sticker message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Sticker to be sent

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    sticker: InputFile,
    /// Sticker thumbnail, if available
    thumbnail: InputThumbnail,
    /// Sticker width
    width: i32,
    /// Sticker height
    height: i32,
}

impl RObject for InputMessageSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageSticker {}

impl InputMessageSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageStickerBuilder {
        let mut inner = InputMessageSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageStickerBuilder { inner }
    }

    pub fn sticker(&self) -> &InputFile {
        &self.sticker
    }

    pub fn thumbnail(&self) -> &InputThumbnail {
        &self.thumbnail
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }
}

#[doc(hidden)]
pub struct RTDInputMessageStickerBuilder {
    inner: InputMessageSticker,
}

impl RTDInputMessageStickerBuilder {
    pub fn build(&self) -> InputMessageSticker {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
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
}

impl AsRef<InputMessageSticker> for InputMessageSticker {
    fn as_ref(&self) -> &InputMessageSticker {
        self
    }
}

impl AsRef<InputMessageSticker> for RTDInputMessageStickerBuilder {
    fn as_ref(&self) -> &InputMessageSticker {
        &self.inner
    }
}

/// A text message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Formatted text to be sent; 1-GetOption("message_text_length_max") characters. Only Bold, Italic, Underline, Strikethrough, Code, Pre, PreCode, TextUrl and MentionName entities are allowed to be specified manually
    text: FormattedText,
    /// True, if rich web page previews for URLs in the message text should be disabled
    disable_web_page_preview: bool,
    /// True, if a chat message draft should be deleted
    clear_draft: bool,
}

impl RObject for InputMessageText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageText {}

impl InputMessageText {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageTextBuilder {
        let mut inner = InputMessageText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageTextBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn disable_web_page_preview(&self) -> bool {
        self.disable_web_page_preview
    }

    pub fn clear_draft(&self) -> bool {
        self.clear_draft
    }
}

#[doc(hidden)]
pub struct RTDInputMessageTextBuilder {
    inner: InputMessageText,
}

impl RTDInputMessageTextBuilder {
    pub fn build(&self) -> InputMessageText {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn disable_web_page_preview(&mut self, disable_web_page_preview: bool) -> &mut Self {
        self.inner.disable_web_page_preview = disable_web_page_preview;
        self
    }

    pub fn clear_draft(&mut self, clear_draft: bool) -> &mut Self {
        self.inner.clear_draft = clear_draft;
        self
    }
}

impl AsRef<InputMessageText> for InputMessageText {
    fn as_ref(&self) -> &InputMessageText {
        self
    }
}

impl AsRef<InputMessageText> for RTDInputMessageTextBuilder {
    fn as_ref(&self) -> &InputMessageText {
        &self.inner
    }
}

/// A message with information about a venue
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVenue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Venue to send
    venue: Venue,
}

impl RObject for InputMessageVenue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageVenue {}

impl InputMessageVenue {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageVenueBuilder {
        let mut inner = InputMessageVenue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageVenueBuilder { inner }
    }

    pub fn venue(&self) -> &Venue {
        &self.venue
    }
}

#[doc(hidden)]
pub struct RTDInputMessageVenueBuilder {
    inner: InputMessageVenue,
}

impl RTDInputMessageVenueBuilder {
    pub fn build(&self) -> InputMessageVenue {
        self.inner.clone()
    }

    pub fn venue<T: AsRef<Venue>>(&mut self, venue: T) -> &mut Self {
        self.inner.venue = venue.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageVenue> for InputMessageVenue {
    fn as_ref(&self) -> &InputMessageVenue {
        self
    }
}

impl AsRef<InputMessageVenue> for RTDInputMessageVenueBuilder {
    fn as_ref(&self) -> &InputMessageVenue {
        &self.inner
    }
}

/// A video message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Video to be sent

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    video: InputFile,
    /// Video thumbnail, if available
    thumbnail: InputThumbnail,
    /// File identifiers of the stickers added to the video, if applicable
    added_sticker_file_ids: Vec<i32>,
    /// Duration of the video, in seconds
    duration: i32,
    /// Video width
    width: i32,
    /// Video height
    height: i32,
    /// True, if the video should be tried to be streamed
    supports_streaming: bool,
    /// Video caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
    /// Video TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats
    ttl: i32,
}

impl RObject for InputMessageVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageVideo {}

impl InputMessageVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageVideoBuilder {
        let mut inner = InputMessageVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageVideoBuilder { inner }
    }

    pub fn video(&self) -> &InputFile {
        &self.video
    }

    pub fn thumbnail(&self) -> &InputThumbnail {
        &self.thumbnail
    }

    pub fn added_sticker_file_ids(&self) -> &Vec<i32> {
        &self.added_sticker_file_ids
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn supports_streaming(&self) -> bool {
        self.supports_streaming
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }

    pub fn ttl(&self) -> i32 {
        self.ttl
    }
}

#[doc(hidden)]
pub struct RTDInputMessageVideoBuilder {
    inner: InputMessageVideo,
}

impl RTDInputMessageVideoBuilder {
    pub fn build(&self) -> InputMessageVideo {
        self.inner.clone()
    }

    pub fn video<T: AsRef<InputFile>>(&mut self, video: T) -> &mut Self {
        self.inner.video = video.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
        self
    }

    pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i32>) -> &mut Self {
        self.inner.added_sticker_file_ids = added_sticker_file_ids;
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
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

    pub fn supports_streaming(&mut self, supports_streaming: bool) -> &mut Self {
        self.inner.supports_streaming = supports_streaming;
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }

    pub fn ttl(&mut self, ttl: i32) -> &mut Self {
        self.inner.ttl = ttl;
        self
    }
}

impl AsRef<InputMessageVideo> for InputMessageVideo {
    fn as_ref(&self) -> &InputMessageVideo {
        self
    }
}

impl AsRef<InputMessageVideo> for RTDInputMessageVideoBuilder {
    fn as_ref(&self) -> &InputMessageVideo {
        &self.inner
    }
}

/// A video note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Video note to be sent

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    video_note: InputFile,
    /// Video thumbnail, if available
    thumbnail: InputThumbnail,
    /// Duration of the video, in seconds
    duration: i32,
    /// Video width and height; must be positive and not greater than 640
    length: i32,
}

impl RObject for InputMessageVideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageVideoNote {}

impl InputMessageVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageVideoNoteBuilder {
        let mut inner = InputMessageVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageVideoNoteBuilder { inner }
    }

    pub fn video_note(&self) -> &InputFile {
        &self.video_note
    }

    pub fn thumbnail(&self) -> &InputThumbnail {
        &self.thumbnail
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn length(&self) -> i32 {
        self.length
    }
}

#[doc(hidden)]
pub struct RTDInputMessageVideoNoteBuilder {
    inner: InputMessageVideoNote,
}

impl RTDInputMessageVideoNoteBuilder {
    pub fn build(&self) -> InputMessageVideoNote {
        self.inner.clone()
    }

    pub fn video_note<T: AsRef<InputFile>>(&mut self, video_note: T) -> &mut Self {
        self.inner.video_note = video_note.as_ref().clone();
        self
    }

    pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
        self.inner.thumbnail = thumbnail.as_ref().clone();
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn length(&mut self, length: i32) -> &mut Self {
        self.inner.length = length;
        self
    }
}

impl AsRef<InputMessageVideoNote> for InputMessageVideoNote {
    fn as_ref(&self) -> &InputMessageVideoNote {
        self
    }
}

impl AsRef<InputMessageVideoNote> for RTDInputMessageVideoNoteBuilder {
    fn as_ref(&self) -> &InputMessageVideoNote {
        &self.inner
    }
}

/// A voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputMessageVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Voice note to be sent

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    voice_note: InputFile,
    /// Duration of the voice note, in seconds
    duration: i32,
    /// Waveform representation of the voice note, in 5-bit format
    waveform: String,
    /// Voice note caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
}

impl RObject for InputMessageVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDInputMessageContent for InputMessageVoiceNote {}

impl InputMessageVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDInputMessageVoiceNoteBuilder {
        let mut inner = InputMessageVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDInputMessageVoiceNoteBuilder { inner }
    }

    pub fn voice_note(&self) -> &InputFile {
        &self.voice_note
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn waveform(&self) -> &String {
        &self.waveform
    }

    pub fn caption(&self) -> &FormattedText {
        &self.caption
    }
}

#[doc(hidden)]
pub struct RTDInputMessageVoiceNoteBuilder {
    inner: InputMessageVoiceNote,
}

impl RTDInputMessageVoiceNoteBuilder {
    pub fn build(&self) -> InputMessageVoiceNote {
        self.inner.clone()
    }

    pub fn voice_note<T: AsRef<InputFile>>(&mut self, voice_note: T) -> &mut Self {
        self.inner.voice_note = voice_note.as_ref().clone();
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn waveform<T: AsRef<str>>(&mut self, waveform: T) -> &mut Self {
        self.inner.waveform = waveform.as_ref().to_string();
        self
    }

    pub fn caption<T: AsRef<FormattedText>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().clone();
        self
    }
}

impl AsRef<InputMessageVoiceNote> for InputMessageVoiceNote {
    fn as_ref(&self) -> &InputMessageVoiceNote {
        self
    }
}

impl AsRef<InputMessageVoiceNote> for RTDInputMessageVoiceNoteBuilder {
    fn as_ref(&self) -> &InputMessageVoiceNote {
        &self.inner
    }
}
