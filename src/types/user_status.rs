use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the last time the user was online
pub trait TDUserStatus: Debug + RObject {}

/// Describes the last time the user was online
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum UserStatus {
    #[doc(hidden)]
    _Default,
    /// The user status was never changed
    #[serde(rename(serialize = "userStatusEmpty", deserialize = "userStatusEmpty"))]
    Empty(UserStatusEmpty),
    /// The user is offline, but was online last month
    #[serde(rename(serialize = "userStatusLastMonth", deserialize = "userStatusLastMonth"))]
    LastMonth(UserStatusLastMonth),
    /// The user is offline, but was online last week
    #[serde(rename(serialize = "userStatusLastWeek", deserialize = "userStatusLastWeek"))]
    LastWeek(UserStatusLastWeek),
    /// The user is offline
    #[serde(rename(serialize = "userStatusOffline", deserialize = "userStatusOffline"))]
    Offline(UserStatusOffline),
    /// The user is online
    #[serde(rename(serialize = "userStatusOnline", deserialize = "userStatusOnline"))]
    Online(UserStatusOnline),
    /// The user was online recently
    #[serde(rename(serialize = "userStatusRecently", deserialize = "userStatusRecently"))]
    Recently(UserStatusRecently),
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::_Default
    }
}

impl RObject for UserStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            UserStatus::Empty(t) => t.extra(),
            UserStatus::LastMonth(t) => t.extra(),
            UserStatus::LastWeek(t) => t.extra(),
            UserStatus::Offline(t) => t.extra(),
            UserStatus::Online(t) => t.extra(),
            UserStatus::Recently(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            UserStatus::Empty(t) => t.client_id(),
            UserStatus::LastMonth(t) => t.client_id(),
            UserStatus::LastWeek(t) => t.client_id(),
            UserStatus::Offline(t) => t.client_id(),
            UserStatus::Online(t) => t.client_id(),
            UserStatus::Recently(t) => t.client_id(),

            _ => None,
        }
    }
}

impl UserStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, UserStatus::_Default)
    }
}

impl AsRef<UserStatus> for UserStatus {
    fn as_ref(&self) -> &UserStatus {
        self
    }
}

/// The user status was never changed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserStatusEmpty {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserStatus for UserStatusEmpty {}

impl UserStatusEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserStatusEmptyBuilder {
        let mut inner = UserStatusEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserStatusEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserStatusEmptyBuilder {
    inner: UserStatusEmpty,
}

impl RTDUserStatusEmptyBuilder {
    pub fn build(&self) -> UserStatusEmpty {
        self.inner.clone()
    }
}

impl AsRef<UserStatusEmpty> for UserStatusEmpty {
    fn as_ref(&self) -> &UserStatusEmpty {
        self
    }
}

impl AsRef<UserStatusEmpty> for RTDUserStatusEmptyBuilder {
    fn as_ref(&self) -> &UserStatusEmpty {
        &self.inner
    }
}

/// The user is offline, but was online last month
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusLastMonth {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserStatusLastMonth {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserStatus for UserStatusLastMonth {}

impl UserStatusLastMonth {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserStatusLastMonthBuilder {
        let mut inner = UserStatusLastMonth::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserStatusLastMonthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserStatusLastMonthBuilder {
    inner: UserStatusLastMonth,
}

impl RTDUserStatusLastMonthBuilder {
    pub fn build(&self) -> UserStatusLastMonth {
        self.inner.clone()
    }
}

impl AsRef<UserStatusLastMonth> for UserStatusLastMonth {
    fn as_ref(&self) -> &UserStatusLastMonth {
        self
    }
}

impl AsRef<UserStatusLastMonth> for RTDUserStatusLastMonthBuilder {
    fn as_ref(&self) -> &UserStatusLastMonth {
        &self.inner
    }
}

/// The user is offline, but was online last week
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusLastWeek {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserStatusLastWeek {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserStatus for UserStatusLastWeek {}

impl UserStatusLastWeek {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserStatusLastWeekBuilder {
        let mut inner = UserStatusLastWeek::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserStatusLastWeekBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserStatusLastWeekBuilder {
    inner: UserStatusLastWeek,
}

impl RTDUserStatusLastWeekBuilder {
    pub fn build(&self) -> UserStatusLastWeek {
        self.inner.clone()
    }
}

impl AsRef<UserStatusLastWeek> for UserStatusLastWeek {
    fn as_ref(&self) -> &UserStatusLastWeek {
        self
    }
}

impl AsRef<UserStatusLastWeek> for RTDUserStatusLastWeekBuilder {
    fn as_ref(&self) -> &UserStatusLastWeek {
        &self.inner
    }
}

/// The user is offline
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusOffline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) when the user was last online
    was_online: i32,
}

impl RObject for UserStatusOffline {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserStatus for UserStatusOffline {}

impl UserStatusOffline {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserStatusOfflineBuilder {
        let mut inner = UserStatusOffline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserStatusOfflineBuilder { inner }
    }

    pub fn was_online(&self) -> i32 {
        self.was_online
    }
}

#[doc(hidden)]
pub struct RTDUserStatusOfflineBuilder {
    inner: UserStatusOffline,
}

impl RTDUserStatusOfflineBuilder {
    pub fn build(&self) -> UserStatusOffline {
        self.inner.clone()
    }

    pub fn was_online(&mut self, was_online: i32) -> &mut Self {
        self.inner.was_online = was_online;
        self
    }
}

impl AsRef<UserStatusOffline> for UserStatusOffline {
    fn as_ref(&self) -> &UserStatusOffline {
        self
    }
}

impl AsRef<UserStatusOffline> for RTDUserStatusOfflineBuilder {
    fn as_ref(&self) -> &UserStatusOffline {
        &self.inner
    }
}

/// The user is online
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusOnline {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) when the user's online status will expire
    expires: i32,
}

impl RObject for UserStatusOnline {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserStatus for UserStatusOnline {}

impl UserStatusOnline {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserStatusOnlineBuilder {
        let mut inner = UserStatusOnline::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserStatusOnlineBuilder { inner }
    }

    pub fn expires(&self) -> i32 {
        self.expires
    }
}

#[doc(hidden)]
pub struct RTDUserStatusOnlineBuilder {
    inner: UserStatusOnline,
}

impl RTDUserStatusOnlineBuilder {
    pub fn build(&self) -> UserStatusOnline {
        self.inner.clone()
    }

    pub fn expires(&mut self, expires: i32) -> &mut Self {
        self.inner.expires = expires;
        self
    }
}

impl AsRef<UserStatusOnline> for UserStatusOnline {
    fn as_ref(&self) -> &UserStatusOnline {
        self
    }
}

impl AsRef<UserStatusOnline> for RTDUserStatusOnlineBuilder {
    fn as_ref(&self) -> &UserStatusOnline {
        &self.inner
    }
}

/// The user was online recently
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserStatusRecently {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for UserStatusRecently {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUserStatus for UserStatusRecently {}

impl UserStatusRecently {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUserStatusRecentlyBuilder {
        let mut inner = UserStatusRecently::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDUserStatusRecentlyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDUserStatusRecentlyBuilder {
    inner: UserStatusRecently,
}

impl RTDUserStatusRecentlyBuilder {
    pub fn build(&self) -> UserStatusRecently {
        self.inner.clone()
    }
}

impl AsRef<UserStatusRecently> for UserStatusRecently {
    fn as_ref(&self) -> &UserStatusRecently {
        self
    }
}

impl AsRef<UserStatusRecently> for RTDUserStatusRecentlyBuilder {
    fn as_ref(&self) -> &UserStatusRecently {
        &self.inner
    }
}
