use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents result of checking whether a username can be set for a chat
pub trait TDCheckChatUsernameResult: Debug + RObject {}

/// Represents result of checking whether a username can be set for a chat
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum CheckChatUsernameResult {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Checks whether a username can be set for a chat
    #[serde(rename = "checkChatUsername")]
    CheckChatUsername(CheckChatUsername),
    /// The username can be set
    #[serde(rename = "checkChatUsernameResultOk")]
    Ok(CheckChatUsernameResultOk),
    /// The user has too many chats with username, one of them must be made private first
    #[serde(rename = "checkChatUsernameResultPublicChatsTooMany")]
    PublicChatsTooMany(CheckChatUsernameResultPublicChatsTooMany),
    /// The user can't be a member of a public supergroup
    #[serde(rename = "checkChatUsernameResultPublicGroupsUnavailable")]
    PublicGroupsUnavailable(CheckChatUsernameResultPublicGroupsUnavailable),
    /// The username is invalid
    #[serde(rename = "checkChatUsernameResultUsernameInvalid")]
    UsernameInvalid(CheckChatUsernameResultUsernameInvalid),
    /// The username is occupied
    #[serde(rename = "checkChatUsernameResultUsernameOccupied")]
    UsernameOccupied(CheckChatUsernameResultUsernameOccupied),
    /// The username can be purchased at fragment.com
    #[serde(rename = "checkChatUsernameResultUsernamePurchasable")]
    UsernamePurchasable(CheckChatUsernameResultUsernamePurchasable),
}

impl RObject for CheckChatUsernameResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CheckChatUsernameResult::CheckChatUsername(t) => t.extra(),
            CheckChatUsernameResult::Ok(t) => t.extra(),
            CheckChatUsernameResult::PublicChatsTooMany(t) => t.extra(),
            CheckChatUsernameResult::PublicGroupsUnavailable(t) => t.extra(),
            CheckChatUsernameResult::UsernameInvalid(t) => t.extra(),
            CheckChatUsernameResult::UsernameOccupied(t) => t.extra(),
            CheckChatUsernameResult::UsernamePurchasable(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CheckChatUsernameResult::CheckChatUsername(t) => t.client_id(),
            CheckChatUsernameResult::Ok(t) => t.client_id(),
            CheckChatUsernameResult::PublicChatsTooMany(t) => t.client_id(),
            CheckChatUsernameResult::PublicGroupsUnavailable(t) => t.client_id(),
            CheckChatUsernameResult::UsernameInvalid(t) => t.client_id(),
            CheckChatUsernameResult::UsernameOccupied(t) => t.client_id(),
            CheckChatUsernameResult::UsernamePurchasable(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CheckChatUsernameResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CheckChatUsernameResult::_Default)
    }
}

impl AsRef<CheckChatUsernameResult> for CheckChatUsernameResult {
    fn as_ref(&self) -> &CheckChatUsernameResult {
        self
    }
}

/// The username can be set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultOk {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckChatUsernameResultOk {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultOk {}

impl CheckChatUsernameResultOk {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatUsernameResultOkBuilder {
        let mut inner = CheckChatUsernameResultOk::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckChatUsernameResultOkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckChatUsernameResultOkBuilder {
    inner: CheckChatUsernameResultOk,
}

#[deprecated]
pub type RTDCheckChatUsernameResultOkBuilder = CheckChatUsernameResultOkBuilder;

impl CheckChatUsernameResultOkBuilder {
    pub fn build(&self) -> CheckChatUsernameResultOk {
        self.inner.clone()
    }
}

impl AsRef<CheckChatUsernameResultOk> for CheckChatUsernameResultOk {
    fn as_ref(&self) -> &CheckChatUsernameResultOk {
        self
    }
}

impl AsRef<CheckChatUsernameResultOk> for CheckChatUsernameResultOkBuilder {
    fn as_ref(&self) -> &CheckChatUsernameResultOk {
        &self.inner
    }
}

/// The user has too many chats with username, one of them must be made private first
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicChatsTooMany {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckChatUsernameResultPublicChatsTooMany {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultPublicChatsTooMany {}

impl CheckChatUsernameResultPublicChatsTooMany {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatUsernameResultPublicChatsTooManyBuilder {
        let mut inner = CheckChatUsernameResultPublicChatsTooMany::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckChatUsernameResultPublicChatsTooManyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckChatUsernameResultPublicChatsTooManyBuilder {
    inner: CheckChatUsernameResultPublicChatsTooMany,
}

#[deprecated]
pub type RTDCheckChatUsernameResultPublicChatsTooManyBuilder =
    CheckChatUsernameResultPublicChatsTooManyBuilder;

impl CheckChatUsernameResultPublicChatsTooManyBuilder {
    pub fn build(&self) -> CheckChatUsernameResultPublicChatsTooMany {
        self.inner.clone()
    }
}

impl AsRef<CheckChatUsernameResultPublicChatsTooMany>
    for CheckChatUsernameResultPublicChatsTooMany
{
    fn as_ref(&self) -> &CheckChatUsernameResultPublicChatsTooMany {
        self
    }
}

impl AsRef<CheckChatUsernameResultPublicChatsTooMany>
    for CheckChatUsernameResultPublicChatsTooManyBuilder
{
    fn as_ref(&self) -> &CheckChatUsernameResultPublicChatsTooMany {
        &self.inner
    }
}

/// The user can't be a member of a public supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultPublicGroupsUnavailable {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckChatUsernameResultPublicGroupsUnavailable {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultPublicGroupsUnavailable {}

impl CheckChatUsernameResultPublicGroupsUnavailable {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatUsernameResultPublicGroupsUnavailableBuilder {
        let mut inner = CheckChatUsernameResultPublicGroupsUnavailable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckChatUsernameResultPublicGroupsUnavailableBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckChatUsernameResultPublicGroupsUnavailableBuilder {
    inner: CheckChatUsernameResultPublicGroupsUnavailable,
}

#[deprecated]
pub type RTDCheckChatUsernameResultPublicGroupsUnavailableBuilder =
    CheckChatUsernameResultPublicGroupsUnavailableBuilder;

impl CheckChatUsernameResultPublicGroupsUnavailableBuilder {
    pub fn build(&self) -> CheckChatUsernameResultPublicGroupsUnavailable {
        self.inner.clone()
    }
}

impl AsRef<CheckChatUsernameResultPublicGroupsUnavailable>
    for CheckChatUsernameResultPublicGroupsUnavailable
{
    fn as_ref(&self) -> &CheckChatUsernameResultPublicGroupsUnavailable {
        self
    }
}

impl AsRef<CheckChatUsernameResultPublicGroupsUnavailable>
    for CheckChatUsernameResultPublicGroupsUnavailableBuilder
{
    fn as_ref(&self) -> &CheckChatUsernameResultPublicGroupsUnavailable {
        &self.inner
    }
}

/// The username is invalid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameInvalid {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckChatUsernameResultUsernameInvalid {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernameInvalid {}

impl CheckChatUsernameResultUsernameInvalid {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatUsernameResultUsernameInvalidBuilder {
        let mut inner = CheckChatUsernameResultUsernameInvalid::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckChatUsernameResultUsernameInvalidBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckChatUsernameResultUsernameInvalidBuilder {
    inner: CheckChatUsernameResultUsernameInvalid,
}

#[deprecated]
pub type RTDCheckChatUsernameResultUsernameInvalidBuilder =
    CheckChatUsernameResultUsernameInvalidBuilder;

impl CheckChatUsernameResultUsernameInvalidBuilder {
    pub fn build(&self) -> CheckChatUsernameResultUsernameInvalid {
        self.inner.clone()
    }
}

impl AsRef<CheckChatUsernameResultUsernameInvalid> for CheckChatUsernameResultUsernameInvalid {
    fn as_ref(&self) -> &CheckChatUsernameResultUsernameInvalid {
        self
    }
}

impl AsRef<CheckChatUsernameResultUsernameInvalid>
    for CheckChatUsernameResultUsernameInvalidBuilder
{
    fn as_ref(&self) -> &CheckChatUsernameResultUsernameInvalid {
        &self.inner
    }
}

/// The username is occupied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernameOccupied {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckChatUsernameResultUsernameOccupied {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernameOccupied {}

impl CheckChatUsernameResultUsernameOccupied {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatUsernameResultUsernameOccupiedBuilder {
        let mut inner = CheckChatUsernameResultUsernameOccupied::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckChatUsernameResultUsernameOccupiedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckChatUsernameResultUsernameOccupiedBuilder {
    inner: CheckChatUsernameResultUsernameOccupied,
}

#[deprecated]
pub type RTDCheckChatUsernameResultUsernameOccupiedBuilder =
    CheckChatUsernameResultUsernameOccupiedBuilder;

impl CheckChatUsernameResultUsernameOccupiedBuilder {
    pub fn build(&self) -> CheckChatUsernameResultUsernameOccupied {
        self.inner.clone()
    }
}

impl AsRef<CheckChatUsernameResultUsernameOccupied> for CheckChatUsernameResultUsernameOccupied {
    fn as_ref(&self) -> &CheckChatUsernameResultUsernameOccupied {
        self
    }
}

impl AsRef<CheckChatUsernameResultUsernameOccupied>
    for CheckChatUsernameResultUsernameOccupiedBuilder
{
    fn as_ref(&self) -> &CheckChatUsernameResultUsernameOccupied {
        &self.inner
    }
}

/// The username can be purchased at fragment.com
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckChatUsernameResultUsernamePurchasable {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckChatUsernameResultUsernamePurchasable {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckChatUsernameResult for CheckChatUsernameResultUsernamePurchasable {}

impl CheckChatUsernameResultUsernamePurchasable {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckChatUsernameResultUsernamePurchasableBuilder {
        let mut inner = CheckChatUsernameResultUsernamePurchasable::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckChatUsernameResultUsernamePurchasableBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckChatUsernameResultUsernamePurchasableBuilder {
    inner: CheckChatUsernameResultUsernamePurchasable,
}

#[deprecated]
pub type RTDCheckChatUsernameResultUsernamePurchasableBuilder =
    CheckChatUsernameResultUsernamePurchasableBuilder;

impl CheckChatUsernameResultUsernamePurchasableBuilder {
    pub fn build(&self) -> CheckChatUsernameResultUsernamePurchasable {
        self.inner.clone()
    }
}

impl AsRef<CheckChatUsernameResultUsernamePurchasable>
    for CheckChatUsernameResultUsernamePurchasable
{
    fn as_ref(&self) -> &CheckChatUsernameResultUsernamePurchasable {
        self
    }
}

impl AsRef<CheckChatUsernameResultUsernamePurchasable>
    for CheckChatUsernameResultUsernamePurchasableBuilder
{
    fn as_ref(&self) -> &CheckChatUsernameResultUsernamePurchasable {
        &self.inner
    }
}
