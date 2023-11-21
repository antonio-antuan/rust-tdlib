use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a story feature available to Premium users
pub trait TDPremiumStoryFeature: Debug + RObject {}

/// Describes a story feature available to Premium users
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum PremiumStoryFeature {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The ability to set custom expiration duration for stories
    #[serde(rename = "premiumStoryFeatureCustomExpirationDuration")]
    CustomExpirationDuration(PremiumStoryFeatureCustomExpirationDuration),
    /// The ability to use links and formatting in story caption
    #[serde(rename = "premiumStoryFeatureLinksAndFormatting")]
    LinksAndFormatting(PremiumStoryFeatureLinksAndFormatting),
    /// The ability to check who opened the current user's stories after they expire
    #[serde(rename = "premiumStoryFeaturePermanentViewsHistory")]
    PermanentViewsHistory(PremiumStoryFeaturePermanentViewsHistory),
    /// User stories are displayed before stories of non-premium contacts and channels
    #[serde(rename = "premiumStoryFeaturePriorityOrder")]
    PriorityOrder(PremiumStoryFeaturePriorityOrder),
    /// The ability to save other's unprotected stories
    #[serde(rename = "premiumStoryFeatureSaveStories")]
    SaveStories(PremiumStoryFeatureSaveStories),
    /// The ability to hide the fact that the user viewed other's stories
    #[serde(rename = "premiumStoryFeatureStealthMode")]
    StealthMode(PremiumStoryFeatureStealthMode),
}

impl RObject for PremiumStoryFeature {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumStoryFeature::CustomExpirationDuration(t) => t.extra(),
            PremiumStoryFeature::LinksAndFormatting(t) => t.extra(),
            PremiumStoryFeature::PermanentViewsHistory(t) => t.extra(),
            PremiumStoryFeature::PriorityOrder(t) => t.extra(),
            PremiumStoryFeature::SaveStories(t) => t.extra(),
            PremiumStoryFeature::StealthMode(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumStoryFeature::CustomExpirationDuration(t) => t.client_id(),
            PremiumStoryFeature::LinksAndFormatting(t) => t.client_id(),
            PremiumStoryFeature::PermanentViewsHistory(t) => t.client_id(),
            PremiumStoryFeature::PriorityOrder(t) => t.client_id(),
            PremiumStoryFeature::SaveStories(t) => t.client_id(),
            PremiumStoryFeature::StealthMode(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumStoryFeature {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PremiumStoryFeature::_Default)
    }
}

impl AsRef<PremiumStoryFeature> for PremiumStoryFeature {
    fn as_ref(&self) -> &PremiumStoryFeature {
        self
    }
}

/// The ability to set custom expiration duration for stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureCustomExpirationDuration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumStoryFeatureCustomExpirationDuration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureCustomExpirationDuration {}

impl PremiumStoryFeatureCustomExpirationDuration {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStoryFeatureCustomExpirationDurationBuilder {
        let mut inner = PremiumStoryFeatureCustomExpirationDuration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStoryFeatureCustomExpirationDurationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumStoryFeatureCustomExpirationDurationBuilder {
    inner: PremiumStoryFeatureCustomExpirationDuration,
}

#[deprecated]
pub type RTDPremiumStoryFeatureCustomExpirationDurationBuilder =
    PremiumStoryFeatureCustomExpirationDurationBuilder;

impl PremiumStoryFeatureCustomExpirationDurationBuilder {
    pub fn build(&self) -> PremiumStoryFeatureCustomExpirationDuration {
        self.inner.clone()
    }
}

impl AsRef<PremiumStoryFeatureCustomExpirationDuration>
    for PremiumStoryFeatureCustomExpirationDuration
{
    fn as_ref(&self) -> &PremiumStoryFeatureCustomExpirationDuration {
        self
    }
}

impl AsRef<PremiumStoryFeatureCustomExpirationDuration>
    for PremiumStoryFeatureCustomExpirationDurationBuilder
{
    fn as_ref(&self) -> &PremiumStoryFeatureCustomExpirationDuration {
        &self.inner
    }
}

/// The ability to use links and formatting in story caption
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureLinksAndFormatting {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumStoryFeatureLinksAndFormatting {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureLinksAndFormatting {}

impl PremiumStoryFeatureLinksAndFormatting {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStoryFeatureLinksAndFormattingBuilder {
        let mut inner = PremiumStoryFeatureLinksAndFormatting::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStoryFeatureLinksAndFormattingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumStoryFeatureLinksAndFormattingBuilder {
    inner: PremiumStoryFeatureLinksAndFormatting,
}

#[deprecated]
pub type RTDPremiumStoryFeatureLinksAndFormattingBuilder =
    PremiumStoryFeatureLinksAndFormattingBuilder;

impl PremiumStoryFeatureLinksAndFormattingBuilder {
    pub fn build(&self) -> PremiumStoryFeatureLinksAndFormatting {
        self.inner.clone()
    }
}

impl AsRef<PremiumStoryFeatureLinksAndFormatting> for PremiumStoryFeatureLinksAndFormatting {
    fn as_ref(&self) -> &PremiumStoryFeatureLinksAndFormatting {
        self
    }
}

impl AsRef<PremiumStoryFeatureLinksAndFormatting> for PremiumStoryFeatureLinksAndFormattingBuilder {
    fn as_ref(&self) -> &PremiumStoryFeatureLinksAndFormatting {
        &self.inner
    }
}

/// The ability to check who opened the current user's stories after they expire
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeaturePermanentViewsHistory {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumStoryFeaturePermanentViewsHistory {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumStoryFeature for PremiumStoryFeaturePermanentViewsHistory {}

impl PremiumStoryFeaturePermanentViewsHistory {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStoryFeaturePermanentViewsHistoryBuilder {
        let mut inner = PremiumStoryFeaturePermanentViewsHistory::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStoryFeaturePermanentViewsHistoryBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumStoryFeaturePermanentViewsHistoryBuilder {
    inner: PremiumStoryFeaturePermanentViewsHistory,
}

#[deprecated]
pub type RTDPremiumStoryFeaturePermanentViewsHistoryBuilder =
    PremiumStoryFeaturePermanentViewsHistoryBuilder;

impl PremiumStoryFeaturePermanentViewsHistoryBuilder {
    pub fn build(&self) -> PremiumStoryFeaturePermanentViewsHistory {
        self.inner.clone()
    }
}

impl AsRef<PremiumStoryFeaturePermanentViewsHistory> for PremiumStoryFeaturePermanentViewsHistory {
    fn as_ref(&self) -> &PremiumStoryFeaturePermanentViewsHistory {
        self
    }
}

impl AsRef<PremiumStoryFeaturePermanentViewsHistory>
    for PremiumStoryFeaturePermanentViewsHistoryBuilder
{
    fn as_ref(&self) -> &PremiumStoryFeaturePermanentViewsHistory {
        &self.inner
    }
}

/// User stories are displayed before stories of non-premium contacts and channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeaturePriorityOrder {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumStoryFeaturePriorityOrder {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumStoryFeature for PremiumStoryFeaturePriorityOrder {}

impl PremiumStoryFeaturePriorityOrder {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStoryFeaturePriorityOrderBuilder {
        let mut inner = PremiumStoryFeaturePriorityOrder::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStoryFeaturePriorityOrderBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumStoryFeaturePriorityOrderBuilder {
    inner: PremiumStoryFeaturePriorityOrder,
}

#[deprecated]
pub type RTDPremiumStoryFeaturePriorityOrderBuilder = PremiumStoryFeaturePriorityOrderBuilder;

impl PremiumStoryFeaturePriorityOrderBuilder {
    pub fn build(&self) -> PremiumStoryFeaturePriorityOrder {
        self.inner.clone()
    }
}

impl AsRef<PremiumStoryFeaturePriorityOrder> for PremiumStoryFeaturePriorityOrder {
    fn as_ref(&self) -> &PremiumStoryFeaturePriorityOrder {
        self
    }
}

impl AsRef<PremiumStoryFeaturePriorityOrder> for PremiumStoryFeaturePriorityOrderBuilder {
    fn as_ref(&self) -> &PremiumStoryFeaturePriorityOrder {
        &self.inner
    }
}

/// The ability to save other's unprotected stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureSaveStories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumStoryFeatureSaveStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureSaveStories {}

impl PremiumStoryFeatureSaveStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStoryFeatureSaveStoriesBuilder {
        let mut inner = PremiumStoryFeatureSaveStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStoryFeatureSaveStoriesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumStoryFeatureSaveStoriesBuilder {
    inner: PremiumStoryFeatureSaveStories,
}

#[deprecated]
pub type RTDPremiumStoryFeatureSaveStoriesBuilder = PremiumStoryFeatureSaveStoriesBuilder;

impl PremiumStoryFeatureSaveStoriesBuilder {
    pub fn build(&self) -> PremiumStoryFeatureSaveStories {
        self.inner.clone()
    }
}

impl AsRef<PremiumStoryFeatureSaveStories> for PremiumStoryFeatureSaveStories {
    fn as_ref(&self) -> &PremiumStoryFeatureSaveStories {
        self
    }
}

impl AsRef<PremiumStoryFeatureSaveStories> for PremiumStoryFeatureSaveStoriesBuilder {
    fn as_ref(&self) -> &PremiumStoryFeatureSaveStories {
        &self.inner
    }
}

/// The ability to hide the fact that the user viewed other's stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureStealthMode {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumStoryFeatureStealthMode {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureStealthMode {}

impl PremiumStoryFeatureStealthMode {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumStoryFeatureStealthModeBuilder {
        let mut inner = PremiumStoryFeatureStealthMode::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumStoryFeatureStealthModeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumStoryFeatureStealthModeBuilder {
    inner: PremiumStoryFeatureStealthMode,
}

#[deprecated]
pub type RTDPremiumStoryFeatureStealthModeBuilder = PremiumStoryFeatureStealthModeBuilder;

impl PremiumStoryFeatureStealthModeBuilder {
    pub fn build(&self) -> PremiumStoryFeatureStealthMode {
        self.inner.clone()
    }
}

impl AsRef<PremiumStoryFeatureStealthMode> for PremiumStoryFeatureStealthMode {
    fn as_ref(&self) -> &PremiumStoryFeatureStealthMode {
        self
    }
}

impl AsRef<PremiumStoryFeatureStealthMode> for PremiumStoryFeatureStealthModeBuilder {
    fn as_ref(&self) -> &PremiumStoryFeatureStealthMode {
        &self.inner
    }
}
