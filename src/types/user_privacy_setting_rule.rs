use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a single rule for managing privacy settings
pub trait TDUserPrivacySettingRule: Debug + RObject {}

/// Represents a single rule for managing privacy settings
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserPrivacySettingRule {
    #[doc(hidden)]
    _Default,
    /// A rule to allow all users to do something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowAll",
        deserialize = "userPrivacySettingRuleAllowAll"
    ))]
    AllowAll(UserPrivacySettingRuleAllowAll),
    /// A rule to allow all members of certain specified basic groups and supergroups to doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowChatMembers",
        deserialize = "userPrivacySettingRuleAllowChatMembers"
    ))]
    AllowChatMembers(UserPrivacySettingRuleAllowChatMembers),
    /// A rule to allow all of a user's contacts to do something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowContacts",
        deserialize = "userPrivacySettingRuleAllowContacts"
    ))]
    AllowContacts(UserPrivacySettingRuleAllowContacts),
    /// A rule to allow certain specified users to do something
    #[serde(rename(
        serialize = "userPrivacySettingRuleAllowUsers",
        deserialize = "userPrivacySettingRuleAllowUsers"
    ))]
    AllowUsers(UserPrivacySettingRuleAllowUsers),
    /// A rule to restrict all users from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictAll",
        deserialize = "userPrivacySettingRuleRestrictAll"
    ))]
    RestrictAll(UserPrivacySettingRuleRestrictAll),
    /// A rule to restrict all members of specified basic groups and supergroups from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictChatMembers",
        deserialize = "userPrivacySettingRuleRestrictChatMembers"
    ))]
    RestrictChatMembers(UserPrivacySettingRuleRestrictChatMembers),
    /// A rule to restrict all contacts of a user from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictContacts",
        deserialize = "userPrivacySettingRuleRestrictContacts"
    ))]
    RestrictContacts(UserPrivacySettingRuleRestrictContacts),
    /// A rule to restrict all specified users from doing something
    #[serde(rename(
        serialize = "userPrivacySettingRuleRestrictUsers",
        deserialize = "userPrivacySettingRuleRestrictUsers"
    ))]
    RestrictUsers(UserPrivacySettingRuleRestrictUsers),
}

impl Default for UserPrivacySettingRule {
    fn default() -> Self {
        UserPrivacySettingRule::_Default
    }
}

impl RObject for UserPrivacySettingRule {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            UserPrivacySettingRule::AllowAll(t) => t.extra(),
            UserPrivacySettingRule::AllowChatMembers(t) => t.extra(),
            UserPrivacySettingRule::AllowContacts(t) => t.extra(),
            UserPrivacySettingRule::AllowUsers(t) => t.extra(),
            UserPrivacySettingRule::RestrictAll(t) => t.extra(),
            UserPrivacySettingRule::RestrictChatMembers(t) => t.extra(),
            UserPrivacySettingRule::RestrictContacts(t) => t.extra(),
            UserPrivacySettingRule::RestrictUsers(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            UserPrivacySettingRule::AllowAll(t) => t.client_id(),
            UserPrivacySettingRule::AllowChatMembers(t) => t.client_id(),
            UserPrivacySettingRule::AllowContacts(t) => t.client_id(),
            UserPrivacySettingRule::AllowUsers(t) => t.client_id(),
            UserPrivacySettingRule::RestrictAll(t) => t.client_id(),
            UserPrivacySettingRule::RestrictChatMembers(t) => t.client_id(),
            UserPrivacySettingRule::RestrictContacts(t) => t.client_id(),
            UserPrivacySettingRule::RestrictUsers(t) => t.client_id(),

            _ => None,
        }
    }
}

impl UserPrivacySettingRule {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, UserPrivacySettingRule::_Default)
    }
}

impl AsRef<UserPrivacySettingRule> for UserPrivacySettingRule {
    fn as_ref(&self) -> &UserPrivacySettingRule {
        self
    }
}

/// A rule to allow all users to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowAll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleAllowAll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowAll {}

impl UserPrivacySettingRuleAllowAll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowAllBuilder {
        let mut inner = UserPrivacySettingRuleAllowAll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleAllowAllBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowAllBuilder {
    inner: UserPrivacySettingRuleAllowAll,
}

impl RTDUserPrivacySettingRuleAllowAllBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleAllowAll {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingRuleAllowAll> for UserPrivacySettingRuleAllowAll {
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowAll {
        self
    }
}

impl AsRef<UserPrivacySettingRuleAllowAll> for RTDUserPrivacySettingRuleAllowAllBuilder {
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowAll {
        &self.inner
    }
}

/// A rule to allow all members of certain specified basic groups and supergroups to doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowChatMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat identifiers, total number of chats in all rules must not exceed 20
    chat_ids: Vec<i64>,
}

impl RObject for UserPrivacySettingRuleAllowChatMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowChatMembers {}

impl UserPrivacySettingRuleAllowChatMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowChatMembersBuilder {
        let mut inner = UserPrivacySettingRuleAllowChatMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleAllowChatMembersBuilder { inner }
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowChatMembersBuilder {
    inner: UserPrivacySettingRuleAllowChatMembers,
}

impl RTDUserPrivacySettingRuleAllowChatMembersBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleAllowChatMembers {
        self.inner.clone()
    }

    pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
        self.inner.chat_ids = chat_ids;
        self
    }
}

impl AsRef<UserPrivacySettingRuleAllowChatMembers> for UserPrivacySettingRuleAllowChatMembers {
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowChatMembers {
        self
    }
}

impl AsRef<UserPrivacySettingRuleAllowChatMembers>
    for RTDUserPrivacySettingRuleAllowChatMembersBuilder
{
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowChatMembers {
        &self.inner
    }
}

/// A rule to allow all of a user's contacts to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleAllowContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowContacts {}

impl UserPrivacySettingRuleAllowContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowContactsBuilder {
        let mut inner = UserPrivacySettingRuleAllowContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleAllowContactsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowContactsBuilder {
    inner: UserPrivacySettingRuleAllowContacts,
}

impl RTDUserPrivacySettingRuleAllowContactsBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleAllowContacts {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingRuleAllowContacts> for UserPrivacySettingRuleAllowContacts {
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowContacts {
        self
    }
}

impl AsRef<UserPrivacySettingRuleAllowContacts> for RTDUserPrivacySettingRuleAllowContactsBuilder {
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowContacts {
        &self.inner
    }
}

/// A rule to allow certain specified users to do something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleAllowUsers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The user identifiers, total number of users in all rules must not exceed 1000
    user_ids: Vec<i32>,
}

impl RObject for UserPrivacySettingRuleAllowUsers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowUsers {}

impl UserPrivacySettingRuleAllowUsers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowUsersBuilder {
        let mut inner = UserPrivacySettingRuleAllowUsers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleAllowUsersBuilder { inner }
    }

    pub fn user_ids(&self) -> &Vec<i32> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleAllowUsersBuilder {
    inner: UserPrivacySettingRuleAllowUsers,
}

impl RTDUserPrivacySettingRuleAllowUsersBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleAllowUsers {
        self.inner.clone()
    }

    pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<UserPrivacySettingRuleAllowUsers> for UserPrivacySettingRuleAllowUsers {
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowUsers {
        self
    }
}

impl AsRef<UserPrivacySettingRuleAllowUsers> for RTDUserPrivacySettingRuleAllowUsersBuilder {
    fn as_ref(&self) -> &UserPrivacySettingRuleAllowUsers {
        &self.inner
    }
}

/// A rule to restrict all users from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictAll {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleRestrictAll {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictAll {}

impl UserPrivacySettingRuleRestrictAll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictAllBuilder {
        let mut inner = UserPrivacySettingRuleRestrictAll::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleRestrictAllBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictAllBuilder {
    inner: UserPrivacySettingRuleRestrictAll,
}

impl RTDUserPrivacySettingRuleRestrictAllBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleRestrictAll {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingRuleRestrictAll> for UserPrivacySettingRuleRestrictAll {
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictAll {
        self
    }
}

impl AsRef<UserPrivacySettingRuleRestrictAll> for RTDUserPrivacySettingRuleRestrictAllBuilder {
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictAll {
        &self.inner
    }
}

/// A rule to restrict all members of specified basic groups and supergroups from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictChatMembers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chat identifiers, total number of chats in all rules must not exceed 20
    chat_ids: Vec<i64>,
}

impl RObject for UserPrivacySettingRuleRestrictChatMembers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictChatMembers {}

impl UserPrivacySettingRuleRestrictChatMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
        let mut inner = UserPrivacySettingRuleRestrictChatMembers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleRestrictChatMembersBuilder { inner }
    }

    pub fn chat_ids(&self) -> &Vec<i64> {
        &self.chat_ids
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
    inner: UserPrivacySettingRuleRestrictChatMembers,
}

impl RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleRestrictChatMembers {
        self.inner.clone()
    }

    pub fn chat_ids(&mut self, chat_ids: Vec<i64>) -> &mut Self {
        self.inner.chat_ids = chat_ids;
        self
    }
}

impl AsRef<UserPrivacySettingRuleRestrictChatMembers>
    for UserPrivacySettingRuleRestrictChatMembers
{
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictChatMembers {
        self
    }
}

impl AsRef<UserPrivacySettingRuleRestrictChatMembers>
    for RTDUserPrivacySettingRuleRestrictChatMembersBuilder
{
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictChatMembers {
        &self.inner
    }
}

/// A rule to restrict all contacts of a user from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleRestrictContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictContacts {}

impl UserPrivacySettingRuleRestrictContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictContactsBuilder {
        let mut inner = UserPrivacySettingRuleRestrictContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleRestrictContactsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictContactsBuilder {
    inner: UserPrivacySettingRuleRestrictContacts,
}

impl RTDUserPrivacySettingRuleRestrictContactsBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleRestrictContacts {
        self.inner.clone()
    }
}

impl AsRef<UserPrivacySettingRuleRestrictContacts> for UserPrivacySettingRuleRestrictContacts {
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictContacts {
        self
    }
}

impl AsRef<UserPrivacySettingRuleRestrictContacts>
    for RTDUserPrivacySettingRuleRestrictContactsBuilder
{
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictContacts {
        &self.inner
    }
}

/// A rule to restrict all specified users from doing something
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserPrivacySettingRuleRestrictUsers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The user identifiers, total number of users in all rules must not exceed 1000
    user_ids: Vec<i32>,
}

impl RObject for UserPrivacySettingRuleRestrictUsers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictUsers {}

impl UserPrivacySettingRuleRestrictUsers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictUsersBuilder {
        let mut inner = UserPrivacySettingRuleRestrictUsers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserPrivacySettingRuleRestrictUsersBuilder { inner }
    }

    pub fn user_ids(&self) -> &Vec<i32> {
        &self.user_ids
    }
}

#[doc(hidden)]
pub struct RTDUserPrivacySettingRuleRestrictUsersBuilder {
    inner: UserPrivacySettingRuleRestrictUsers,
}

impl RTDUserPrivacySettingRuleRestrictUsersBuilder {
    pub fn build(&self) -> UserPrivacySettingRuleRestrictUsers {
        self.inner.clone()
    }

    pub fn user_ids(&mut self, user_ids: Vec<i32>) -> &mut Self {
        self.inner.user_ids = user_ids;
        self
    }
}

impl AsRef<UserPrivacySettingRuleRestrictUsers> for UserPrivacySettingRuleRestrictUsers {
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictUsers {
        self
    }
}

impl AsRef<UserPrivacySettingRuleRestrictUsers> for RTDUserPrivacySettingRuleRestrictUsersBuilder {
    fn as_ref(&self) -> &UserPrivacySettingRuleRestrictUsers {
        &self.inner
    }
}
