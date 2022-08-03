use crate::errors::Result;
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
    #[serde(rename = "inputMessageAnimation")]
    InputMessageAnimation(InputMessageAnimation),
    /// An audio message
    #[serde(rename = "inputMessageAudio")]
    InputMessageAudio(InputMessageAudio),
    /// A message containing a user contact
    #[serde(rename = "inputMessageContact")]
    InputMessageContact(InputMessageContact),
    /// A dice message
    #[serde(rename = "inputMessageDice")]
    InputMessageDice(InputMessageDice),
    /// A document message (general file)
    #[serde(rename = "inputMessageDocument")]
    InputMessageDocument(InputMessageDocument),
    /// A forwarded message
    #[serde(rename = "inputMessageForwarded")]
    InputMessageForwarded(InputMessageForwarded),
    /// A message with a game; not supported for channels or secret chats
    #[serde(rename = "inputMessageGame")]
    InputMessageGame(InputMessageGame),
    /// A message with an invoice; can be used only by bots
    #[serde(rename = "inputMessageInvoice")]
    InputMessageInvoice(InputMessageInvoice),
    /// A message with a location
    #[serde(rename = "inputMessageLocation")]
    InputMessageLocation(InputMessageLocation),
    /// A photo message
    #[serde(rename = "inputMessagePhoto")]
    InputMessagePhoto(InputMessagePhoto),
    /// A message with a poll. Polls can't be sent to secret chats. Polls can be sent only to a private chat with a bot
    #[serde(rename = "inputMessagePoll")]
    InputMessagePoll(InputMessagePoll),
    /// A sticker message
    #[serde(rename = "inputMessageSticker")]
    InputMessageSticker(InputMessageSticker),
    /// A text message
    #[serde(rename = "inputMessageText")]
    InputMessageText(InputMessageText),
    /// A message with information about a venue
    #[serde(rename = "inputMessageVenue")]
    InputMessageVenue(InputMessageVenue),
    /// A video message
    #[serde(rename = "inputMessageVideo")]
    InputMessageVideo(InputMessageVideo),
    /// A video note message
    #[serde(rename = "inputMessageVideoNote")]
    InputMessageVideoNote(InputMessageVideoNote),
    /// A voice note message
    #[serde(rename = "inputMessageVoiceNote")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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
    /// Animation thumbnail; pass null to skip thumbnail uploading
    thumbnail: InputThumbnail,
    /// File identifiers of the stickers added to the animation, if applicable

    #[serde(default)]
    added_sticker_file_ids: Vec<i32>,
    /// Duration of the animation, in seconds

    #[serde(default)]
    duration: i32,
    /// Width of the animation; may be replaced by the server

    #[serde(default)]
    width: i32,
    /// Height of the animation; may be replaced by the server

    #[serde(default)]
    height: i32,
    /// Animation caption; pass null to use an empty caption; 0-GetOption("message_caption_length_max") characters
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageAnimationBuilder {
        let mut inner = InputMessageAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageAnimationBuilder { inner }
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
pub struct InputMessageAnimationBuilder {
    inner: InputMessageAnimation,
}

#[deprecated]
pub type RTDInputMessageAnimationBuilder = InputMessageAnimationBuilder;

impl InputMessageAnimationBuilder {
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

impl AsRef<InputMessageAnimation> for InputMessageAnimationBuilder {
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
    /// Thumbnail of the cover for the album; pass null to skip thumbnail uploading
    album_cover_thumbnail: InputThumbnail,
    /// Duration of the audio, in seconds; may be replaced by the server

    #[serde(default)]
    duration: i32,
    /// Title of the audio; 0-64 characters; may be replaced by the server

    #[serde(default)]
    title: String,
    /// Performer of the audio; 0-64 characters, may be replaced by the server

    #[serde(default)]
    performer: String,
    /// Audio caption; pass null to use an empty caption; 0-GetOption("message_caption_length_max") characters
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageAudioBuilder {
        let mut inner = InputMessageAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageAudioBuilder { inner }
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
pub struct InputMessageAudioBuilder {
    inner: InputMessageAudio,
}

#[deprecated]
pub type RTDInputMessageAudioBuilder = InputMessageAudioBuilder;

impl InputMessageAudioBuilder {
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

impl AsRef<InputMessageAudio> for InputMessageAudioBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageContactBuilder {
        let mut inner = InputMessageContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageContactBuilder { inner }
    }

    pub fn contact(&self) -> &Contact {
        &self.contact
    }
}

#[doc(hidden)]
pub struct InputMessageContactBuilder {
    inner: InputMessageContact,
}

#[deprecated]
pub type RTDInputMessageContactBuilder = InputMessageContactBuilder;

impl InputMessageContactBuilder {
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

impl AsRef<InputMessageContact> for InputMessageContactBuilder {
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

    #[serde(default)]
    emoji: String,
    /// True, if the chat message draft must be deleted

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageDiceBuilder {
        let mut inner = InputMessageDice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageDiceBuilder { inner }
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }

    pub fn clear_draft(&self) -> bool {
        self.clear_draft
    }
}

#[doc(hidden)]
pub struct InputMessageDiceBuilder {
    inner: InputMessageDice,
}

#[deprecated]
pub type RTDInputMessageDiceBuilder = InputMessageDiceBuilder;

impl InputMessageDiceBuilder {
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

impl AsRef<InputMessageDice> for InputMessageDiceBuilder {
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
    /// Document thumbnail; pass null to skip thumbnail uploading
    thumbnail: InputThumbnail,
    /// If true, automatic file type detection will be disabled and the document will be always sent as file. Always true for files sent to secret chats

    #[serde(default)]
    disable_content_type_detection: bool,
    /// Document caption; pass null to use an empty caption; 0-GetOption("message_caption_length_max") characters
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageDocumentBuilder {
        let mut inner = InputMessageDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageDocumentBuilder { inner }
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
pub struct InputMessageDocumentBuilder {
    inner: InputMessageDocument,
}

#[deprecated]
pub type RTDInputMessageDocumentBuilder = InputMessageDocumentBuilder;

impl InputMessageDocumentBuilder {
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

impl AsRef<InputMessageDocument> for InputMessageDocumentBuilder {
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

    #[serde(default)]
    from_chat_id: i64,
    /// Identifier of the message to forward

    #[serde(default)]
    message_id: i64,
    /// True, if a game message is being shared from a launched game; applies only to game messages

    #[serde(default)]
    in_game_share: bool,
    /// Options to be used to copy content of the message without reference to the original sender; pass null to forward the message as usual
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageForwardedBuilder {
        let mut inner = InputMessageForwarded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageForwardedBuilder { inner }
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
pub struct InputMessageForwardedBuilder {
    inner: InputMessageForwarded,
}

#[deprecated]
pub type RTDInputMessageForwardedBuilder = InputMessageForwardedBuilder;

impl InputMessageForwardedBuilder {
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

impl AsRef<InputMessageForwarded> for InputMessageForwardedBuilder {
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

    #[serde(default)]
    bot_user_id: i64,
    /// Short name of the game

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageGameBuilder {
        let mut inner = InputMessageGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageGameBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn game_short_name(&self) -> &String {
        &self.game_short_name
    }
}

#[doc(hidden)]
pub struct InputMessageGameBuilder {
    inner: InputMessageGame,
}

#[deprecated]
pub type RTDInputMessageGameBuilder = InputMessageGameBuilder;

impl InputMessageGameBuilder {
    pub fn build(&self) -> InputMessageGame {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
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

impl AsRef<InputMessageGame> for InputMessageGameBuilder {
    fn as_ref(&self) -> &InputMessageGame {
        &self.inner
    }
}

/// A message with an invoice; can be used only by bots
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

    #[serde(default)]
    title: String,
    /// A message with an invoice; can be used only by bots

    #[serde(default)]
    description: String,
    /// Product photo URL; optional

    #[serde(default)]
    photo_url: String,
    /// Product photo size

    #[serde(default)]
    photo_size: i32,
    /// Product photo width

    #[serde(default)]
    photo_width: i32,
    /// Product photo height

    #[serde(default)]
    photo_height: i32,
    /// The invoice payload

    #[serde(default)]
    payload: String,
    /// Payment provider token

    #[serde(default)]
    provider_token: String,
    /// JSON-encoded data about the invoice, which will be shared with the payment provider

    #[serde(default)]
    provider_data: String,
    /// Unique invoice bot deep link parameter for the generation of this invoice. If empty, it would be possible to pay directly from forwards of the invoice message

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageInvoiceBuilder {
        let mut inner = InputMessageInvoice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageInvoiceBuilder { inner }
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
pub struct InputMessageInvoiceBuilder {
    inner: InputMessageInvoice,
}

#[deprecated]
pub type RTDInputMessageInvoiceBuilder = InputMessageInvoiceBuilder;

impl InputMessageInvoiceBuilder {
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

impl AsRef<InputMessageInvoice> for InputMessageInvoiceBuilder {
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
    /// Period for which the location can be updated, in seconds; must be between 60 and 86400 for a live location and 0 otherwise

    #[serde(default)]
    live_period: i32,
    /// For live locations, a direction in which the location moves, in degrees; 1-360. Pass 0 if unknown

    #[serde(default)]
    heading: i32,
    /// For live locations, a maximum distance to another chat member for proximity alerts, in meters (0-100000). Pass 0 if the notification is disabled. Can't be enabled in channels and Saved Messages

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageLocationBuilder {
        let mut inner = InputMessageLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageLocationBuilder { inner }
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
pub struct InputMessageLocationBuilder {
    inner: InputMessageLocation,
}

#[deprecated]
pub type RTDInputMessageLocationBuilder = InputMessageLocationBuilder;

impl InputMessageLocationBuilder {
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

impl AsRef<InputMessageLocation> for InputMessageLocationBuilder {
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
    /// Photo thumbnail to be sent; pass null to skip thumbnail uploading. The thumbnail is sent to the other party only in secret chats
    thumbnail: InputThumbnail,
    /// File identifiers of the stickers added to the photo, if applicable

    #[serde(default)]
    added_sticker_file_ids: Vec<i32>,
    /// Photo width

    #[serde(default)]
    width: i32,
    /// Photo height

    #[serde(default)]
    height: i32,
    /// Photo caption; pass null to use an empty caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
    /// Photo TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessagePhotoBuilder {
        let mut inner = InputMessagePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessagePhotoBuilder { inner }
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
pub struct InputMessagePhotoBuilder {
    inner: InputMessagePhoto,
}

#[deprecated]
pub type RTDInputMessagePhotoBuilder = InputMessagePhotoBuilder;

impl InputMessagePhotoBuilder {
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

impl AsRef<InputMessagePhoto> for InputMessagePhotoBuilder {
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
    /// Poll question; 1-255 characters (up to 300 characters for bots)

    #[serde(default)]
    question: String,
    /// List of poll answer options, 2-10 strings 1-100 characters each

    #[serde(default)]
    options: Vec<String>,
    /// True, if the poll voters are anonymous. Non-anonymous polls can't be sent or forwarded to channels

    #[serde(default)]
    is_anonymous: bool,
    /// Type of the poll

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "PollType::_is_default")]
    type_: PollType,
    /// Amount of time the poll will be active after creation, in seconds; for bots only

    #[serde(default)]
    open_period: i32,
    /// Point in time (Unix timestamp) when the poll will automatically be closed; for bots only

    #[serde(default)]
    close_date: i32,
    /// True, if the poll needs to be sent already closed; for bots only

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessagePollBuilder {
        let mut inner = InputMessagePoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessagePollBuilder { inner }
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
pub struct InputMessagePollBuilder {
    inner: InputMessagePoll,
}

#[deprecated]
pub type RTDInputMessagePollBuilder = InputMessagePollBuilder;

impl InputMessagePollBuilder {
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

impl AsRef<InputMessagePoll> for InputMessagePollBuilder {
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
    /// Sticker thumbnail; pass null to skip thumbnail uploading
    thumbnail: InputThumbnail,
    /// Sticker width

    #[serde(default)]
    width: i32,
    /// Sticker height

    #[serde(default)]
    height: i32,
    /// Emoji used to choose the sticker

    #[serde(default)]
    emoji: String,
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageStickerBuilder {
        let mut inner = InputMessageSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageStickerBuilder { inner }
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

    pub fn emoji(&self) -> &String {
        &self.emoji
    }
}

#[doc(hidden)]
pub struct InputMessageStickerBuilder {
    inner: InputMessageSticker,
}

#[deprecated]
pub type RTDInputMessageStickerBuilder = InputMessageStickerBuilder;

impl InputMessageStickerBuilder {
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

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }
}

impl AsRef<InputMessageSticker> for InputMessageSticker {
    fn as_ref(&self) -> &InputMessageSticker {
        self
    }
}

impl AsRef<InputMessageSticker> for InputMessageStickerBuilder {
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
    /// True, if rich web page previews for URLs in the message text must be disabled

    #[serde(default)]
    disable_web_page_preview: bool,
    /// True, if a chat message draft must be deleted

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageTextBuilder {
        let mut inner = InputMessageText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageTextBuilder { inner }
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
pub struct InputMessageTextBuilder {
    inner: InputMessageText,
}

#[deprecated]
pub type RTDInputMessageTextBuilder = InputMessageTextBuilder;

impl InputMessageTextBuilder {
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

impl AsRef<InputMessageText> for InputMessageTextBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageVenueBuilder {
        let mut inner = InputMessageVenue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageVenueBuilder { inner }
    }

    pub fn venue(&self) -> &Venue {
        &self.venue
    }
}

#[doc(hidden)]
pub struct InputMessageVenueBuilder {
    inner: InputMessageVenue,
}

#[deprecated]
pub type RTDInputMessageVenueBuilder = InputMessageVenueBuilder;

impl InputMessageVenueBuilder {
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

impl AsRef<InputMessageVenue> for InputMessageVenueBuilder {
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
    /// Video thumbnail; pass null to skip thumbnail uploading
    thumbnail: InputThumbnail,
    /// File identifiers of the stickers added to the video, if applicable

    #[serde(default)]
    added_sticker_file_ids: Vec<i32>,
    /// Duration of the video, in seconds

    #[serde(default)]
    duration: i32,
    /// Video width

    #[serde(default)]
    width: i32,
    /// Video height

    #[serde(default)]
    height: i32,
    /// True, if the video is supposed to be streamed

    #[serde(default)]
    supports_streaming: bool,
    /// Video caption; pass null to use an empty caption; 0-GetOption("message_caption_length_max") characters
    caption: FormattedText,
    /// Video TTL (Time To Live), in seconds (0-60). A non-zero TTL can be specified only in private chats

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageVideoBuilder {
        let mut inner = InputMessageVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageVideoBuilder { inner }
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
pub struct InputMessageVideoBuilder {
    inner: InputMessageVideo,
}

#[deprecated]
pub type RTDInputMessageVideoBuilder = InputMessageVideoBuilder;

impl InputMessageVideoBuilder {
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

impl AsRef<InputMessageVideo> for InputMessageVideoBuilder {
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
    /// Video thumbnail; pass null to skip thumbnail uploading
    thumbnail: InputThumbnail,
    /// Duration of the video, in seconds

    #[serde(default)]
    duration: i32,
    /// Video width and height; must be positive and not greater than 640

    #[serde(default)]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageVideoNoteBuilder {
        let mut inner = InputMessageVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageVideoNoteBuilder { inner }
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
pub struct InputMessageVideoNoteBuilder {
    inner: InputMessageVideoNote,
}

#[deprecated]
pub type RTDInputMessageVideoNoteBuilder = InputMessageVideoNoteBuilder;

impl InputMessageVideoNoteBuilder {
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

impl AsRef<InputMessageVideoNote> for InputMessageVideoNoteBuilder {
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

    #[serde(default)]
    duration: i32,
    /// Waveform representation of the voice note, in 5-bit format

    #[serde(default)]
    waveform: String,
    /// Voice note caption; pass null to use an empty caption; 0-GetOption("message_caption_length_max") characters
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputMessageVoiceNoteBuilder {
        let mut inner = InputMessageVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputMessageVoiceNoteBuilder { inner }
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
pub struct InputMessageVoiceNoteBuilder {
    inner: InputMessageVoiceNote,
}

#[deprecated]
pub type RTDInputMessageVoiceNoteBuilder = InputMessageVoiceNoteBuilder;

impl InputMessageVoiceNoteBuilder {
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

impl AsRef<InputMessageVoiceNote> for InputMessageVoiceNoteBuilder {
    fn as_ref(&self) -> &InputMessageVoiceNote {
        &self.inner
    }
}
