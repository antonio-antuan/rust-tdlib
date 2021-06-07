use crate::errors::*;
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
    #[serde(rename(
        serialize = "pushMessageContentAnimation",
        deserialize = "pushMessageContentAnimation"
    ))]
    Animation(PushMessageContentAnimation),
    /// An audio message
    #[serde(rename(
        serialize = "pushMessageContentAudio",
        deserialize = "pushMessageContentAudio"
    ))]
    Audio(PushMessageContentAudio),
    /// A newly created basic group
    #[serde(rename(
        serialize = "pushMessageContentBasicGroupChatCreate",
        deserialize = "pushMessageContentBasicGroupChatCreate"
    ))]
    BasicGroupChatCreate(PushMessageContentBasicGroupChatCreate),
    /// New chat members were invited to a group
    #[serde(rename(
        serialize = "pushMessageContentChatAddMembers",
        deserialize = "pushMessageContentChatAddMembers"
    ))]
    ChatAddMembers(PushMessageContentChatAddMembers),
    /// A chat photo was edited
    #[serde(rename(
        serialize = "pushMessageContentChatChangePhoto",
        deserialize = "pushMessageContentChatChangePhoto"
    ))]
    ChatChangePhoto(PushMessageContentChatChangePhoto),
    /// A chat title was edited
    #[serde(rename(
        serialize = "pushMessageContentChatChangeTitle",
        deserialize = "pushMessageContentChatChangeTitle"
    ))]
    ChatChangeTitle(PushMessageContentChatChangeTitle),
    /// A chat member was deleted
    #[serde(rename(
        serialize = "pushMessageContentChatDeleteMember",
        deserialize = "pushMessageContentChatDeleteMember"
    ))]
    ChatDeleteMember(PushMessageContentChatDeleteMember),
    /// A new member joined the chat by invite link
    #[serde(rename(
        serialize = "pushMessageContentChatJoinByLink",
        deserialize = "pushMessageContentChatJoinByLink"
    ))]
    ChatJoinByLink(PushMessageContentChatJoinByLink),
    /// A message with a user contact
    #[serde(rename(
        serialize = "pushMessageContentContact",
        deserialize = "pushMessageContentContact"
    ))]
    Contact(PushMessageContentContact),
    /// A contact has registered with Telegram
    #[serde(rename(
        serialize = "pushMessageContentContactRegistered",
        deserialize = "pushMessageContentContactRegistered"
    ))]
    ContactRegistered(PushMessageContentContactRegistered),
    /// A document message (a general file)
    #[serde(rename(
        serialize = "pushMessageContentDocument",
        deserialize = "pushMessageContentDocument"
    ))]
    Document(PushMessageContentDocument),
    /// A message with a game
    #[serde(rename(
        serialize = "pushMessageContentGame",
        deserialize = "pushMessageContentGame"
    ))]
    Game(PushMessageContentGame),
    /// A new high score was achieved in a game
    #[serde(rename(
        serialize = "pushMessageContentGameScore",
        deserialize = "pushMessageContentGameScore"
    ))]
    GameScore(PushMessageContentGameScore),
    /// A general message with hidden content
    #[serde(rename(
        serialize = "pushMessageContentHidden",
        deserialize = "pushMessageContentHidden"
    ))]
    Hidden(PushMessageContentHidden),
    /// A message with an invoice from a bot
    #[serde(rename(
        serialize = "pushMessageContentInvoice",
        deserialize = "pushMessageContentInvoice"
    ))]
    Invoice(PushMessageContentInvoice),
    /// A message with a location
    #[serde(rename(
        serialize = "pushMessageContentLocation",
        deserialize = "pushMessageContentLocation"
    ))]
    Location(PushMessageContentLocation),
    /// A media album
    #[serde(rename(
        serialize = "pushMessageContentMediaAlbum",
        deserialize = "pushMessageContentMediaAlbum"
    ))]
    MediaAlbum(PushMessageContentMediaAlbum),
    /// A forwarded messages
    #[serde(rename(
        serialize = "pushMessageContentMessageForwards",
        deserialize = "pushMessageContentMessageForwards"
    ))]
    MessageForwards(PushMessageContentMessageForwards),
    /// A photo message
    #[serde(rename(
        serialize = "pushMessageContentPhoto",
        deserialize = "pushMessageContentPhoto"
    ))]
    Photo(PushMessageContentPhoto),
    /// A message with a poll
    #[serde(rename(
        serialize = "pushMessageContentPoll",
        deserialize = "pushMessageContentPoll"
    ))]
    Poll(PushMessageContentPoll),
    /// A screenshot of a message in the chat has been taken
    #[serde(rename(
        serialize = "pushMessageContentScreenshotTaken",
        deserialize = "pushMessageContentScreenshotTaken"
    ))]
    ScreenshotTaken(PushMessageContentScreenshotTaken),
    /// A message with a sticker
    #[serde(rename(
        serialize = "pushMessageContentSticker",
        deserialize = "pushMessageContentSticker"
    ))]
    Sticker(PushMessageContentSticker),
    /// A text message
    #[serde(rename(
        serialize = "pushMessageContentText",
        deserialize = "pushMessageContentText"
    ))]
    Text(PushMessageContentText),
    /// A video message
    #[serde(rename(
        serialize = "pushMessageContentVideo",
        deserialize = "pushMessageContentVideo"
    ))]
    Video(PushMessageContentVideo),
    /// A video note message
    #[serde(rename(
        serialize = "pushMessageContentVideoNote",
        deserialize = "pushMessageContentVideoNote"
    ))]
    VideoNote(PushMessageContentVideoNote),
    /// A voice note message
    #[serde(rename(
        serialize = "pushMessageContentVoiceNote",
        deserialize = "pushMessageContentVoiceNote"
    ))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    caption: String,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentAnimationBuilder {
        let mut inner = PushMessageContentAnimation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentAnimationBuilder { inner }
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
pub struct RTDPushMessageContentAnimationBuilder {
    inner: PushMessageContentAnimation,
}

impl RTDPushMessageContentAnimationBuilder {
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

impl AsRef<PushMessageContentAnimation> for RTDPushMessageContentAnimationBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentAudioBuilder {
        let mut inner = PushMessageContentAudio::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentAudioBuilder { inner }
    }

    pub fn audio(&self) -> &Option<Audio> {
        &self.audio
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentAudioBuilder {
    inner: PushMessageContentAudio,
}

impl RTDPushMessageContentAudioBuilder {
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

impl AsRef<PushMessageContentAudio> for RTDPushMessageContentAudioBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentBasicGroupChatCreateBuilder {
        let mut inner = PushMessageContentBasicGroupChatCreate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentBasicGroupChatCreateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentBasicGroupChatCreateBuilder {
    inner: PushMessageContentBasicGroupChatCreate,
}

impl RTDPushMessageContentBasicGroupChatCreateBuilder {
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
    for RTDPushMessageContentBasicGroupChatCreateBuilder
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
    member_name: String,
    /// True, if the current user was added to the group
    is_current_user: bool,
    /// True, if the user has returned to the group themself
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentChatAddMembersBuilder {
        let mut inner = PushMessageContentChatAddMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentChatAddMembersBuilder { inner }
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
pub struct RTDPushMessageContentChatAddMembersBuilder {
    inner: PushMessageContentChatAddMembers,
}

impl RTDPushMessageContentChatAddMembersBuilder {
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

impl AsRef<PushMessageContentChatAddMembers> for RTDPushMessageContentChatAddMembersBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentChatChangePhotoBuilder {
        let mut inner = PushMessageContentChatChangePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentChatChangePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatChangePhotoBuilder {
    inner: PushMessageContentChatChangePhoto,
}

impl RTDPushMessageContentChatChangePhotoBuilder {
    pub fn build(&self) -> PushMessageContentChatChangePhoto {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentChatChangePhoto> for PushMessageContentChatChangePhoto {
    fn as_ref(&self) -> &PushMessageContentChatChangePhoto {
        self
    }
}

impl AsRef<PushMessageContentChatChangePhoto> for RTDPushMessageContentChatChangePhotoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentChatChangeTitleBuilder {
        let mut inner = PushMessageContentChatChangeTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentChatChangeTitleBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatChangeTitleBuilder {
    inner: PushMessageContentChatChangeTitle,
}

impl RTDPushMessageContentChatChangeTitleBuilder {
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

impl AsRef<PushMessageContentChatChangeTitle> for RTDPushMessageContentChatChangeTitleBuilder {
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
    member_name: String,
    /// True, if the current user was deleted from the group
    is_current_user: bool,
    /// True, if the user has left the group themself
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentChatDeleteMemberBuilder {
        let mut inner = PushMessageContentChatDeleteMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentChatDeleteMemberBuilder { inner }
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
pub struct RTDPushMessageContentChatDeleteMemberBuilder {
    inner: PushMessageContentChatDeleteMember,
}

impl RTDPushMessageContentChatDeleteMemberBuilder {
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

impl AsRef<PushMessageContentChatDeleteMember> for RTDPushMessageContentChatDeleteMemberBuilder {
    fn as_ref(&self) -> &PushMessageContentChatDeleteMember {
        &self.inner
    }
}

/// A new member joined the chat by invite link
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentChatJoinByLinkBuilder {
        let mut inner = PushMessageContentChatJoinByLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentChatJoinByLinkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentChatJoinByLinkBuilder {
    inner: PushMessageContentChatJoinByLink,
}

impl RTDPushMessageContentChatJoinByLinkBuilder {
    pub fn build(&self) -> PushMessageContentChatJoinByLink {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentChatJoinByLink> for PushMessageContentChatJoinByLink {
    fn as_ref(&self) -> &PushMessageContentChatJoinByLink {
        self
    }
}

impl AsRef<PushMessageContentChatJoinByLink> for RTDPushMessageContentChatJoinByLinkBuilder {
    fn as_ref(&self) -> &PushMessageContentChatJoinByLink {
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
    name: String,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentContactBuilder {
        let mut inner = PushMessageContentContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentContactBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentContactBuilder {
    inner: PushMessageContentContact,
}

impl RTDPushMessageContentContactBuilder {
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

impl AsRef<PushMessageContentContact> for RTDPushMessageContentContactBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentContactRegisteredBuilder {
        let mut inner = PushMessageContentContactRegistered::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentContactRegisteredBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentContactRegisteredBuilder {
    inner: PushMessageContentContactRegistered,
}

impl RTDPushMessageContentContactRegisteredBuilder {
    pub fn build(&self) -> PushMessageContentContactRegistered {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentContactRegistered> for PushMessageContentContactRegistered {
    fn as_ref(&self) -> &PushMessageContentContactRegistered {
        self
    }
}

impl AsRef<PushMessageContentContactRegistered> for RTDPushMessageContentContactRegisteredBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentDocumentBuilder {
        let mut inner = PushMessageContentDocument::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentDocumentBuilder { inner }
    }

    pub fn document(&self) -> &Option<Document> {
        &self.document
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentDocumentBuilder {
    inner: PushMessageContentDocument,
}

impl RTDPushMessageContentDocumentBuilder {
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

impl AsRef<PushMessageContentDocument> for RTDPushMessageContentDocumentBuilder {
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
    title: String,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentGameBuilder {
        let mut inner = PushMessageContentGame::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentGameBuilder { inner }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentGameBuilder {
    inner: PushMessageContentGame,
}

impl RTDPushMessageContentGameBuilder {
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

impl AsRef<PushMessageContentGame> for RTDPushMessageContentGameBuilder {
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
    title: String,
    /// New score, 0 for pinned message
    score: i32,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentGameScoreBuilder {
        let mut inner = PushMessageContentGameScore::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentGameScoreBuilder { inner }
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
pub struct RTDPushMessageContentGameScoreBuilder {
    inner: PushMessageContentGameScore,
}

impl RTDPushMessageContentGameScoreBuilder {
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

impl AsRef<PushMessageContentGameScore> for RTDPushMessageContentGameScoreBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentHiddenBuilder {
        let mut inner = PushMessageContentHidden::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentHiddenBuilder { inner }
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentHiddenBuilder {
    inner: PushMessageContentHidden,
}

impl RTDPushMessageContentHiddenBuilder {
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

impl AsRef<PushMessageContentHidden> for RTDPushMessageContentHiddenBuilder {
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
    price: String,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentInvoiceBuilder {
        let mut inner = PushMessageContentInvoice::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentInvoiceBuilder { inner }
    }

    pub fn price(&self) -> &String {
        &self.price
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentInvoiceBuilder {
    inner: PushMessageContentInvoice,
}

impl RTDPushMessageContentInvoiceBuilder {
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

impl AsRef<PushMessageContentInvoice> for RTDPushMessageContentInvoiceBuilder {
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
    is_live: bool,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentLocationBuilder {
        let mut inner = PushMessageContentLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentLocationBuilder { inner }
    }

    pub fn is_live(&self) -> bool {
        self.is_live
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentLocationBuilder {
    inner: PushMessageContentLocation,
}

impl RTDPushMessageContentLocationBuilder {
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

impl AsRef<PushMessageContentLocation> for RTDPushMessageContentLocationBuilder {
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
    total_count: i32,
    /// True, if the album has at least one photo
    has_photos: bool,
    /// True, if the album has at least one video
    has_videos: bool,
    /// True, if the album has at least one audio file
    has_audios: bool,
    /// True, if the album has at least one document
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentMediaAlbumBuilder {
        let mut inner = PushMessageContentMediaAlbum::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentMediaAlbumBuilder { inner }
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
pub struct RTDPushMessageContentMediaAlbumBuilder {
    inner: PushMessageContentMediaAlbum,
}

impl RTDPushMessageContentMediaAlbumBuilder {
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

impl AsRef<PushMessageContentMediaAlbum> for RTDPushMessageContentMediaAlbumBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentMessageForwardsBuilder {
        let mut inner = PushMessageContentMessageForwards::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentMessageForwardsBuilder { inner }
    }

    pub fn total_count(&self) -> i32 {
        self.total_count
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentMessageForwardsBuilder {
    inner: PushMessageContentMessageForwards,
}

impl RTDPushMessageContentMessageForwardsBuilder {
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

impl AsRef<PushMessageContentMessageForwards> for RTDPushMessageContentMessageForwardsBuilder {
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
    caption: String,
    /// True, if the photo is secret
    is_secret: bool,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentPhotoBuilder {
        let mut inner = PushMessageContentPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentPhotoBuilder { inner }
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
pub struct RTDPushMessageContentPhotoBuilder {
    inner: PushMessageContentPhoto,
}

impl RTDPushMessageContentPhotoBuilder {
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

impl AsRef<PushMessageContentPhoto> for RTDPushMessageContentPhotoBuilder {
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
    question: String,
    /// True, if the poll is regular and not in quiz mode
    is_regular: bool,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentPollBuilder {
        let mut inner = PushMessageContentPoll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentPollBuilder { inner }
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
pub struct RTDPushMessageContentPollBuilder {
    inner: PushMessageContentPoll,
}

impl RTDPushMessageContentPollBuilder {
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

impl AsRef<PushMessageContentPoll> for RTDPushMessageContentPollBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentScreenshotTakenBuilder {
        let mut inner = PushMessageContentScreenshotTaken::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentScreenshotTakenBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentScreenshotTakenBuilder {
    inner: PushMessageContentScreenshotTaken,
}

impl RTDPushMessageContentScreenshotTakenBuilder {
    pub fn build(&self) -> PushMessageContentScreenshotTaken {
        self.inner.clone()
    }
}

impl AsRef<PushMessageContentScreenshotTaken> for PushMessageContentScreenshotTaken {
    fn as_ref(&self) -> &PushMessageContentScreenshotTaken {
        self
    }
}

impl AsRef<PushMessageContentScreenshotTaken> for RTDPushMessageContentScreenshotTakenBuilder {
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
    emoji: String,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentStickerBuilder {
        let mut inner = PushMessageContentSticker::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentStickerBuilder { inner }
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
pub struct RTDPushMessageContentStickerBuilder {
    inner: PushMessageContentSticker,
}

impl RTDPushMessageContentStickerBuilder {
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

impl AsRef<PushMessageContentSticker> for RTDPushMessageContentStickerBuilder {
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
    text: String,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentTextBuilder {
        let mut inner = PushMessageContentText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentTextBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentTextBuilder {
    inner: PushMessageContentText,
}

impl RTDPushMessageContentTextBuilder {
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

impl AsRef<PushMessageContentText> for RTDPushMessageContentTextBuilder {
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
    caption: String,
    /// True, if the video is secret
    is_secret: bool,
    /// True, if the message is a pinned message with the specified content
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentVideoBuilder {
        let mut inner = PushMessageContentVideo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentVideoBuilder { inner }
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
pub struct RTDPushMessageContentVideoBuilder {
    inner: PushMessageContentVideo,
}

impl RTDPushMessageContentVideoBuilder {
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

impl AsRef<PushMessageContentVideo> for RTDPushMessageContentVideoBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentVideoNoteBuilder {
        let mut inner = PushMessageContentVideoNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentVideoNoteBuilder { inner }
    }

    pub fn video_note(&self) -> &Option<VideoNote> {
        &self.video_note
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentVideoNoteBuilder {
    inner: PushMessageContentVideoNote,
}

impl RTDPushMessageContentVideoNoteBuilder {
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

impl AsRef<PushMessageContentVideoNote> for RTDPushMessageContentVideoNoteBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPushMessageContentVoiceNoteBuilder {
        let mut inner = PushMessageContentVoiceNote::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPushMessageContentVoiceNoteBuilder { inner }
    }

    pub fn voice_note(&self) -> &Option<VoiceNote> {
        &self.voice_note
    }

    pub fn is_pinned(&self) -> bool {
        self.is_pinned
    }
}

#[doc(hidden)]
pub struct RTDPushMessageContentVoiceNoteBuilder {
    inner: PushMessageContentVoiceNote,
}

impl RTDPushMessageContentVoiceNoteBuilder {
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

impl AsRef<PushMessageContentVoiceNote> for RTDPushMessageContentVoiceNoteBuilder {
    fn as_ref(&self) -> &PushMessageContentVoiceNote {
        &self.inner
    }
}
