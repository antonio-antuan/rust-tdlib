use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents a single rule for managing privacy settings
pub trait TDUserPrivacySettingRule: Debug + RObject {}

/// Represents a single rule for managing privacy settings
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserPrivacySettingRule {
    #[doc(hidden)]
    _Default(()),
    /// A rule to allow all users to do something
    AllowAll(UserPrivacySettingRuleAllowAll),
    /// A rule to allow all members of certain specified basic groups and supergroups to doing something
    AllowChatMembers(UserPrivacySettingRuleAllowChatMembers),
    /// A rule to allow all of a user's contacts to do something
    AllowContacts(UserPrivacySettingRuleAllowContacts),
    /// A rule to allow certain specified users to do something
    AllowUsers(UserPrivacySettingRuleAllowUsers),
    /// A rule to restrict all users from doing something
    RestrictAll(UserPrivacySettingRuleRestrictAll),
    /// A rule to restrict all members of specified basic groups and supergroups from doing something
    RestrictChatMembers(UserPrivacySettingRuleRestrictChatMembers),
    /// A rule to restrict all contacts of a user from doing something
    RestrictContacts(UserPrivacySettingRuleRestrictContacts),
    /// A rule to restrict all specified users from doing something
    RestrictUsers(UserPrivacySettingRuleRestrictUsers),
}

impl Default for UserPrivacySettingRule {
    fn default() -> Self {
        UserPrivacySettingRule::_Default(())
    }
}

impl<'de> Deserialize<'de> for UserPrivacySettingRule {
    fn deserialize<D>(deserializer: D) -> Result<UserPrivacySettingRule, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          UserPrivacySettingRule,
          (userPrivacySettingRuleAllowAll, AllowAll);
          (userPrivacySettingRuleAllowChatMembers, AllowChatMembers);
          (userPrivacySettingRuleAllowContacts, AllowContacts);
          (userPrivacySettingRuleAllowUsers, AllowUsers);
          (userPrivacySettingRuleRestrictAll, RestrictAll);
          (userPrivacySettingRuleRestrictChatMembers, RestrictChatMembers);
          (userPrivacySettingRuleRestrictContacts, RestrictContacts);
          (userPrivacySettingRuleRestrictUsers, RestrictUsers);

        )(deserializer)
    }
}

impl RObject for UserPrivacySettingRule {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            UserPrivacySettingRule::AllowAll(t) => t.td_name(),
            UserPrivacySettingRule::AllowChatMembers(t) => t.td_name(),
            UserPrivacySettingRule::AllowContacts(t) => t.td_name(),
            UserPrivacySettingRule::AllowUsers(t) => t.td_name(),
            UserPrivacySettingRule::RestrictAll(t) => t.td_name(),
            UserPrivacySettingRule::RestrictChatMembers(t) => t.td_name(),
            UserPrivacySettingRule::RestrictContacts(t) => t.td_name(),
            UserPrivacySettingRule::RestrictUsers(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
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
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
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
        matches!(self, UserPrivacySettingRule::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleAllowAll {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleAllowAll"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowAll {}

impl UserPrivacySettingRuleAllowAll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowAllBuilder {
        let mut inner = UserPrivacySettingRuleAllowAll::default();
        inner.td_name = "userPrivacySettingRuleAllowAll".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleAllowChatMembers"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowChatMembers {}

impl UserPrivacySettingRuleAllowChatMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowChatMembersBuilder {
        let mut inner = UserPrivacySettingRuleAllowChatMembers::default();
        inner.td_name = "userPrivacySettingRuleAllowChatMembers".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleAllowContacts {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleAllowContacts"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowContacts {}

impl UserPrivacySettingRuleAllowContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowContactsBuilder {
        let mut inner = UserPrivacySettingRuleAllowContacts::default();
        inner.td_name = "userPrivacySettingRuleAllowContacts".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleAllowUsers"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleAllowUsers {}

impl UserPrivacySettingRuleAllowUsers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleAllowUsersBuilder {
        let mut inner = UserPrivacySettingRuleAllowUsers::default();
        inner.td_name = "userPrivacySettingRuleAllowUsers".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleRestrictAll {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleRestrictAll"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictAll {}

impl UserPrivacySettingRuleRestrictAll {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictAllBuilder {
        let mut inner = UserPrivacySettingRuleRestrictAll::default();
        inner.td_name = "userPrivacySettingRuleRestrictAll".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleRestrictChatMembers"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictChatMembers {}

impl UserPrivacySettingRuleRestrictChatMembers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictChatMembersBuilder {
        let mut inner = UserPrivacySettingRuleRestrictChatMembers::default();
        inner.td_name = "userPrivacySettingRuleRestrictChatMembers".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserPrivacySettingRuleRestrictContacts {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleRestrictContacts"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictContacts {}

impl UserPrivacySettingRuleRestrictContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictContactsBuilder {
        let mut inner = UserPrivacySettingRuleRestrictContacts::default();
        inner.td_name = "userPrivacySettingRuleRestrictContacts".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
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
    fn td_name(&self) -> &'static str {
        "userPrivacySettingRuleRestrictUsers"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDUserPrivacySettingRule for UserPrivacySettingRuleRestrictUsers {}

impl UserPrivacySettingRuleRestrictUsers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserPrivacySettingRuleRestrictUsersBuilder {
        let mut inner = UserPrivacySettingRuleRestrictUsers::default();
        inner.td_name = "userPrivacySettingRuleRestrictUsers".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
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
