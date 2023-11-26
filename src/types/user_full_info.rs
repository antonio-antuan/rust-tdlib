use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains full information about a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserFullInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User profile photo set by the current user for the contact; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown. If non-null, then it is the same photo as in user.profile_photo and chat.photo. This photo isn't returned in the list of user photos
    personal_photo: Option<ChatPhoto>,
    /// User profile photo; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown. If non-null and personal_photo is null, then it is the same photo as in user.profile_photo and chat.photo
    photo: Option<ChatPhoto>,
    /// User profile photo visible if the main photo is hidden by privacy settings; may be null. If null and user.profile_photo is null, then the photo is empty; otherwise, it is unknown. If non-null and both photo and personal_photo are null, then it is the same photo as in user.profile_photo and chat.photo. This photo isn't returned in the list of user photos
    public_photo: Option<ChatPhoto>,
    /// Block list to which the user is added; may be null if none
    block_list: Option<BlockList>,
    /// True, if the user can be called

    #[serde(default)]
    can_be_called: bool,
    /// True, if a video call can be created with the user

    #[serde(default)]
    supports_video_calls: bool,
    /// True, if the user can't be called due to their privacy settings

    #[serde(default)]
    has_private_calls: bool,
    /// True, if the user can't be linked in forwarded messages due to their privacy settings

    #[serde(default)]
    has_private_forwards: bool,
    /// True, if voice and video notes can't be sent or forwarded to the user

    #[serde(default)]
    has_restricted_voice_and_video_note_messages: bool,
    /// True, if the user has pinned stories

    #[serde(default)]
    has_pinned_stories: bool,
    /// True, if the current user needs to explicitly allow to share their phone number with the user when the method addContact is used

    #[serde(default)]
    need_phone_number_privacy_exception: bool,
    /// A short user bio; may be null for bots
    bio: Option<FormattedText>,
    /// The list of available options for gifting Telegram Premium to the user

    #[serde(default)]
    premium_gift_options: Vec<PremiumPaymentOption>,
    /// Number of group chats where both the other user and the current user are a member; 0 for the current user

    #[serde(default)]
    group_in_common_count: i32,
    /// For bots, information about the bot; may be null if the user isn't a bot
    bot_info: Option<BotInfo>,
}

impl RObject for UserFullInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl UserFullInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserFullInfoBuilder {
        let mut inner = UserFullInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserFullInfoBuilder { inner }
    }

    pub fn personal_photo(&self) -> &Option<ChatPhoto> {
        &self.personal_photo
    }

    pub fn photo(&self) -> &Option<ChatPhoto> {
        &self.photo
    }

    pub fn public_photo(&self) -> &Option<ChatPhoto> {
        &self.public_photo
    }

    pub fn block_list(&self) -> &Option<BlockList> {
        &self.block_list
    }

    pub fn can_be_called(&self) -> bool {
        self.can_be_called
    }

    pub fn supports_video_calls(&self) -> bool {
        self.supports_video_calls
    }

    pub fn has_private_calls(&self) -> bool {
        self.has_private_calls
    }

    pub fn has_private_forwards(&self) -> bool {
        self.has_private_forwards
    }

    pub fn has_restricted_voice_and_video_note_messages(&self) -> bool {
        self.has_restricted_voice_and_video_note_messages
    }

    pub fn has_pinned_stories(&self) -> bool {
        self.has_pinned_stories
    }

    pub fn need_phone_number_privacy_exception(&self) -> bool {
        self.need_phone_number_privacy_exception
    }

    pub fn bio(&self) -> &Option<FormattedText> {
        &self.bio
    }

    pub fn premium_gift_options(&self) -> &Vec<PremiumPaymentOption> {
        &self.premium_gift_options
    }

    pub fn group_in_common_count(&self) -> i32 {
        self.group_in_common_count
    }

    pub fn bot_info(&self) -> &Option<BotInfo> {
        &self.bot_info
    }
}

#[doc(hidden)]
pub struct UserFullInfoBuilder {
    inner: UserFullInfo,
}

#[deprecated]
pub type RTDUserFullInfoBuilder = UserFullInfoBuilder;

impl UserFullInfoBuilder {
    pub fn build(&self) -> UserFullInfo {
        self.inner.clone()
    }

    pub fn personal_photo<T: AsRef<ChatPhoto>>(&mut self, personal_photo: T) -> &mut Self {
        self.inner.personal_photo = Some(personal_photo.as_ref().clone());
        self
    }

    pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
        self.inner.photo = Some(photo.as_ref().clone());
        self
    }

    pub fn public_photo<T: AsRef<ChatPhoto>>(&mut self, public_photo: T) -> &mut Self {
        self.inner.public_photo = Some(public_photo.as_ref().clone());
        self
    }

    pub fn block_list<T: AsRef<BlockList>>(&mut self, block_list: T) -> &mut Self {
        self.inner.block_list = Some(block_list.as_ref().clone());
        self
    }

    pub fn can_be_called(&mut self, can_be_called: bool) -> &mut Self {
        self.inner.can_be_called = can_be_called;
        self
    }

    pub fn supports_video_calls(&mut self, supports_video_calls: bool) -> &mut Self {
        self.inner.supports_video_calls = supports_video_calls;
        self
    }

    pub fn has_private_calls(&mut self, has_private_calls: bool) -> &mut Self {
        self.inner.has_private_calls = has_private_calls;
        self
    }

    pub fn has_private_forwards(&mut self, has_private_forwards: bool) -> &mut Self {
        self.inner.has_private_forwards = has_private_forwards;
        self
    }

    pub fn has_restricted_voice_and_video_note_messages(
        &mut self,
        has_restricted_voice_and_video_note_messages: bool,
    ) -> &mut Self {
        self.inner.has_restricted_voice_and_video_note_messages =
            has_restricted_voice_and_video_note_messages;
        self
    }

    pub fn has_pinned_stories(&mut self, has_pinned_stories: bool) -> &mut Self {
        self.inner.has_pinned_stories = has_pinned_stories;
        self
    }

    pub fn need_phone_number_privacy_exception(
        &mut self,
        need_phone_number_privacy_exception: bool,
    ) -> &mut Self {
        self.inner.need_phone_number_privacy_exception = need_phone_number_privacy_exception;
        self
    }

    pub fn bio<T: AsRef<FormattedText>>(&mut self, bio: T) -> &mut Self {
        self.inner.bio = Some(bio.as_ref().clone());
        self
    }

    pub fn premium_gift_options(
        &mut self,
        premium_gift_options: Vec<PremiumPaymentOption>,
    ) -> &mut Self {
        self.inner.premium_gift_options = premium_gift_options;
        self
    }

    pub fn group_in_common_count(&mut self, group_in_common_count: i32) -> &mut Self {
        self.inner.group_in_common_count = group_in_common_count;
        self
    }

    pub fn bot_info<T: AsRef<BotInfo>>(&mut self, bot_info: T) -> &mut Self {
        self.inner.bot_info = Some(bot_info.as_ref().clone());
        self
    }
}

impl AsRef<UserFullInfo> for UserFullInfo {
    fn as_ref(&self) -> &UserFullInfo {
        self
    }
}

impl AsRef<UserFullInfo> for UserFullInfoBuilder {
    fn as_ref(&self) -> &UserFullInfo {
        &self.inner
    }
}
