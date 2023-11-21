use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct User {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    id: i64,
    /// First name of the user

    #[serde(default)]
    first_name: String,
    /// Last name of the user

    #[serde(default)]
    last_name: String,
    /// Usernames of the user; may be null
    usernames: Option<Usernames>,
    /// Phone number of the user

    #[serde(default)]
    phone_number: String,
    /// Current online status of the user

    #[serde(skip_serializing_if = "UserStatus::_is_default")]
    status: UserStatus,
    /// Profile photo of the user; may be null
    profile_photo: Option<ProfilePhoto>,
    /// Identifier of the accent color for name, and backgrounds of profile photo, reply header, and link preview

    #[serde(default)]
    accent_color_id: i32,
    /// Identifier of a custom emoji to be shown on the reply header background; 0 if none. For Telegram Premium users only

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    background_custom_emoji_id: i64,
    /// Emoji status to be shown instead of the default Telegram Premium badge; may be null. For Telegram Premium users only
    emoji_status: Option<EmojiStatus>,
    /// The user is a contact of the current user

    #[serde(default)]
    is_contact: bool,
    /// The user is a contact of the current user and the current user is a contact of the user

    #[serde(default)]
    is_mutual_contact: bool,
    /// The user is a close friend of the current user; implies that the user is a contact

    #[serde(default)]
    is_close_friend: bool,
    /// True, if the user is verified

    #[serde(default)]
    is_verified: bool,
    /// True, if the user is a Telegram Premium user

    #[serde(default)]
    is_premium: bool,
    /// True, if the user is Telegram support account

    #[serde(default)]
    is_support: bool,
    /// If non-empty, it contains a human-readable description of the reason why access to this user must be restricted

    #[serde(default)]
    restriction_reason: String,
    /// True, if many users reported this user as a scam

    #[serde(default)]
    is_scam: bool,
    /// True, if many users reported this user as a fake account

    #[serde(default)]
    is_fake: bool,
    /// True, if the user has non-expired stories available to the current user

    #[serde(default)]
    has_active_stories: bool,
    /// True, if the user has unread non-expired stories available to the current user

    #[serde(default)]
    has_unread_active_stories: bool,
    /// If false, the user is inaccessible, and the only information known about the user is inside this class. Identifier of the user can't be passed to any method

    #[serde(default)]
    have_access: bool,
    /// Type of the user

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "UserType::_is_default")]
    type_: UserType,
    /// IETF language tag of the user's language; only available to bots

    #[serde(default)]
    language_code: String,
    /// True, if the user added the current bot to attachment menu; only available to bots

    #[serde(default)]
    added_to_attachment_menu: bool,
}

impl RObject for User {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl User {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> UserBuilder {
        let mut inner = User::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        UserBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn first_name(&self) -> &String {
        &self.first_name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn usernames(&self) -> &Option<Usernames> {
        &self.usernames
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }

    pub fn status(&self) -> &UserStatus {
        &self.status
    }

    pub fn profile_photo(&self) -> &Option<ProfilePhoto> {
        &self.profile_photo
    }

    pub fn accent_color_id(&self) -> i32 {
        self.accent_color_id
    }

    pub fn background_custom_emoji_id(&self) -> i64 {
        self.background_custom_emoji_id
    }

    pub fn emoji_status(&self) -> &Option<EmojiStatus> {
        &self.emoji_status
    }

    pub fn is_contact(&self) -> bool {
        self.is_contact
    }

    pub fn is_mutual_contact(&self) -> bool {
        self.is_mutual_contact
    }

    pub fn is_close_friend(&self) -> bool {
        self.is_close_friend
    }

    pub fn is_verified(&self) -> bool {
        self.is_verified
    }

    pub fn is_premium(&self) -> bool {
        self.is_premium
    }

    pub fn is_support(&self) -> bool {
        self.is_support
    }

    pub fn restriction_reason(&self) -> &String {
        &self.restriction_reason
    }

    pub fn is_scam(&self) -> bool {
        self.is_scam
    }

    pub fn is_fake(&self) -> bool {
        self.is_fake
    }

    pub fn has_active_stories(&self) -> bool {
        self.has_active_stories
    }

    pub fn has_unread_active_stories(&self) -> bool {
        self.has_unread_active_stories
    }

    pub fn have_access(&self) -> bool {
        self.have_access
    }

    pub fn type_(&self) -> &UserType {
        &self.type_
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    pub fn added_to_attachment_menu(&self) -> bool {
        self.added_to_attachment_menu
    }
}

#[doc(hidden)]
pub struct UserBuilder {
    inner: User,
}

#[deprecated]
pub type RTDUserBuilder = UserBuilder;

impl UserBuilder {
    pub fn build(&self) -> User {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn first_name<T: AsRef<str>>(&mut self, first_name: T) -> &mut Self {
        self.inner.first_name = first_name.as_ref().to_string();
        self
    }

    pub fn last_name<T: AsRef<str>>(&mut self, last_name: T) -> &mut Self {
        self.inner.last_name = last_name.as_ref().to_string();
        self
    }

    pub fn usernames<T: AsRef<Usernames>>(&mut self, usernames: T) -> &mut Self {
        self.inner.usernames = Some(usernames.as_ref().clone());
        self
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }

    pub fn status<T: AsRef<UserStatus>>(&mut self, status: T) -> &mut Self {
        self.inner.status = status.as_ref().clone();
        self
    }

    pub fn profile_photo<T: AsRef<ProfilePhoto>>(&mut self, profile_photo: T) -> &mut Self {
        self.inner.profile_photo = Some(profile_photo.as_ref().clone());
        self
    }

    pub fn accent_color_id(&mut self, accent_color_id: i32) -> &mut Self {
        self.inner.accent_color_id = accent_color_id;
        self
    }

    pub fn background_custom_emoji_id(&mut self, background_custom_emoji_id: i64) -> &mut Self {
        self.inner.background_custom_emoji_id = background_custom_emoji_id;
        self
    }

    pub fn emoji_status<T: AsRef<EmojiStatus>>(&mut self, emoji_status: T) -> &mut Self {
        self.inner.emoji_status = Some(emoji_status.as_ref().clone());
        self
    }

    pub fn is_contact(&mut self, is_contact: bool) -> &mut Self {
        self.inner.is_contact = is_contact;
        self
    }

    pub fn is_mutual_contact(&mut self, is_mutual_contact: bool) -> &mut Self {
        self.inner.is_mutual_contact = is_mutual_contact;
        self
    }

    pub fn is_close_friend(&mut self, is_close_friend: bool) -> &mut Self {
        self.inner.is_close_friend = is_close_friend;
        self
    }

    pub fn is_verified(&mut self, is_verified: bool) -> &mut Self {
        self.inner.is_verified = is_verified;
        self
    }

    pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
        self.inner.is_premium = is_premium;
        self
    }

    pub fn is_support(&mut self, is_support: bool) -> &mut Self {
        self.inner.is_support = is_support;
        self
    }

    pub fn restriction_reason<T: AsRef<str>>(&mut self, restriction_reason: T) -> &mut Self {
        self.inner.restriction_reason = restriction_reason.as_ref().to_string();
        self
    }

    pub fn is_scam(&mut self, is_scam: bool) -> &mut Self {
        self.inner.is_scam = is_scam;
        self
    }

    pub fn is_fake(&mut self, is_fake: bool) -> &mut Self {
        self.inner.is_fake = is_fake;
        self
    }

    pub fn has_active_stories(&mut self, has_active_stories: bool) -> &mut Self {
        self.inner.has_active_stories = has_active_stories;
        self
    }

    pub fn has_unread_active_stories(&mut self, has_unread_active_stories: bool) -> &mut Self {
        self.inner.has_unread_active_stories = has_unread_active_stories;
        self
    }

    pub fn have_access(&mut self, have_access: bool) -> &mut Self {
        self.inner.have_access = have_access;
        self
    }

    pub fn type_<T: AsRef<UserType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }

    pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
        self.inner.language_code = language_code.as_ref().to_string();
        self
    }

    pub fn added_to_attachment_menu(&mut self, added_to_attachment_menu: bool) -> &mut Self {
        self.inner.added_to_attachment_menu = added_to_attachment_menu;
        self
    }
}

impl AsRef<User> for User {
    fn as_ref(&self) -> &User {
        self
    }
}

impl AsRef<User> for UserBuilder {
    fn as_ref(&self) -> &User {
        &self.inner
    }
}
