use crate::errors::*;
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
    #[serde(rename(
        serialize = "supergroupMembersFilterAdministrators",
        deserialize = "supergroupMembersFilterAdministrators"
    ))]
    Administrators(SupergroupMembersFilterAdministrators),
    /// Returns users banned from the supergroup or channel; can be used only by administrators
    #[serde(rename(
        serialize = "supergroupMembersFilterBanned",
        deserialize = "supergroupMembersFilterBanned"
    ))]
    Banned(SupergroupMembersFilterBanned),
    /// Returns bot members of the supergroup or channel
    #[serde(rename(
        serialize = "supergroupMembersFilterBots",
        deserialize = "supergroupMembersFilterBots"
    ))]
    Bots(SupergroupMembersFilterBots),
    /// Returns contacts of the user, which are members of the supergroup or channel
    #[serde(rename(
        serialize = "supergroupMembersFilterContacts",
        deserialize = "supergroupMembersFilterContacts"
    ))]
    Contacts(SupergroupMembersFilterContacts),
    /// Returns users which can be mentioned in the supergroup
    #[serde(rename(
        serialize = "supergroupMembersFilterMention",
        deserialize = "supergroupMembersFilterMention"
    ))]
    Mention(SupergroupMembersFilterMention),
    /// Returns recently active users in reverse chronological order
    #[serde(rename(
        serialize = "supergroupMembersFilterRecent",
        deserialize = "supergroupMembersFilterRecent"
    ))]
    Recent(SupergroupMembersFilterRecent),
    /// Returns restricted supergroup members; can be used only by administrators
    #[serde(rename(
        serialize = "supergroupMembersFilterRestricted",
        deserialize = "supergroupMembersFilterRestricted"
    ))]
    Restricted(SupergroupMembersFilterRestricted),
    /// Used to search for supergroup or channel members via a (string) query
    #[serde(rename(
        serialize = "supergroupMembersFilterSearch",
        deserialize = "supergroupMembersFilterSearch"
    ))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterAdministratorsBuilder {
        let mut inner = SupergroupMembersFilterAdministrators::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterAdministratorsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterAdministratorsBuilder {
    inner: SupergroupMembersFilterAdministrators,
}

impl RTDSupergroupMembersFilterAdministratorsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterAdministrators {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterAdministrators> for SupergroupMembersFilterAdministrators {
    fn as_ref(&self) -> &SupergroupMembersFilterAdministrators {
        self
    }
}

impl AsRef<SupergroupMembersFilterAdministrators>
    for RTDSupergroupMembersFilterAdministratorsBuilder
{
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterBannedBuilder {
        let mut inner = SupergroupMembersFilterBanned::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterBannedBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterBannedBuilder {
    inner: SupergroupMembersFilterBanned,
}

impl RTDSupergroupMembersFilterBannedBuilder {
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

impl AsRef<SupergroupMembersFilterBanned> for RTDSupergroupMembersFilterBannedBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterBotsBuilder {
        let mut inner = SupergroupMembersFilterBots::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterBotsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterBotsBuilder {
    inner: SupergroupMembersFilterBots,
}

impl RTDSupergroupMembersFilterBotsBuilder {
    pub fn build(&self) -> SupergroupMembersFilterBots {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterBots> for SupergroupMembersFilterBots {
    fn as_ref(&self) -> &SupergroupMembersFilterBots {
        self
    }
}

impl AsRef<SupergroupMembersFilterBots> for RTDSupergroupMembersFilterBotsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterContactsBuilder {
        let mut inner = SupergroupMembersFilterContacts::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterContactsBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterContactsBuilder {
    inner: SupergroupMembersFilterContacts,
}

impl RTDSupergroupMembersFilterContactsBuilder {
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

impl AsRef<SupergroupMembersFilterContacts> for RTDSupergroupMembersFilterContactsBuilder {
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
    query: String,
    /// If non-zero, the identifier of the current message thread
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterMentionBuilder {
        let mut inner = SupergroupMembersFilterMention::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterMentionBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }

    pub fn message_thread_id(&self) -> i64 {
        self.message_thread_id
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterMentionBuilder {
    inner: SupergroupMembersFilterMention,
}

impl RTDSupergroupMembersFilterMentionBuilder {
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

impl AsRef<SupergroupMembersFilterMention> for RTDSupergroupMembersFilterMentionBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterRecentBuilder {
        let mut inner = SupergroupMembersFilterRecent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterRecentBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterRecentBuilder {
    inner: SupergroupMembersFilterRecent,
}

impl RTDSupergroupMembersFilterRecentBuilder {
    pub fn build(&self) -> SupergroupMembersFilterRecent {
        self.inner.clone()
    }
}

impl AsRef<SupergroupMembersFilterRecent> for SupergroupMembersFilterRecent {
    fn as_ref(&self) -> &SupergroupMembersFilterRecent {
        self
    }
}

impl AsRef<SupergroupMembersFilterRecent> for RTDSupergroupMembersFilterRecentBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterRestrictedBuilder {
        let mut inner = SupergroupMembersFilterRestricted::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterRestrictedBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterRestrictedBuilder {
    inner: SupergroupMembersFilterRestricted,
}

impl RTDSupergroupMembersFilterRestrictedBuilder {
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

impl AsRef<SupergroupMembersFilterRestricted> for RTDSupergroupMembersFilterRestrictedBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSupergroupMembersFilterSearchBuilder {
        let mut inner = SupergroupMembersFilterSearch::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSupergroupMembersFilterSearchBuilder { inner }
    }

    pub fn query(&self) -> &String {
        &self.query
    }
}

#[doc(hidden)]
pub struct RTDSupergroupMembersFilterSearchBuilder {
    inner: SupergroupMembersFilterSearch,
}

impl RTDSupergroupMembersFilterSearchBuilder {
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

impl AsRef<SupergroupMembersFilterSearch> for RTDSupergroupMembersFilterSearchBuilder {
    fn as_ref(&self) -> &SupergroupMembersFilterSearch {
        &self.inner
    }
}
