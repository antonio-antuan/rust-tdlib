use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Specifies the kind of chat members to return in getSupergroupMembers
pub trait TDSupergroupMembersFilter: Debug + RObject {}

/// Specifies the kind of chat members to return in getSupergroupMembers
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum SupergroupMembersFilter {
    #[doc(hidden)]
    _Default,
    /// Returns the owner and administrators
    #[serde(rename = "supergroupMembersFilterAdministrators")]
    Administrators(SupergroupMembersFilterAdministrators),
    /// Returns users banned from the supergroup or channel; can be used only by administrators
    #[serde(rename = "supergroupMembersFilterBanned")]
    Banned(SupergroupMembersFilterBanned),
    /// Returns bot members of the supergroup or channel
    #[serde(rename = "supergroupMembersFilterBots")]
    Bots(SupergroupMembersFilterBots),
    /// Returns contacts of the user, which are members of the supergroup or channel
    #[serde(rename = "supergroupMembersFilterContacts")]
    Contacts(SupergroupMembersFilterContacts),
    /// Returns users which can be mentioned in the supergroup
    #[serde(rename = "supergroupMembersFilterMention")]
    Mention(SupergroupMembersFilterMention),
    /// Returns recently active users in reverse chronological order
    #[serde(rename = "supergroupMembersFilterRecent")]
    Recent(SupergroupMembersFilterRecent),
    /// Returns restricted supergroup members; can be used only by administrators
    #[serde(rename = "supergroupMembersFilterRestricted")]
    Restricted(SupergroupMembersFilterRestricted),
    /// Used to search for supergroup or channel members via a (string) query
    #[serde(rename = "supergroupMembersFilterSearch")]
    Search(SupergroupMembersFilterSearch),
}

impl Default for SupergroupMembersFilter {
    fn default() -> Self {
        SupergroupMembersFilter::_Default
    }
}

impl RObject for SupergroupMembersFilter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            SupergroupMembersFilter::Administrators(t) => t.extra(),
            SupergroupMembersFilter::Banned(t) => t.extra(),
            SupergroupMembersFilter::Bots(t) => t.extra(),
            SupergroupMembersFilter::Contacts(t) => t.extra(),
            SupergroupMembersFilter::Mention(t) => t.extra(),
            SupergroupMembersFilter::Recent(t) => t.extra(),
            SupergroupMembersFilter::Restricted(t) => t.extra(),
            SupergroupMembersFilter::Search(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            SupergroupMembersFilter::Administrators(t) => t.client_id(),
            SupergroupMembersFilter::Banned(t) => t.client_id(),
            SupergroupMembersFilter::Bots(t) => t.client_id(),
            SupergroupMembersFilter::Contacts(t) => t.client_id(),
            SupergroupMembersFilter::Mention(t) => t.client_id(),
            SupergroupMembersFilter::Recent(t) => t.client_id(),
            SupergroupMembersFilter::Restricted(t) => t.client_id(),
            SupergroupMembersFilter::Search(t) => t.client_id(),

            _ => None,
        }
    }
}

impl SupergroupMembersFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, SupergroupMembersFilter::_Default)
    }
}

impl AsRef<SupergroupMembersFilter> for SupergroupMembersFilter {
    fn as_ref(&self) -> &SupergroupMembersFilter {
        self
    }
}

/// Returns the owner and administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterAdministrators {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SupergroupMembersFilterAdministrators {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterAdministrators {}

impl SupergroupMembersFilterAdministrators {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterAdministratorsBuilder {
        let mut inner = SupergroupMembersFilterAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterAdministratorsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterAdministratorsBuilder {
    inner: SupergroupMembersFilterAdministrators,
}

#[deprecated]
pub type RTDSupergroupMembersFilterAdministratorsBuilder =
    SupergroupMembersFilterAdministratorsBuilder;

impl SupergroupMembersFilterAdministratorsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterAdministrators {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterAdministrators> for SupergroupMembersFilterAdministrators {
    fn as_ref(&self) -> &SupergroupMembersFilterAdministrators {
        self
    }
}

impl AsRef<SupergroupMembersFilterAdministrators> for SupergroupMembersFilterAdministratorsBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterAdministrators {
        &self.inner
    }
}

/// Returns users banned from the supergroup or channel; can be used only by administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterBanned {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for

    #[serde(default)]
    query: String,
}

impl RObject for SupergroupMembersFilterBanned {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterBanned {}

impl SupergroupMembersFilterBanned {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterBannedBuilder {
        let mut inner = SupergroupMembersFilterBanned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterBannedBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterBannedBuilder {
    inner: SupergroupMembersFilterBanned,
}

#[deprecated]
pub type RTDSupergroupMembersFilterBannedBuilder = SupergroupMembersFilterBannedBuilder;

impl SupergroupMembersFilterBannedBuilder {
    pub fn build(&self) -> SupergroupMembersFilterBanned {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterBanned> for SupergroupMembersFilterBanned {
    fn as_ref(&self) -> &SupergroupMembersFilterBanned {
        self
    }
}

impl AsRef<SupergroupMembersFilterBanned> for SupergroupMembersFilterBannedBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterBanned {
        &self.inner
    }
}

/// Returns bot members of the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterBots {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SupergroupMembersFilterBots {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterBots {}

impl SupergroupMembersFilterBots {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterBotsBuilder {
        let mut inner = SupergroupMembersFilterBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterBotsBuilder {
    inner: SupergroupMembersFilterBots,
}

#[deprecated]
pub type RTDSupergroupMembersFilterBotsBuilder = SupergroupMembersFilterBotsBuilder;

impl SupergroupMembersFilterBotsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterBots {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterBots> for SupergroupMembersFilterBots {
    fn as_ref(&self) -> &SupergroupMembersFilterBots {
        self
    }
}

impl AsRef<SupergroupMembersFilterBots> for SupergroupMembersFilterBotsBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterBots {
        &self.inner
    }
}

/// Returns contacts of the user, which are members of the supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterContacts {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for

    #[serde(default)]
    query: String,
}

impl RObject for SupergroupMembersFilterContacts {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterContacts {}

impl SupergroupMembersFilterContacts {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterContactsBuilder {
        let mut inner = SupergroupMembersFilterContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterContactsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterContactsBuilder {
    inner: SupergroupMembersFilterContacts,
}

#[deprecated]
pub type RTDSupergroupMembersFilterContactsBuilder = SupergroupMembersFilterContactsBuilder;

impl SupergroupMembersFilterContactsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterContacts {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterContacts> for SupergroupMembersFilterContacts {
    fn as_ref(&self) -> &SupergroupMembersFilterContacts {
        self
    }
}

impl AsRef<SupergroupMembersFilterContacts> for SupergroupMembersFilterContactsBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterContacts {
        &self.inner
    }
}

/// Returns users which can be mentioned in the supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterMention {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for

    #[serde(default)]
    query: String,
    /// If non-zero, the identifier of the current message thread

    #[serde(default)]
    message_thread_id: i64,
}

impl RObject for SupergroupMembersFilterMention {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterMention {}

impl SupergroupMembersFilterMention {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterMentionBuilder {
        let mut inner = SupergroupMembersFilterMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterMentionBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterMentionBuilder {
    inner: SupergroupMembersFilterMention,
}

#[deprecated]
pub type RTDSupergroupMembersFilterMentionBuilder = SupergroupMembersFilterMentionBuilder;

impl SupergroupMembersFilterMentionBuilder {
    pub fn build(&self) -> SupergroupMembersFilterMention {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }

    pub fn message_thread_id(&mut self, message_thread_id: i64) -> &mut Self {
        self.inner.message_thread_id = message_thread_id;
        self
    }
}

impl AsRef<SupergroupMembersFilterMention> for SupergroupMembersFilterMention {
    fn as_ref(&self) -> &SupergroupMembersFilterMention {
        self
    }
}

impl AsRef<SupergroupMembersFilterMention> for SupergroupMembersFilterMentionBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterMention {
        &self.inner
    }
}

/// Returns recently active users in reverse chronological order
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterRecent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for SupergroupMembersFilterRecent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterRecent {}

impl SupergroupMembersFilterRecent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterRecentBuilder {
        let mut inner = SupergroupMembersFilterRecent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterRecentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterRecentBuilder {
    inner: SupergroupMembersFilterRecent,
}

#[deprecated]
pub type RTDSupergroupMembersFilterRecentBuilder = SupergroupMembersFilterRecentBuilder;

impl SupergroupMembersFilterRecentBuilder {
    pub fn build(&self) -> SupergroupMembersFilterRecent {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterRecent> for SupergroupMembersFilterRecent {
    fn as_ref(&self) -> &SupergroupMembersFilterRecent {
        self
    }
}

impl AsRef<SupergroupMembersFilterRecent> for SupergroupMembersFilterRecentBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterRecent {
        &self.inner
    }
}

/// Returns restricted supergroup members; can be used only by administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterRestricted {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for

    #[serde(default)]
    query: String,
}

impl RObject for SupergroupMembersFilterRestricted {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterRestricted {}

impl SupergroupMembersFilterRestricted {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterRestrictedBuilder {
        let mut inner = SupergroupMembersFilterRestricted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterRestrictedBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterRestrictedBuilder {
    inner: SupergroupMembersFilterRestricted,
}

#[deprecated]
pub type RTDSupergroupMembersFilterRestrictedBuilder = SupergroupMembersFilterRestrictedBuilder;

impl SupergroupMembersFilterRestrictedBuilder {
    pub fn build(&self) -> SupergroupMembersFilterRestricted {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterRestricted> for SupergroupMembersFilterRestricted {
    fn as_ref(&self) -> &SupergroupMembersFilterRestricted {
        self
    }
}

impl AsRef<SupergroupMembersFilterRestricted> for SupergroupMembersFilterRestrictedBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterRestricted {
        &self.inner
    }
}

/// Used to search for supergroup or channel members via a (string) query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SupergroupMembersFilterSearch {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Query to search for

    #[serde(default)]
    query: String,
}

impl RObject for SupergroupMembersFilterSearch {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDSupergroupMembersFilter for SupergroupMembersFilterSearch {}

impl SupergroupMembersFilterSearch {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SupergroupMembersFilterSearchBuilder {
        let mut inner = SupergroupMembersFilterSearch::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        SupergroupMembersFilterSearchBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct SupergroupMembersFilterSearchBuilder {
    inner: SupergroupMembersFilterSearch,
}

#[deprecated]
pub type RTDSupergroupMembersFilterSearchBuilder = SupergroupMembersFilterSearchBuilder;

impl SupergroupMembersFilterSearchBuilder {
    pub fn build(&self) -> SupergroupMembersFilterSearch {
        self.inner.clone()
    }

    pub fn query<T: AsRef<str>>(&mut self, query: T) -> &mut Self {
        self.inner.query = query.as_ref().to_string();
        self
    }
}

impl AsRef<SupergroupMembersFilterSearch> for SupergroupMembersFilterSearch {
    fn as_ref(&self) -> &SupergroupMembersFilterSearch {
        self
    }
}

impl AsRef<SupergroupMembersFilterSearch> for SupergroupMembersFilterSearchBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterSearch {
        &self.inner
    }
}
