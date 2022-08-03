use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains content of a push message notification
pub trait TDPushMessageContent: Debug + RObject {}

/// Contains content of a push message notification
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PushMessageContent {
    #[doc(hidden)]
    _Default,
    /// An animation message (GIF-style).
    #[serde(rename = "pushMessageContentAnimation")]
    Animation(PushMessageContentAnimation),
    /// An audio message
    #[serde(rename = "pushMessageContentAudio")]
    Audio(PushMessageContentAudio),
    /// A newly created basic group
    #[serde(rename = "pushMessageContentBasicGroupChatCreate")]
    BasicGroupChatCreate(PushMessageContentBasicGroupChatCreate),
    /// New chat members were invited to a group
    #[serde(rename = "pushMessageContentChatAddMembers")]
    ChatAddMembers(PushMessageContentChatAddMembers),
    /// A chat photo was edited
    #[serde(rename = "pushMessageContentChatChangePhoto")]
    ChatChangePhoto(PushMessageContentChatChangePhoto),
    /// A chat title was edited
    #[serde(rename = "pushMessageContentChatChangeTitle")]
    ChatChangeTitle(PushMessageContentChatChangeTitle),
    /// A chat member was deleted
    #[serde(rename = "pushMessageContentChatDeleteMember")]
    ChatDeleteMember(PushMessageContentChatDeleteMember),
    /// A new member joined the chat via an invite link
    #[serde(rename = "pushMessageContentChatJoinByLink")]
    ChatJoinByLink(PushMessageContentChatJoinByLink),
    /// A new member was accepted to the chat by an administrator
    #[serde(rename = "pushMessageContentChatJoinByRequest")]
    ChatJoinByRequest(PushMessageContentChatJoinByRequest),
    /// A chat theme was edited
    #[serde(rename = "pushMessageContentChatSetTheme")]
    ChatSetTheme(PushMessageContentChatSetTheme),
    /// A message with a user contact
    #[serde(rename = "pushMessageContentContact")]
    Contact(PushMessageContentContact),
    /// A contact has registered with Telegram
    #[serde(rename = "pushMessageContentContactRegistered")]
    ContactRegistered(PushMessageContentContactRegistered),
    /// A document message (a general file)
    #[serde(rename = "pushMessageContentDocument")]
    Document(PushMessageContentDocument),
    /// A message with a game
    #[serde(rename = "pushMessageContentGame")]
    Game(PushMessageContentGame),
    /// A new high score was achieved in a game
    #[serde(rename = "pushMessageContentGameScore")]
    GameScore(PushMessageContentGameScore),
    /// A general message with hidden content
    #[serde(rename = "pushMessageContentHidden")]
    Hidden(PushMessageContentHidden),
    /// A message with an invoice from a bot
    #[serde(rename = "pushMessageContentInvoice")]
    Invoice(PushMessageContentInvoice),
    /// A message with a location
    #[serde(rename = "pushMessageContentLocation")]
    Location(PushMessageContentLocation),
    /// A media album
    #[serde(rename = "pushMessageContentMediaAlbum")]
    MediaAlbum(PushMessageContentMediaAlbum),
    /// A forwarded messages
    #[serde(rename = "pushMessageContentMessageForwards")]
    MessageForwards(PushMessageContentMessageForwards),
    /// A photo message
    #[serde(rename = "pushMessageContentPhoto")]
    Photo(PushMessageContentPhoto),
    /// A message with a poll
    #[serde(rename = "pushMessageContentPoll")]
    Poll(PushMessageContentPoll),
    /// A screenshot of a message in the chat has been taken
    #[serde(rename = "pushMessageContentScreenshotTaken")]
    ScreenshotTaken(PushMessageContentScreenshotTaken),
    /// A message with a sticker
    #[serde(rename = "pushMessageContentSticker")]
    Sticker(PushMessageContentSticker),
    /// A text message
    #[serde(rename = "pushMessageContentText")]
    Text(PushMessageContentText),
    /// A video message
    #[serde(rename = "pushMessageContentVideo")]
    Video(PushMessageContentVideo),
    /// A video note message
    #[serde(rename = "pushMessageContentVideoNote")]
    VideoNote(PushMessageContentVideoNote),
    /// A voice note message
    #[serde(rename = "pushMessageContentVoiceNote")]
    VoiceNote(PushMessageContentVoiceNote),
}

impl Default for PushMessageContent {
    fn default() -> Self {
        PushMessageContent::_Default
    }
}

impl RObject for PushMessageContent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PushMessageContent::Animation(t) => t.extra(),
            PushMessageContent::Audio(t) => t.extra(),
            PushMessageContent::BasicGroupChatCreate(t) => t.extra(),
            PushMessageContent::ChatAddMembers(t) => t.extra(),
            PushMessageContent::ChatChangePhoto(t) => t.extra(),
            PushMessageContent::ChatChangeTitle(t) => t.extra(),
            PushMessageContent::ChatDeleteMember(t) => t.extra(),
            PushMessageContent::ChatJoinByLink(t) => t.extra(),
            PushMessageContent::ChatJoinByRequest(t) => t.extra(),
            PushMessageContent::ChatSetTheme(t) => t.extra(),
            PushMessageContent::Contact(t) => t.extra(),
            PushMessageContent::ContactRegistered(t) => t.extra(),
            PushMessageContent::Document(t) => t.extra(),
            PushMessageContent::Game(t) => t.extra(),
            PushMessageContent::GameScore(t) => t.extra(),
            PushMessageContent::Hidden(t) => t.extra(),
            PushMessageContent::Invoice(t) => t.extra(),
            PushMessageContent::Location(t) => t.extra(),
            PushMessageContent::MediaAlbum(t) => t.extra(),
            PushMessageContent::MessageForwards(t) => t.extra(),
            PushMessageContent::Photo(t) => t.extra(),
            PushMessageContent::Poll(t) => t.extra(),
            PushMessageContent::ScreenshotTaken(t) => t.extra(),
            PushMessageContent::Sticker(t) => t.extra(),
            PushMessageContent::Text(t) => t.extra(),
            PushMessageContent::Video(t) => t.extra(),
            PushMessageContent::VideoNote(t) => t.extra(),
            PushMessageContent::VoiceNote(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PushMessageContent::Animation(t) => t.client_id(),
            PushMessageContent::Audio(t) => t.client_id(),
            PushMessageContent::BasicGroupChatCreate(t) => t.client_id(),
            PushMessageContent::ChatAddMembers(t) => t.client_id(),
            PushMessageContent::ChatChangePhoto(t) => t.client_id(),
            PushMessageContent::ChatChangeTitle(t) => t.client_id(),
            PushMessageContent::ChatDeleteMember(t) => t.client_id(),
            PushMessageContent::ChatJoinByLink(t) => t.client_id(),
            PushMessageContent::ChatJoinByRequest(t) => t.client_id(),
            PushMessageContent::ChatSetTheme(t) => t.client_id(),
            PushMessageContent::Contact(t) => t.client_id(),
            PushMessageContent::ContactRegistered(t) => t.client_id(),
            PushMessageContent::Document(t) => t.client_id(),
            PushMessageContent::Game(t) => t.client_id(),
            PushMessageContent::GameScore(t) => t.client_id(),
            PushMessageContent::Hidden(t) => t.client_id(),
            PushMessageContent::Invoice(t) => t.client_id(),
            PushMessageContent::Location(t) => t.client_id(),
            PushMessageContent::MediaAlbum(t) => t.client_id(),
            PushMessageContent::MessageForwards(t) => t.client_id(),
            PushMessageContent::Photo(t) => t.client_id(),
            PushMessageContent::Poll(t) => t.client_id(),
            PushMessageContent::ScreenshotTaken(t) => t.client_id(),
            PushMessageContent::Sticker(t) => t.client_id(),
            PushMessageContent::Text(t) => t.client_id(),
            PushMessageContent::Video(t) => t.client_id(),
            PushMessageContent::VideoNote(t) => t.client_id(),
            PushMessageContent::VoiceNote(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PushMessageContent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PushMessageContent::_Default)
    }
}

impl AsRef<PushMessageContent> for PushMessageContent {
    fn as_ref(&self) -> &PushMessageContent {
        self
    }
}

/// An animation message (GIF-style).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentAnimation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    animation: Option<Animation>,
    /// Animation caption

    #[serde(default)]
    caption: String,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentAnimation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentAnimation {}

impl PushMessageContentAnimation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentAnimationBuilder {
        let mut inner = PushMessageContentAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentAnimationBuilder { inner }
    }

    pub fn animation(&self) -> &Option<Animation> {
        &self.animation
    }

    pub fn caption(&self) -> &String {
        &self.caption
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentAnimationBuilder {
    inner: PushMessageContentAnimation,
}

#[deprecated]
pub type RTDPushMessageContentAnimationBuilder = PushMessageContentAnimationBuilder;

impl PushMessageContentAnimationBuilder {
    pub fn build(&self) -> PushMessageContentAnimation {
        self.inner.clone()
    }

    pub fn animation<T: AsRef<Animation>>(&mut self, animation: T) -> &mut Self {
        self.inner.animation = Some(animation.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<str>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().to_string();
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentAnimation> for PushMessageContentAnimation {
    fn as_ref(&self) -> &PushMessageContentAnimation {
        self
    }
}

impl AsRef<PushMessageContentAnimation> for PushMessageContentAnimationBuilder {
    fn as_ref(&self) -> &PushMessageContentAnimation {
        &self.inner
    }
}

/// An audio message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentAudio {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    audio: Option<Audio>,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentAudio {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentAudio {}

impl PushMessageContentAudio {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentAudioBuilder {
        let mut inner = PushMessageContentAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentAudioBuilder { inner }
    }

    pub fn audio(&self) -> &Option<Audio> {
        &self.audio
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentAudioBuilder {
    inner: PushMessageContentAudio,
}

#[deprecated]
pub type RTDPushMessageContentAudioBuilder = PushMessageContentAudioBuilder;

impl PushMessageContentAudioBuilder {
    pub fn build(&self) -> PushMessageContentAudio {
        self.inner.clone()
    }

    pub fn audio<T: AsRef<Audio>>(&mut self, audio: T) -> &mut Self {
        self.inner.audio = Some(audio.as_ref().clone());
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentAudio> for PushMessageContentAudio {
    fn as_ref(&self) -> &PushMessageContentAudio {
        self
    }
}

impl AsRef<PushMessageContentAudio> for PushMessageContentAudioBuilder {
    fn as_ref(&self) -> &PushMessageContentAudio {
        &self.inner
    }
}

/// A newly created basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentBasicGroupChatCreate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PushMessageContentBasicGroupChatCreate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentBasicGroupChatCreate {}

impl PushMessageContentBasicGroupChatCreate {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentBasicGroupChatCreateBuilder {
        let mut inner = PushMessageContentBasicGroupChatCreate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentBasicGroupChatCreateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PushMessageContentBasicGroupChatCreateBuilder {
    inner: PushMessageContentBasicGroupChatCreate,
}

#[deprecated]
pub type RTDPushMessageContentBasicGroupChatCreateBuilder =
    PushMessageContentBasicGroupChatCreateBuilder;

impl PushMessageContentBasicGroupChatCreateBuilder {
    pub fn build(&self) -> PushMessageContentBasicGroupChatCreate {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentBasicGroupChatCreate> for PushMessageContentBasicGroupChatCreate {
    fn as_ref(&self) -> &PushMessageContentBasicGroupChatCreate {
        self
    }
}

impl AsRef<PushMessageContentBasicGroupChatCreate>
    for PushMessageContentBasicGroupChatCreateBuilder
{
    fn as_ref(&self) -> &PushMessageContentBasicGroupChatCreate {
        &self.inner
    }
}

/// New chat members were invited to a group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatAddMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the added member

    #[serde(default)]
    member_name: String,
    /// True, if the current user was added to the group

    #[serde(default)]
    is_current_user: bool,
    /// True, if the user has returned to the group themselves

    #[serde(default)]
    is_returned: bool,
}

impl RObject for PushMessageContentChatAddMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentChatAddMembers {}

impl PushMessageContentChatAddMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentChatAddMembersBuilder {
        let mut inner = PushMessageContentChatAddMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentChatAddMembersBuilder { inner }
    }

    pub fn member_name(&self) -> &String {
        &self.member_name
    }

    pub fn is_current_user(&self) -> bool {
        self.is_current_user
    }

    pub fn is_returned(&self) -> bool {
        self.is_returned
    }
}

#[doc(hidden)]
pub struct PushMessageContentChatAddMembersBuilder {
    inner: PushMessageContentChatAddMembers,
}

#[deprecated]
pub type RTDPushMessageContentChatAddMembersBuilder = PushMessageContentChatAddMembersBuilder;

impl PushMessageContentChatAddMembersBuilder {
    pub fn build(&self) -> PushMessageContentChatAddMembers {
        self.inner.clone()
    }

    pub fn member_name<T: AsRef<str>>(&mut self, member_name: T) -> &mut Self {
        self.inner.member_name = member_name.as_ref().to_string();
        self
    }

    pub fn is_current_user(&mut self, is_current_user: bool) -> &mut Self {
        self.inner.is_current_user = is_current_user;
        self
    }

    pub fn is_returned(&mut self, is_returned: bool) -> &mut Self {
        self.inner.is_returned = is_returned;
        self
    }
}

impl AsRef<PushMessageContentChatAddMembers> for PushMessageContentChatAddMembers {
    fn as_ref(&self) -> &PushMessageContentChatAddMembers {
        self
    }
}

impl AsRef<PushMessageContentChatAddMembers> for PushMessageContentChatAddMembersBuilder {
    fn as_ref(&self) -> &PushMessageContentChatAddMembers {
        &self.inner
    }
}

/// A chat photo was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatChangePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PushMessageContentChatChangePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentChatChangePhoto {}

impl PushMessageContentChatChangePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentChatChangePhotoBuilder {
        let mut inner = PushMessageContentChatChangePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentChatChangePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PushMessageContentChatChangePhotoBuilder {
    inner: PushMessageContentChatChangePhoto,
}

#[deprecated]
pub type RTDPushMessageContentChatChangePhotoBuilder = PushMessageContentChatChangePhotoBuilder;

impl PushMessageContentChatChangePhotoBuilder {
    pub fn build(&self) -> PushMessageContentChatChangePhoto {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentChatChangePhoto> for PushMessageContentChatChangePhoto {
    fn as_ref(&self) -> &PushMessageContentChatChangePhoto {
        self
    }
}

impl AsRef<PushMessageContentChatChangePhoto> for PushMessageContentChatChangePhotoBuilder {
    fn as_ref(&self) -> &PushMessageContentChatChangePhoto {
        &self.inner
    }
}

/// A chat title was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatChangeTitle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New chat title

    #[serde(default)]
    title: String,
}

impl RObject for PushMessageContentChatChangeTitle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentChatChangeTitle {}

impl PushMessageContentChatChangeTitle {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentChatChangeTitleBuilder {
        let mut inner = PushMessageContentChatChangeTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentChatChangeTitleBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct PushMessageContentChatChangeTitleBuilder {
    inner: PushMessageContentChatChangeTitle,
}

#[deprecated]
pub type RTDPushMessageContentChatChangeTitleBuilder = PushMessageContentChatChangeTitleBuilder;

impl PushMessageContentChatChangeTitleBuilder {
    pub fn build(&self) -> PushMessageContentChatChangeTitle {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<PushMessageContentChatChangeTitle> for PushMessageContentChatChangeTitle {
    fn as_ref(&self) -> &PushMessageContentChatChangeTitle {
        self
    }
}

impl AsRef<PushMessageContentChatChangeTitle> for PushMessageContentChatChangeTitleBuilder {
    fn as_ref(&self) -> &PushMessageContentChatChangeTitle {
        &self.inner
    }
}

/// A chat member was deleted
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatDeleteMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Name of the deleted member

    #[serde(default)]
    member_name: String,
    /// True, if the current user was deleted from the group

    #[serde(default)]
    is_current_user: bool,
    /// True, if the user has left the group themselves

    #[serde(default)]
    is_left: bool,
}

impl RObject for PushMessageContentChatDeleteMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentChatDeleteMember {}

impl PushMessageContentChatDeleteMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentChatDeleteMemberBuilder {
        let mut inner = PushMessageContentChatDeleteMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentChatDeleteMemberBuilder { inner }
    }

    pub fn member_name(&self) -> &String {
        &self.member_name
    }

    pub fn is_current_user(&self) -> bool {
        self.is_current_user
    }

    pub fn is_left(&self) -> bool {
        self.is_left
    }
}

#[doc(hidden)]
pub struct PushMessageContentChatDeleteMemberBuilder {
    inner: PushMessageContentChatDeleteMember,
}

#[deprecated]
pub type RTDPushMessageContentChatDeleteMemberBuilder = PushMessageContentChatDeleteMemberBuilder;

impl PushMessageContentChatDeleteMemberBuilder {
    pub fn build(&self) -> PushMessageContentChatDeleteMember {
        self.inner.clone()
    }

    pub fn member_name<T: AsRef<str>>(&mut self, member_name: T) -> &mut Self {
        self.inner.member_name = member_name.as_ref().to_string();
        self
    }

    pub fn is_current_user(&mut self, is_current_user: bool) -> &mut Self {
        self.inner.is_current_user = is_current_user;
        self
    }

    pub fn is_left(&mut self, is_left: bool) -> &mut Self {
        self.inner.is_left = is_left;
        self
    }
}

impl AsRef<PushMessageContentChatDeleteMember> for PushMessageContentChatDeleteMember {
    fn as_ref(&self) -> &PushMessageContentChatDeleteMember {
        self
    }
}

impl AsRef<PushMessageContentChatDeleteMember> for PushMessageContentChatDeleteMemberBuilder {
    fn as_ref(&self) -> &PushMessageContentChatDeleteMember {
        &self.inner
    }
}

/// A new member joined the chat via an invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatJoinByLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PushMessageContentChatJoinByLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentChatJoinByLink {}

impl PushMessageContentChatJoinByLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentChatJoinByLinkBuilder {
        let mut inner = PushMessageContentChatJoinByLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentChatJoinByLinkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PushMessageContentChatJoinByLinkBuilder {
    inner: PushMessageContentChatJoinByLink,
}

#[deprecated]
pub type RTDPushMessageContentChatJoinByLinkBuilder = PushMessageContentChatJoinByLinkBuilder;

impl PushMessageContentChatJoinByLinkBuilder {
    pub fn build(&self) -> PushMessageContentChatJoinByLink {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentChatJoinByLink> for PushMessageContentChatJoinByLink {
    fn as_ref(&self) -> &PushMessageContentChatJoinByLink {
        self
    }
}

impl AsRef<PushMessageContentChatJoinByLink> for PushMessageContentChatJoinByLinkBuilder {
    fn as_ref(&self) -> &PushMessageContentChatJoinByLink {
        &self.inner
    }
}

/// A new member was accepted to the chat by an administrator
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatJoinByRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PushMessageContentChatJoinByRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentChatJoinByRequest {}

impl PushMessageContentChatJoinByRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentChatJoinByRequestBuilder {
        let mut inner = PushMessageContentChatJoinByRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentChatJoinByRequestBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PushMessageContentChatJoinByRequestBuilder {
    inner: PushMessageContentChatJoinByRequest,
}

#[deprecated]
pub type RTDPushMessageContentChatJoinByRequestBuilder = PushMessageContentChatJoinByRequestBuilder;

impl PushMessageContentChatJoinByRequestBuilder {
    pub fn build(&self) -> PushMessageContentChatJoinByRequest {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentChatJoinByRequest> for PushMessageContentChatJoinByRequest {
    fn as_ref(&self) -> &PushMessageContentChatJoinByRequest {
        self
    }
}

impl AsRef<PushMessageContentChatJoinByRequest> for PushMessageContentChatJoinByRequestBuilder {
    fn as_ref(&self) -> &PushMessageContentChatJoinByRequest {
        &self.inner
    }
}

/// A chat theme was edited
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentChatSetTheme {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If non-empty, name of a new theme, set for the chat. Otherwise chat theme was reset to the default one

    #[serde(default)]
    theme_name: String,
}

impl RObject for PushMessageContentChatSetTheme {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentChatSetTheme {}

impl PushMessageContentChatSetTheme {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentChatSetThemeBuilder {
        let mut inner = PushMessageContentChatSetTheme::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentChatSetThemeBuilder { inner }
    }

    pub fn theme_name(&self) -> &String {
        &self.theme_name
    }
}

#[doc(hidden)]
pub struct PushMessageContentChatSetThemeBuilder {
    inner: PushMessageContentChatSetTheme,
}

#[deprecated]
pub type RTDPushMessageContentChatSetThemeBuilder = PushMessageContentChatSetThemeBuilder;

impl PushMessageContentChatSetThemeBuilder {
    pub fn build(&self) -> PushMessageContentChatSetTheme {
        self.inner.clone()
    }

    pub fn theme_name<T: AsRef<str>>(&mut self, theme_name: T) -> &mut Self {
        self.inner.theme_name = theme_name.as_ref().to_string();
        self
    }
}

impl AsRef<PushMessageContentChatSetTheme> for PushMessageContentChatSetTheme {
    fn as_ref(&self) -> &PushMessageContentChatSetTheme {
        self
    }
}

impl AsRef<PushMessageContentChatSetTheme> for PushMessageContentChatSetThemeBuilder {
    fn as_ref(&self) -> &PushMessageContentChatSetTheme {
        &self.inner
    }
}

/// A message with a user contact
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentContact {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Contact's name

    #[serde(default)]
    name: String,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentContact {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentContact {}

impl PushMessageContentContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentContactBuilder {
        let mut inner = PushMessageContentContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentContactBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentContactBuilder {
    inner: PushMessageContentContact,
}

#[deprecated]
pub type RTDPushMessageContentContactBuilder = PushMessageContentContactBuilder;

impl PushMessageContentContactBuilder {
    pub fn build(&self) -> PushMessageContentContact {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentContact> for PushMessageContentContact {
    fn as_ref(&self) -> &PushMessageContentContact {
        self
    }
}

impl AsRef<PushMessageContentContact> for PushMessageContentContactBuilder {
    fn as_ref(&self) -> &PushMessageContentContact {
        &self.inner
    }
}

/// A contact has registered with Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentContactRegistered {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PushMessageContentContactRegistered {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentContactRegistered {}

impl PushMessageContentContactRegistered {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentContactRegisteredBuilder {
        let mut inner = PushMessageContentContactRegistered::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentContactRegisteredBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PushMessageContentContactRegisteredBuilder {
    inner: PushMessageContentContactRegistered,
}

#[deprecated]
pub type RTDPushMessageContentContactRegisteredBuilder = PushMessageContentContactRegisteredBuilder;

impl PushMessageContentContactRegisteredBuilder {
    pub fn build(&self) -> PushMessageContentContactRegistered {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentContactRegistered> for PushMessageContentContactRegistered {
    fn as_ref(&self) -> &PushMessageContentContactRegistered {
        self
    }
}

impl AsRef<PushMessageContentContactRegistered> for PushMessageContentContactRegisteredBuilder {
    fn as_ref(&self) -> &PushMessageContentContactRegistered {
        &self.inner
    }
}

/// A document message (a general file)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentDocument {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    document: Option<Document>,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentDocument {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentDocument {}

impl PushMessageContentDocument {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentDocumentBuilder {
        let mut inner = PushMessageContentDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentDocumentBuilder { inner }
    }

    pub fn document(&self) -> &Option<Document> {
        &self.document
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentDocumentBuilder {
    inner: PushMessageContentDocument,
}

#[deprecated]
pub type RTDPushMessageContentDocumentBuilder = PushMessageContentDocumentBuilder;

impl PushMessageContentDocumentBuilder {
    pub fn build(&self) -> PushMessageContentDocument {
        self.inner.clone()
    }

    pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
        self.inner.document = Some(document.as_ref().clone());
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentDocument> for PushMessageContentDocument {
    fn as_ref(&self) -> &PushMessageContentDocument {
        self
    }
}

impl AsRef<PushMessageContentDocument> for PushMessageContentDocumentBuilder {
    fn as_ref(&self) -> &PushMessageContentDocument {
        &self.inner
    }
}

/// A message with a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentGame {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Game title, empty for pinned game message

    #[serde(default)]
    title: String,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentGame {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentGame {}

impl PushMessageContentGame {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentGameBuilder {
        let mut inner = PushMessageContentGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentGameBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentGameBuilder {
    inner: PushMessageContentGame,
}

#[deprecated]
pub type RTDPushMessageContentGameBuilder = PushMessageContentGameBuilder;

impl PushMessageContentGameBuilder {
    pub fn build(&self) -> PushMessageContentGame {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentGame> for PushMessageContentGame {
    fn as_ref(&self) -> &PushMessageContentGame {
        self
    }
}

impl AsRef<PushMessageContentGame> for PushMessageContentGameBuilder {
    fn as_ref(&self) -> &PushMessageContentGame {
        &self.inner
    }
}

/// A new high score was achieved in a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentGameScore {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Game title, empty for pinned message

    #[serde(default)]
    title: String,
    /// New score, 0 for pinned message

    #[serde(default)]
    score: i32,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentGameScore {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentGameScore {}

impl PushMessageContentGameScore {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentGameScoreBuilder {
        let mut inner = PushMessageContentGameScore::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentGameScoreBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentGameScoreBuilder {
    inner: PushMessageContentGameScore,
}

#[deprecated]
pub type RTDPushMessageContentGameScoreBuilder = PushMessageContentGameScoreBuilder;

impl PushMessageContentGameScoreBuilder {
    pub fn build(&self) -> PushMessageContentGameScore {
        self.inner.clone()
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn score(&mut self, score: i32) -> &mut Self {
        self.inner.score = score;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentGameScore> for PushMessageContentGameScore {
    fn as_ref(&self) -> &PushMessageContentGameScore {
        self
    }
}

impl AsRef<PushMessageContentGameScore> for PushMessageContentGameScoreBuilder {
    fn as_ref(&self) -> &PushMessageContentGameScore {
        &self.inner
    }
}

/// A general message with hidden content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentHidden {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentHidden {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentHidden {}

impl PushMessageContentHidden {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentHiddenBuilder {
        let mut inner = PushMessageContentHidden::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentHiddenBuilder { inner }
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentHiddenBuilder {
    inner: PushMessageContentHidden,
}

#[deprecated]
pub type RTDPushMessageContentHiddenBuilder = PushMessageContentHiddenBuilder;

impl PushMessageContentHiddenBuilder {
    pub fn build(&self) -> PushMessageContentHidden {
        self.inner.clone()
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentHidden> for PushMessageContentHidden {
    fn as_ref(&self) -> &PushMessageContentHidden {
        self
    }
}

impl AsRef<PushMessageContentHidden> for PushMessageContentHiddenBuilder {
    fn as_ref(&self) -> &PushMessageContentHidden {
        &self.inner
    }
}

/// A message with an invoice from a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentInvoice {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Product price

    #[serde(default)]
    price: String,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentInvoice {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentInvoice {}

impl PushMessageContentInvoice {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentInvoiceBuilder {
        let mut inner = PushMessageContentInvoice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentInvoiceBuilder { inner }
    }

    pub fn price(&self) -> &String {
        &self.price
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentInvoiceBuilder {
    inner: PushMessageContentInvoice,
}

#[deprecated]
pub type RTDPushMessageContentInvoiceBuilder = PushMessageContentInvoiceBuilder;

impl PushMessageContentInvoiceBuilder {
    pub fn build(&self) -> PushMessageContentInvoice {
        self.inner.clone()
    }

    pub fn price<T: AsRef<str>>(&mut self, price: T) -> &mut Self {
        self.inner.price = price.as_ref().to_string();
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentInvoice> for PushMessageContentInvoice {
    fn as_ref(&self) -> &PushMessageContentInvoice {
        self
    }
}

impl AsRef<PushMessageContentInvoice> for PushMessageContentInvoiceBuilder {
    fn as_ref(&self) -> &PushMessageContentInvoice {
        &self.inner
    }
}

/// A message with a location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the location is live

    #[serde(default)]
    is_live: bool,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentLocation {}

impl PushMessageContentLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentLocationBuilder {
        let mut inner = PushMessageContentLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentLocationBuilder { inner }
    }

    pub fn is_live(&self) -> bool {
        self.is_live
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentLocationBuilder {
    inner: PushMessageContentLocation,
}

#[deprecated]
pub type RTDPushMessageContentLocationBuilder = PushMessageContentLocationBuilder;

impl PushMessageContentLocationBuilder {
    pub fn build(&self) -> PushMessageContentLocation {
        self.inner.clone()
    }

    pub fn is_live(&mut self, is_live: bool) -> &mut Self {
        self.inner.is_live = is_live;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentLocation> for PushMessageContentLocation {
    fn as_ref(&self) -> &PushMessageContentLocation {
        self
    }
}

impl AsRef<PushMessageContentLocation> for PushMessageContentLocationBuilder {
    fn as_ref(&self) -> &PushMessageContentLocation {
        &self.inner
    }
}

/// A media album
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentMediaAlbum {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of messages in the album

    #[serde(default)]
    total_count: i32,
    /// True, if the album has at least one photo

    #[serde(default)]
    has_photos: bool,
    /// True, if the album has at least one video

    #[serde(default)]
    has_videos: bool,
    /// True, if the album has at least one audio file

    #[serde(default)]
    has_audios: bool,
    /// True, if the album has at least one document

    #[serde(default)]
    has_documents: bool,
}

impl RObject for PushMessageContentMediaAlbum {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentMediaAlbum {}

impl PushMessageContentMediaAlbum {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentMediaAlbumBuilder {
        let mut inner = PushMessageContentMediaAlbum::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentMediaAlbumBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }

    pub fn has_photos(&self) -> bool {
        self.has_photos
    }

    pub fn has_videos(&self) -> bool {
        self.has_videos
    }

    pub fn has_audios(&self) -> bool {
        self.has_audios
    }

    pub fn has_documents(&self) -> bool {
        self.has_documents
    }
}

#[doc(hidden)]
pub struct PushMessageContentMediaAlbumBuilder {
    inner: PushMessageContentMediaAlbum,
}

#[deprecated]
pub type RTDPushMessageContentMediaAlbumBuilder = PushMessageContentMediaAlbumBuilder;

impl PushMessageContentMediaAlbumBuilder {
    pub fn build(&self) -> PushMessageContentMediaAlbum {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }

    pub fn has_photos(&mut self, has_photos: bool) -> &mut Self {
        self.inner.has_photos = has_photos;
        self
    }

    pub fn has_videos(&mut self, has_videos: bool) -> &mut Self {
        self.inner.has_videos = has_videos;
        self
    }

    pub fn has_audios(&mut self, has_audios: bool) -> &mut Self {
        self.inner.has_audios = has_audios;
        self
    }

    pub fn has_documents(&mut self, has_documents: bool) -> &mut Self {
        self.inner.has_documents = has_documents;
        self
    }
}

impl AsRef<PushMessageContentMediaAlbum> for PushMessageContentMediaAlbum {
    fn as_ref(&self) -> &PushMessageContentMediaAlbum {
        self
    }
}

impl AsRef<PushMessageContentMediaAlbum> for PushMessageContentMediaAlbumBuilder {
    fn as_ref(&self) -> &PushMessageContentMediaAlbum {
        &self.inner
    }
}

/// A forwarded messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentMessageForwards {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of forwarded messages

    #[serde(default)]
    total_count: i32,
}

impl RObject for PushMessageContentMessageForwards {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentMessageForwards {}

impl PushMessageContentMessageForwards {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentMessageForwardsBuilder {
        let mut inner = PushMessageContentMessageForwards::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentMessageForwardsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }
}

#[doc(hidden)]
pub struct PushMessageContentMessageForwardsBuilder {
    inner: PushMessageContentMessageForwards,
}

#[deprecated]
pub type RTDPushMessageContentMessageForwardsBuilder = PushMessageContentMessageForwardsBuilder;

impl PushMessageContentMessageForwardsBuilder {
    pub fn build(&self) -> PushMessageContentMessageForwards {
        self.inner.clone()
    }

    pub fn total_count(&mut self, total_count: i32) -> &mut Self {
        self.inner.total_count = total_count;
        self
    }
}

impl AsRef<PushMessageContentMessageForwards> for PushMessageContentMessageForwards {
    fn as_ref(&self) -> &PushMessageContentMessageForwards {
        self
    }
}

impl AsRef<PushMessageContentMessageForwards> for PushMessageContentMessageForwardsBuilder {
    fn as_ref(&self) -> &PushMessageContentMessageForwards {
        &self.inner
    }
}

/// A photo message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    photo: Option<Photo>,
    /// Photo caption

    #[serde(default)]
    caption: String,
    /// True, if the photo is secret

    #[serde(default)]
    is_secret: bool,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentPhoto {}

impl PushMessageContentPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentPhotoBuilder {
        let mut inner = PushMessageContentPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentPhotoBuilder { inner }
    }

    pub fn photo(&self) -> &Option<Photo> {
        &self.photo
    }

    pub fn caption(&self) -> &String {
        &self.caption
    }

    pub fn is_secret(&self) -> bool {
        self.is_secret
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentPhotoBuilder {
    inner: PushMessageContentPhoto,
}

#[deprecated]
pub type RTDPushMessageContentPhotoBuilder = PushMessageContentPhotoBuilder;

impl PushMessageContentPhotoBuilder {
    pub fn build(&self) -> PushMessageContentPhoto {
        self.inner.clone()
    }

    pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<str>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().to_string();
        self
    }

    pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
        self.inner.is_secret = is_secret;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentPhoto> for PushMessageContentPhoto {
    fn as_ref(&self) -> &PushMessageContentPhoto {
        self
    }
}

impl AsRef<PushMessageContentPhoto> for PushMessageContentPhotoBuilder {
    fn as_ref(&self) -> &PushMessageContentPhoto {
        &self.inner
    }
}

/// A message with a poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentPoll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Poll question

    #[serde(default)]
    question: String,
    /// True, if the poll is regular and not in quiz mode

    #[serde(default)]
    is_regular: bool,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentPoll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentPoll {}

impl PushMessageContentPoll {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentPollBuilder {
        let mut inner = PushMessageContentPoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentPollBuilder { inner }
    }

    pub fn question(&self) -> &String {
        &self.question
    }

    pub fn is_regular(&self) -> bool {
        self.is_regular
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentPollBuilder {
    inner: PushMessageContentPoll,
}

#[deprecated]
pub type RTDPushMessageContentPollBuilder = PushMessageContentPollBuilder;

impl PushMessageContentPollBuilder {
    pub fn build(&self) -> PushMessageContentPoll {
        self.inner.clone()
    }

    pub fn question<T: AsRef<str>>(&mut self, question: T) -> &mut Self {
        self.inner.question = question.as_ref().to_string();
        self
    }

    pub fn is_regular(&mut self, is_regular: bool) -> &mut Self {
        self.inner.is_regular = is_regular;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentPoll> for PushMessageContentPoll {
    fn as_ref(&self) -> &PushMessageContentPoll {
        self
    }
}

impl AsRef<PushMessageContentPoll> for PushMessageContentPollBuilder {
    fn as_ref(&self) -> &PushMessageContentPoll {
        &self.inner
    }
}

/// A screenshot of a message in the chat has been taken
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentScreenshotTaken {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PushMessageContentScreenshotTaken {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentScreenshotTaken {}

impl PushMessageContentScreenshotTaken {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentScreenshotTakenBuilder {
        let mut inner = PushMessageContentScreenshotTaken::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentScreenshotTakenBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PushMessageContentScreenshotTakenBuilder {
    inner: PushMessageContentScreenshotTaken,
}

#[deprecated]
pub type RTDPushMessageContentScreenshotTakenBuilder = PushMessageContentScreenshotTakenBuilder;

impl PushMessageContentScreenshotTakenBuilder {
    pub fn build(&self) -> PushMessageContentScreenshotTaken {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentScreenshotTaken> for PushMessageContentScreenshotTaken {
    fn as_ref(&self) -> &PushMessageContentScreenshotTaken {
        self
    }
}

impl AsRef<PushMessageContentScreenshotTaken> for PushMessageContentScreenshotTakenBuilder {
    fn as_ref(&self) -> &PushMessageContentScreenshotTaken {
        &self.inner
    }
}

/// A message with a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentSticker {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    sticker: Option<Sticker>,
    /// Emoji corresponding to the sticker; may be empty

    #[serde(default)]
    emoji: String,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentSticker {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentSticker {}

impl PushMessageContentSticker {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentStickerBuilder {
        let mut inner = PushMessageContentSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentStickerBuilder { inner }
    }

    pub fn sticker(&self) -> &Option<Sticker> {
        &self.sticker
    }

    pub fn emoji(&self) -> &String {
        &self.emoji
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentStickerBuilder {
    inner: PushMessageContentSticker,
}

#[deprecated]
pub type RTDPushMessageContentStickerBuilder = PushMessageContentStickerBuilder;

impl PushMessageContentStickerBuilder {
    pub fn build(&self) -> PushMessageContentSticker {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = Some(sticker.as_ref().clone());
        self
    }

    pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
        self.inner.emoji = emoji.as_ref().to_string();
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentSticker> for PushMessageContentSticker {
    fn as_ref(&self) -> &PushMessageContentSticker {
        self
    }
}

impl AsRef<PushMessageContentSticker> for PushMessageContentStickerBuilder {
    fn as_ref(&self) -> &PushMessageContentSticker {
        &self.inner
    }
}

/// A text message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message text

    #[serde(default)]
    text: String,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentText {}

impl PushMessageContentText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentTextBuilder {
        let mut inner = PushMessageContentText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentTextBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentTextBuilder {
    inner: PushMessageContentText,
}

#[deprecated]
pub type RTDPushMessageContentTextBuilder = PushMessageContentTextBuilder;

impl PushMessageContentTextBuilder {
    pub fn build(&self) -> PushMessageContentText {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentText> for PushMessageContentText {
    fn as_ref(&self) -> &PushMessageContentText {
        self
    }
}

impl AsRef<PushMessageContentText> for PushMessageContentTextBuilder {
    fn as_ref(&self) -> &PushMessageContentText {
        &self.inner
    }
}

/// A video message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVideo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    video: Option<Video>,
    /// Video caption

    #[serde(default)]
    caption: String,
    /// True, if the video is secret

    #[serde(default)]
    is_secret: bool,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentVideo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentVideo {}

impl PushMessageContentVideo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentVideoBuilder {
        let mut inner = PushMessageContentVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentVideoBuilder { inner }
    }

    pub fn video(&self) -> &Option<Video> {
        &self.video
    }

    pub fn caption(&self) -> &String {
        &self.caption
    }

    pub fn is_secret(&self) -> bool {
        self.is_secret
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentVideoBuilder {
    inner: PushMessageContentVideo,
}

#[deprecated]
pub type RTDPushMessageContentVideoBuilder = PushMessageContentVideoBuilder;

impl PushMessageContentVideoBuilder {
    pub fn build(&self) -> PushMessageContentVideo {
        self.inner.clone()
    }

    pub fn video<T: AsRef<Video>>(&mut self, video: T) -> &mut Self {
        self.inner.video = Some(video.as_ref().clone());
        self
    }

    pub fn caption<T: AsRef<str>>(&mut self, caption: T) -> &mut Self {
        self.inner.caption = caption.as_ref().to_string();
        self
    }

    pub fn is_secret(&mut self, is_secret: bool) -> &mut Self {
        self.inner.is_secret = is_secret;
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentVideo> for PushMessageContentVideo {
    fn as_ref(&self) -> &PushMessageContentVideo {
        self
    }
}

impl AsRef<PushMessageContentVideo> for PushMessageContentVideoBuilder {
    fn as_ref(&self) -> &PushMessageContentVideo {
        &self.inner
    }
}

/// A video note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVideoNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    video_note: Option<VideoNote>,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentVideoNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentVideoNote {}

impl PushMessageContentVideoNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentVideoNoteBuilder {
        let mut inner = PushMessageContentVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentVideoNoteBuilder { inner }
    }

    pub fn video_note(&self) -> &Option<VideoNote> {
        &self.video_note
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentVideoNoteBuilder {
    inner: PushMessageContentVideoNote,
}

#[deprecated]
pub type RTDPushMessageContentVideoNoteBuilder = PushMessageContentVideoNoteBuilder;

impl PushMessageContentVideoNoteBuilder {
    pub fn build(&self) -> PushMessageContentVideoNote {
        self.inner.clone()
    }

    pub fn video_note<T: AsRef<VideoNote>>(&mut self, video_note: T) -> &mut Self {
        self.inner.video_note = Some(video_note.as_ref().clone());
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentVideoNote> for PushMessageContentVideoNote {
    fn as_ref(&self) -> &PushMessageContentVideoNote {
        self
    }
}

impl AsRef<PushMessageContentVideoNote> for PushMessageContentVideoNoteBuilder {
    fn as_ref(&self) -> &PushMessageContentVideoNote {
        &self.inner
    }
}

/// A voice note message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PushMessageContentVoiceNote {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Message content; may be null
    voice_note: Option<VoiceNote>,
    /// True, if the message is a pinned message with the specified content

    #[serde(default)]
    is_pinned: bool,
}

impl RObject for PushMessageContentVoiceNote {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPushMessageContent for PushMessageContentVoiceNote {}

impl PushMessageContentVoiceNote {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PushMessageContentVoiceNoteBuilder {
        let mut inner = PushMessageContentVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PushMessageContentVoiceNoteBuilder { inner }
    }

    pub fn voice_note(&self) -> &Option<VoiceNote> {
        &self.voice_note
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct PushMessageContentVoiceNoteBuilder {
    inner: PushMessageContentVoiceNote,
}

#[deprecated]
pub type RTDPushMessageContentVoiceNoteBuilder = PushMessageContentVoiceNoteBuilder;

impl PushMessageContentVoiceNoteBuilder {
    pub fn build(&self) -> PushMessageContentVoiceNote {
        self.inner.clone()
    }

    pub fn voice_note<T: AsRef<VoiceNote>>(&mut self, voice_note: T) -> &mut Self {
        self.inner.voice_note = Some(voice_note.as_ref().clone());
        self
    }

    pub fn is_pinned(&mut self, is_pinned: bool) -> &mut Self {
        self.inner.is_pinned = is_pinned;
        self
    }
}

impl AsRef<PushMessageContentVoiceNote> for PushMessageContentVoiceNote {
    fn as_ref(&self) -> &PushMessageContentVoiceNote {
        self
    }
}

impl AsRef<PushMessageContentVoiceNote> for PushMessageContentVoiceNoteBuilder {
    fn as_ref(&self) -> &PushMessageContentVoiceNote {
        &self.inner
    }
}
