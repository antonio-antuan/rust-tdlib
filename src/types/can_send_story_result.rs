use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents result of checking whether the current user can send a story in the specific chat
pub trait TDCanSendStoryResult: Debug + RObject {}

/// Represents result of checking whether the current user can send a story in the specific chat
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum CanSendStoryResult {
    #[doc(hidden)]
    #[default]
    _Default,
    /// Checks whether the current user can send a story on behalf of a chat; requires can_post_stories rights for channel chats
    #[serde(rename = "canSendStory")]
    CanSendStory(CanSendStory),
    /// The limit for the number of active stories exceeded. The user can buy Telegram Premium, delete an active story, or wait for the oldest story to expire
    #[serde(rename = "canSendStoryResultActiveStoryLimitExceeded")]
    ActiveStoryLimitExceeded(CanSendStoryResultActiveStoryLimitExceeded),
    /// The channel chat must be boosted first by Telegram Premium subscribers to post more stories. Call getChatBoostStatus to get current boost status of the chat
    #[serde(rename = "canSendStoryResultBoostNeeded")]
    BoostNeeded(CanSendStoryResultBoostNeeded),
    /// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
    #[serde(rename = "canSendStoryResultMonthlyLimitExceeded")]
    MonthlyLimitExceeded(CanSendStoryResultMonthlyLimitExceeded),
    /// A story can be sent
    #[serde(rename = "canSendStoryResultOk")]
    Ok(CanSendStoryResultOk),
    /// The user must subscribe to Telegram Premium to be able to post stories
    #[serde(rename = "canSendStoryResultPremiumNeeded")]
    PremiumNeeded(CanSendStoryResultPremiumNeeded),
    /// The weekly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
    #[serde(rename = "canSendStoryResultWeeklyLimitExceeded")]
    WeeklyLimitExceeded(CanSendStoryResultWeeklyLimitExceeded),
}

impl RObject for CanSendStoryResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CanSendStoryResult::CanSendStory(t) => t.extra(),
            CanSendStoryResult::ActiveStoryLimitExceeded(t) => t.extra(),
            CanSendStoryResult::BoostNeeded(t) => t.extra(),
            CanSendStoryResult::MonthlyLimitExceeded(t) => t.extra(),
            CanSendStoryResult::Ok(t) => t.extra(),
            CanSendStoryResult::PremiumNeeded(t) => t.extra(),
            CanSendStoryResult::WeeklyLimitExceeded(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CanSendStoryResult::CanSendStory(t) => t.client_id(),
            CanSendStoryResult::ActiveStoryLimitExceeded(t) => t.client_id(),
            CanSendStoryResult::BoostNeeded(t) => t.client_id(),
            CanSendStoryResult::MonthlyLimitExceeded(t) => t.client_id(),
            CanSendStoryResult::Ok(t) => t.client_id(),
            CanSendStoryResult::PremiumNeeded(t) => t.client_id(),
            CanSendStoryResult::WeeklyLimitExceeded(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CanSendStoryResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CanSendStoryResult::_Default)
    }
}

impl AsRef<CanSendStoryResult> for CanSendStoryResult {
    fn as_ref(&self) -> &CanSendStoryResult {
        self
    }
}

/// The limit for the number of active stories exceeded. The user can buy Telegram Premium, delete an active story, or wait for the oldest story to expire
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendStoryResultActiveStoryLimitExceeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CanSendStoryResultActiveStoryLimitExceeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanSendStoryResult for CanSendStoryResultActiveStoryLimitExceeded {}

impl CanSendStoryResultActiveStoryLimitExceeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanSendStoryResultActiveStoryLimitExceededBuilder {
        let mut inner = CanSendStoryResultActiveStoryLimitExceeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanSendStoryResultActiveStoryLimitExceededBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CanSendStoryResultActiveStoryLimitExceededBuilder {
    inner: CanSendStoryResultActiveStoryLimitExceeded,
}

#[deprecated]
pub type RTDCanSendStoryResultActiveStoryLimitExceededBuilder =
    CanSendStoryResultActiveStoryLimitExceededBuilder;

impl CanSendStoryResultActiveStoryLimitExceededBuilder {
    pub fn build(&self) -> CanSendStoryResultActiveStoryLimitExceeded {
        self.inner.clone()
    }
}

impl AsRef<CanSendStoryResultActiveStoryLimitExceeded>
    for CanSendStoryResultActiveStoryLimitExceeded
{
    fn as_ref(&self) -> &CanSendStoryResultActiveStoryLimitExceeded {
        self
    }
}

impl AsRef<CanSendStoryResultActiveStoryLimitExceeded>
    for CanSendStoryResultActiveStoryLimitExceededBuilder
{
    fn as_ref(&self) -> &CanSendStoryResultActiveStoryLimitExceeded {
        &self.inner
    }
}

/// The channel chat must be boosted first by Telegram Premium subscribers to post more stories. Call getChatBoostStatus to get current boost status of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendStoryResultBoostNeeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CanSendStoryResultBoostNeeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanSendStoryResult for CanSendStoryResultBoostNeeded {}

impl CanSendStoryResultBoostNeeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanSendStoryResultBoostNeededBuilder {
        let mut inner = CanSendStoryResultBoostNeeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanSendStoryResultBoostNeededBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CanSendStoryResultBoostNeededBuilder {
    inner: CanSendStoryResultBoostNeeded,
}

#[deprecated]
pub type RTDCanSendStoryResultBoostNeededBuilder = CanSendStoryResultBoostNeededBuilder;

impl CanSendStoryResultBoostNeededBuilder {
    pub fn build(&self) -> CanSendStoryResultBoostNeeded {
        self.inner.clone()
    }
}

impl AsRef<CanSendStoryResultBoostNeeded> for CanSendStoryResultBoostNeeded {
    fn as_ref(&self) -> &CanSendStoryResultBoostNeeded {
        self
    }
}

impl AsRef<CanSendStoryResultBoostNeeded> for CanSendStoryResultBoostNeededBuilder {
    fn as_ref(&self) -> &CanSendStoryResultBoostNeeded {
        &self.inner
    }
}

/// The monthly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendStoryResultMonthlyLimitExceeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Time left before the user can send the next story

    #[serde(default)]
    retry_after: i32,
}

impl RObject for CanSendStoryResultMonthlyLimitExceeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanSendStoryResult for CanSendStoryResultMonthlyLimitExceeded {}

impl CanSendStoryResultMonthlyLimitExceeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanSendStoryResultMonthlyLimitExceededBuilder {
        let mut inner = CanSendStoryResultMonthlyLimitExceeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanSendStoryResultMonthlyLimitExceededBuilder { inner }
    }

    pub fn retry_after(&self) -> i32 {
        self.retry_after
    }
}

#[doc(hidden)]
pub struct CanSendStoryResultMonthlyLimitExceededBuilder {
    inner: CanSendStoryResultMonthlyLimitExceeded,
}

#[deprecated]
pub type RTDCanSendStoryResultMonthlyLimitExceededBuilder =
    CanSendStoryResultMonthlyLimitExceededBuilder;

impl CanSendStoryResultMonthlyLimitExceededBuilder {
    pub fn build(&self) -> CanSendStoryResultMonthlyLimitExceeded {
        self.inner.clone()
    }

    pub fn retry_after(&mut self, retry_after: i32) -> &mut Self {
        self.inner.retry_after = retry_after;
        self
    }
}

impl AsRef<CanSendStoryResultMonthlyLimitExceeded> for CanSendStoryResultMonthlyLimitExceeded {
    fn as_ref(&self) -> &CanSendStoryResultMonthlyLimitExceeded {
        self
    }
}

impl AsRef<CanSendStoryResultMonthlyLimitExceeded>
    for CanSendStoryResultMonthlyLimitExceededBuilder
{
    fn as_ref(&self) -> &CanSendStoryResultMonthlyLimitExceeded {
        &self.inner
    }
}

/// A story can be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendStoryResultOk {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CanSendStoryResultOk {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanSendStoryResult for CanSendStoryResultOk {}

impl CanSendStoryResultOk {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanSendStoryResultOkBuilder {
        let mut inner = CanSendStoryResultOk::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanSendStoryResultOkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CanSendStoryResultOkBuilder {
    inner: CanSendStoryResultOk,
}

#[deprecated]
pub type RTDCanSendStoryResultOkBuilder = CanSendStoryResultOkBuilder;

impl CanSendStoryResultOkBuilder {
    pub fn build(&self) -> CanSendStoryResultOk {
        self.inner.clone()
    }
}

impl AsRef<CanSendStoryResultOk> for CanSendStoryResultOk {
    fn as_ref(&self) -> &CanSendStoryResultOk {
        self
    }
}

impl AsRef<CanSendStoryResultOk> for CanSendStoryResultOkBuilder {
    fn as_ref(&self) -> &CanSendStoryResultOk {
        &self.inner
    }
}

/// The user must subscribe to Telegram Premium to be able to post stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendStoryResultPremiumNeeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CanSendStoryResultPremiumNeeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanSendStoryResult for CanSendStoryResultPremiumNeeded {}

impl CanSendStoryResultPremiumNeeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanSendStoryResultPremiumNeededBuilder {
        let mut inner = CanSendStoryResultPremiumNeeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanSendStoryResultPremiumNeededBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CanSendStoryResultPremiumNeededBuilder {
    inner: CanSendStoryResultPremiumNeeded,
}

#[deprecated]
pub type RTDCanSendStoryResultPremiumNeededBuilder = CanSendStoryResultPremiumNeededBuilder;

impl CanSendStoryResultPremiumNeededBuilder {
    pub fn build(&self) -> CanSendStoryResultPremiumNeeded {
        self.inner.clone()
    }
}

impl AsRef<CanSendStoryResultPremiumNeeded> for CanSendStoryResultPremiumNeeded {
    fn as_ref(&self) -> &CanSendStoryResultPremiumNeeded {
        self
    }
}

impl AsRef<CanSendStoryResultPremiumNeeded> for CanSendStoryResultPremiumNeededBuilder {
    fn as_ref(&self) -> &CanSendStoryResultPremiumNeeded {
        &self.inner
    }
}

/// The weekly limit for the number of posted stories exceeded. The user needs to buy Telegram Premium or wait specified time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanSendStoryResultWeeklyLimitExceeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Time left before the user can send the next story

    #[serde(default)]
    retry_after: i32,
}

impl RObject for CanSendStoryResultWeeklyLimitExceeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanSendStoryResult for CanSendStoryResultWeeklyLimitExceeded {}

impl CanSendStoryResultWeeklyLimitExceeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanSendStoryResultWeeklyLimitExceededBuilder {
        let mut inner = CanSendStoryResultWeeklyLimitExceeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CanSendStoryResultWeeklyLimitExceededBuilder { inner }
    }

    pub fn retry_after(&self) -> i32 {
        self.retry_after
    }
}

#[doc(hidden)]
pub struct CanSendStoryResultWeeklyLimitExceededBuilder {
    inner: CanSendStoryResultWeeklyLimitExceeded,
}

#[deprecated]
pub type RTDCanSendStoryResultWeeklyLimitExceededBuilder =
    CanSendStoryResultWeeklyLimitExceededBuilder;

impl CanSendStoryResultWeeklyLimitExceededBuilder {
    pub fn build(&self) -> CanSendStoryResultWeeklyLimitExceeded {
        self.inner.clone()
    }

    pub fn retry_after(&mut self, retry_after: i32) -> &mut Self {
        self.inner.retry_after = retry_after;
        self
    }
}

impl AsRef<CanSendStoryResultWeeklyLimitExceeded> for CanSendStoryResultWeeklyLimitExceeded {
    fn as_ref(&self) -> &CanSendStoryResultWeeklyLimitExceeded {
        self
    }
}

impl AsRef<CanSendStoryResultWeeklyLimitExceeded> for CanSendStoryResultWeeklyLimitExceededBuilder {
    fn as_ref(&self) -> &CanSendStoryResultWeeklyLimitExceeded {
        &self.inner
    }
}
