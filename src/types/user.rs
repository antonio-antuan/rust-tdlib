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
    /// Username of the user

    #[serde(default)]
    username: String,
    /// Phone number of the user

    #[serde(default)]
    phone_number: String,
    /// Current online status of the user

    #[serde(skip_serializing_if = "UserStatus::_is_default")]
    status: UserStatus,
    /// Profile photo of the user; may be null
    profile_photo: Option<ProfilePhoto>,
    /// The user is a contact of the current user

    #[serde(default)]
    is_contact: bool,
    /// The user is a contact of the current user and the current user is a contact of the user

    #[serde(default)]
    is_mutual_contact: bool,
    /// True, if the user is verified

    #[serde(default)]
    is_verified: bool,
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
    /// If false, the user is inaccessible, and the only information known about the user is inside this class. It can't be passed to any method except GetUser

    #[serde(default)]
    have_access: bool,
    /// Type of the user

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "UserType::_is_default")]
    type_: UserType,
    /// IETF language tag of the user's language; only available to bots

    #[serde(default)]
    language_code: String,
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

    pub fn username(&self) -> &String {
        &self.username
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

    pub fn is_contact(&self) -> bool {
        self.is_contact
    }

    pub fn is_mutual_contact(&self) -> bool {
        self.is_mutual_contact
    }

    pub fn is_verified(&self) -> bool {
        self.is_verified
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

    pub fn have_access(&self) -> bool {
        self.have_access
    }

    pub fn type_(&self) -> &UserType {
        &self.type_
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
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

    pub fn username<T: AsRef<str>>(&mut self, username: T) -> &mut Self {
        self.inner.username = username.as_ref().to_string();
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

    pub fn is_contact(&mut self, is_contact: bool) -> &mut Self {
        self.inner.is_contact = is_contact;
        self
    }

    pub fn is_mutual_contact(&mut self, is_mutual_contact: bool) -> &mut Self {
        self.inner.is_mutual_contact = is_mutual_contact;
        self
    }

    pub fn is_verified(&mut self, is_verified: bool) -> &mut Self {
        self.inner.is_verified = is_verified;
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
