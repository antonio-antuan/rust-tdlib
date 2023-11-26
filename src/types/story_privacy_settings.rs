use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes privacy settings of a story
pub trait TDStoryPrivacySettings: Debug + RObject {}

/// Describes privacy settings of a story
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum StoryPrivacySettings {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The story can be viewed by all close friends
    #[serde(rename = "storyPrivacySettingsCloseFriends")]
    CloseFriends(StoryPrivacySettingsCloseFriends),
    /// The story can be viewed by all contacts except chosen users
    #[serde(rename = "storyPrivacySettingsContacts")]
    Contacts(StoryPrivacySettingsContacts),
    /// The story can be viewed by everyone
    #[serde(rename = "storyPrivacySettingsEveryone")]
    Everyone(StoryPrivacySettingsEveryone),
    /// The story can be viewed by certain specified users
    #[serde(rename = "storyPrivacySettingsSelectedUsers")]
    SelectedUsers(StoryPrivacySettingsSelectedUsers),
}

impl RObject for StoryPrivacySettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StoryPrivacySettings::CloseFriends(t) => t.extra(),
            StoryPrivacySettings::Contacts(t) => t.extra(),
            StoryPrivacySettings::Everyone(t) => t.extra(),
            StoryPrivacySettings::SelectedUsers(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StoryPrivacySettings::CloseFriends(t) => t.client_id(),
            StoryPrivacySettings::Contacts(t) => t.client_id(),
            StoryPrivacySettings::Everyone(t) => t.client_id(),
            StoryPrivacySettings::SelectedUsers(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StoryPrivacySettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StoryPrivacySettings::_Default)
    }
}

impl AsRef<StoryPrivacySettings> for StoryPrivacySettings {
    fn as_ref(&self) -> &StoryPrivacySettings {
        self
    }
}

/// The story can be viewed by all close friends
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsCloseFriends {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for StoryPrivacySettingsCloseFriends {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsCloseFriends {}

impl StoryPrivacySettingsCloseFriends {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryPrivacySettingsCloseFriendsBuilder {
        let mut inner = StoryPrivacySettingsCloseFriends::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryPrivacySettingsCloseFriendsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct StoryPrivacySettingsCloseFriendsBuilder {
    inner: StoryPrivacySettingsCloseFriends,
}

#[deprecated]
pub type RTDStoryPrivacySettingsCloseFriendsBuilder = StoryPrivacySettingsCloseFriendsBuilder;

impl StoryPrivacySettingsCloseFriendsBuilder {
    pub fn build(&self) -> StoryPrivacySettingsCloseFriends {
        self.inner.clone()
    }
}

impl AsRef<StoryPrivacySettingsCloseFriends> for StoryPrivacySettingsCloseFriends {
    fn as_ref(&self) -> &StoryPrivacySettingsCloseFriends {
        self
    }
}

impl AsRef<StoryPrivacySettingsCloseFriends> for StoryPrivacySettingsCloseFriendsBuilder {
    fn as_ref(&self) -> &StoryPrivacySettingsCloseFriends {
        &self.inner
    }
}

/// The story can be viewed by all contacts except chosen users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifiers of the contacts that can't see the story; always unknown and empty for non-owned stories

    #[serde(default)]
    except_user_ids: Vec<i64>,
}

impl RObject for StoryPrivacySettingsContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsContacts {}

impl StoryPrivacySettingsContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryPrivacySettingsContactsBuilder {
        let mut inner = StoryPrivacySettingsContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryPrivacySettingsContactsBuilder { inner }
    }

    pub fn except_user_ids(&self) -> &Vec<i64> {
        &self.except_user_ids
    }
}

#[doc(hidden)]
pub struct StoryPrivacySettingsContactsBuilder {
    inner: StoryPrivacySettingsContacts,
}

#[deprecated]
pub type RTDStoryPrivacySettingsContactsBuilder = StoryPrivacySettingsContactsBuilder;

impl StoryPrivacySettingsContactsBuilder {
    pub fn build(&self) -> StoryPrivacySettingsContacts {
        self.inner.clone()
    }

    pub fn except_user_ids(&mut self, except_user_ids: Vec<i64>) -> &mut Self {
        self.inner.except_user_ids = except_user_ids;
        self
    }
}

impl AsRef<StoryPrivacySettingsContacts> for StoryPrivacySettingsContacts {
    fn as_ref(&self) -> &StoryPrivacySettingsContacts {
        self
    }
}

impl AsRef<StoryPrivacySettingsContacts> for StoryPrivacySettingsContactsBuilder {
    fn as_ref(&self) -> &StoryPrivacySettingsContacts {
        &self.inner
    }
}

/// The story can be viewed by everyone
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsEveryone {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of the users that can't see the story; always unknown and empty for non-owned stories

    #[serde(default)]
    except_user_ids: Vec<i64>,
}

impl RObject for StoryPrivacySettingsEveryone {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsEveryone {}

impl StoryPrivacySettingsEveryone {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryPrivacySettingsEveryoneBuilder {
        let mut inner = StoryPrivacySettingsEveryone::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryPrivacySettingsEveryoneBuilder { inner }
    }

    pub fn except_user_ids(&self) -> &Vec<i64> {
        &self.except_user_ids
    }
}

#[doc(hidden)]
pub struct StoryPrivacySettingsEveryoneBuilder {
    inner: StoryPrivacySettingsEveryone,
}

#[deprecated]
pub type RTDStoryPrivacySettingsEveryoneBuilder = StoryPrivacySettingsEveryoneBuilder;

impl StoryPrivacySettingsEveryoneBuilder {
    pub fn build(&self) -> StoryPrivacySettingsEveryone {
        self.inner.clone()
    }

    pub fn except_user_ids(&mut self, except_user_ids: Vec<i64>) -> &mut Self {
        self.inner.except_user_ids = except_user_ids;
        self
    }
}

impl AsRef<StoryPrivacySettingsEveryone> for StoryPrivacySettingsEveryone {
    fn as_ref(&self) -> &StoryPrivacySettingsEveryone {
        self
    }
}

impl AsRef<StoryPrivacySettingsEveryone> for StoryPrivacySettingsEveryoneBuilder {
    fn as_ref(&self) -> &StoryPrivacySettingsEveryone {
        &self.inner
    }
}

/// The story can be viewed by certain specified users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryPrivacySettingsSelectedUsers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifiers of the users; always unknown and empty for non-owned stories

    #[serde(default)]
    user_ids: Vec<i64>,
}

impl RObject for StoryPrivacySettingsSelectedUsers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStoryPrivacySettings for StoryPrivacySettingsSelectedUsers {}

impl StoryPrivacySettingsSelectedUsers {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryPrivacySettingsSelectedUsersBuilder {
        let mut inner = StoryPrivacySettingsSelectedUsers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryPrivacySettingsSelectedUsersBuilder { inner }
    }

    pub fn user_ids(&self) -> &Vec<i64> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct StoryPrivacySettingsSelectedUsersBuilder {
    inner: StoryPrivacySettingsSelectedUsers,
}

#[deprecated]
pub type RTDStoryPrivacySettingsSelectedUsersBuilder = StoryPrivacySettingsSelectedUsersBuilder;

impl StoryPrivacySettingsSelectedUsersBuilder {
    pub fn build(&self) -> StoryPrivacySettingsSelectedUsers {
        self.inner.clone()
    }

    pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<StoryPrivacySettingsSelectedUsers> for StoryPrivacySettingsSelectedUsers {
    fn as_ref(&self) -> &StoryPrivacySettingsSelectedUsers {
        self
    }
}

impl AsRef<StoryPrivacySettingsSelectedUsers> for StoryPrivacySettingsSelectedUsersBuilder {
    fn as_ref(&self) -> &StoryPrivacySettingsSelectedUsers {
        &self.inner
    }
}
