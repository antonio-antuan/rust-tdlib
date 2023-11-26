use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a source from which the Premium features screen is opened
pub trait TDPremiumSource: Debug + RObject {}

/// Describes a source from which the Premium features screen is opened
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum PremiumSource {
    #[doc(hidden)]
    #[default]
    _Default,
    /// A user tried to use a Premium feature
    #[serde(rename = "premiumSourceFeature")]
    Feature(PremiumSourceFeature),
    /// A limit was exceeded
    #[serde(rename = "premiumSourceLimitExceeded")]
    LimitExceeded(PremiumSourceLimitExceeded),
    /// A user opened an internal link of the type internalLinkTypePremiumFeatures
    #[serde(rename = "premiumSourceLink")]
    Link(PremiumSourceLink),
    /// A user opened the Premium features screen from settings
    #[serde(rename = "premiumSourceSettings")]
    Settings(PremiumSourceSettings),
    /// A user tried to use a Premium story feature
    #[serde(rename = "premiumSourceStoryFeature")]
    StoryFeature(PremiumSourceStoryFeature),
}

impl RObject for PremiumSource {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumSource::Feature(t) => t.extra(),
            PremiumSource::LimitExceeded(t) => t.extra(),
            PremiumSource::Link(t) => t.extra(),
            PremiumSource::Settings(t) => t.extra(),
            PremiumSource::StoryFeature(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumSource::Feature(t) => t.client_id(),
            PremiumSource::LimitExceeded(t) => t.client_id(),
            PremiumSource::Link(t) => t.client_id(),
            PremiumSource::Settings(t) => t.client_id(),
            PremiumSource::StoryFeature(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PremiumSource::_Default)
    }
}

impl AsRef<PremiumSource> for PremiumSource {
    fn as_ref(&self) -> &PremiumSource {
        self
    }
}

/// A user tried to use a Premium feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceFeature {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The used feature

    #[serde(skip_serializing_if = "PremiumFeature::_is_default")]
    feature: PremiumFeature,
}

impl RObject for PremiumSourceFeature {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumSource for PremiumSourceFeature {}

impl PremiumSourceFeature {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumSourceFeatureBuilder {
        let mut inner = PremiumSourceFeature::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumSourceFeatureBuilder { inner }
    }

    pub fn feature(&self) -> &PremiumFeature {
        &self.feature
    }
}

#[doc(hidden)]
pub struct PremiumSourceFeatureBuilder {
    inner: PremiumSourceFeature,
}

#[deprecated]
pub type RTDPremiumSourceFeatureBuilder = PremiumSourceFeatureBuilder;

impl PremiumSourceFeatureBuilder {
    pub fn build(&self) -> PremiumSourceFeature {
        self.inner.clone()
    }

    pub fn feature<T: AsRef<PremiumFeature>>(&mut self, feature: T) -> &mut Self {
        self.inner.feature = feature.as_ref().clone();
        self
    }
}

impl AsRef<PremiumSourceFeature> for PremiumSourceFeature {
    fn as_ref(&self) -> &PremiumSourceFeature {
        self
    }
}

impl AsRef<PremiumSourceFeature> for PremiumSourceFeatureBuilder {
    fn as_ref(&self) -> &PremiumSourceFeature {
        &self.inner
    }
}

/// A limit was exceeded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceLimitExceeded {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Type of the exceeded limit

    #[serde(skip_serializing_if = "PremiumLimitType::_is_default")]
    limit_type: PremiumLimitType,
}

impl RObject for PremiumSourceLimitExceeded {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumSource for PremiumSourceLimitExceeded {}

impl PremiumSourceLimitExceeded {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumSourceLimitExceededBuilder {
        let mut inner = PremiumSourceLimitExceeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumSourceLimitExceededBuilder { inner }
    }

    pub fn limit_type(&self) -> &PremiumLimitType {
        &self.limit_type
    }
}

#[doc(hidden)]
pub struct PremiumSourceLimitExceededBuilder {
    inner: PremiumSourceLimitExceeded,
}

#[deprecated]
pub type RTDPremiumSourceLimitExceededBuilder = PremiumSourceLimitExceededBuilder;

impl PremiumSourceLimitExceededBuilder {
    pub fn build(&self) -> PremiumSourceLimitExceeded {
        self.inner.clone()
    }

    pub fn limit_type<T: AsRef<PremiumLimitType>>(&mut self, limit_type: T) -> &mut Self {
        self.inner.limit_type = limit_type.as_ref().clone();
        self
    }
}

impl AsRef<PremiumSourceLimitExceeded> for PremiumSourceLimitExceeded {
    fn as_ref(&self) -> &PremiumSourceLimitExceeded {
        self
    }
}

impl AsRef<PremiumSourceLimitExceeded> for PremiumSourceLimitExceededBuilder {
    fn as_ref(&self) -> &PremiumSourceLimitExceeded {
        &self.inner
    }
}

/// A user opened an internal link of the type internalLinkTypePremiumFeatures
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceLink {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The referrer from the link

    #[serde(default)]
    referrer: String,
}

impl RObject for PremiumSourceLink {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumSource for PremiumSourceLink {}

impl PremiumSourceLink {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumSourceLinkBuilder {
        let mut inner = PremiumSourceLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumSourceLinkBuilder { inner }
    }

    pub fn referrer(&self) -> &String {
        &self.referrer
    }
}

#[doc(hidden)]
pub struct PremiumSourceLinkBuilder {
    inner: PremiumSourceLink,
}

#[deprecated]
pub type RTDPremiumSourceLinkBuilder = PremiumSourceLinkBuilder;

impl PremiumSourceLinkBuilder {
    pub fn build(&self) -> PremiumSourceLink {
        self.inner.clone()
    }

    pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
        self.inner.referrer = referrer.as_ref().to_string();
        self
    }
}

impl AsRef<PremiumSourceLink> for PremiumSourceLink {
    fn as_ref(&self) -> &PremiumSourceLink {
        self
    }
}

impl AsRef<PremiumSourceLink> for PremiumSourceLinkBuilder {
    fn as_ref(&self) -> &PremiumSourceLink {
        &self.inner
    }
}

/// A user opened the Premium features screen from settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceSettings {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumSourceSettings {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumSource for PremiumSourceSettings {}

impl PremiumSourceSettings {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumSourceSettingsBuilder {
        let mut inner = PremiumSourceSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumSourceSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumSourceSettingsBuilder {
    inner: PremiumSourceSettings,
}

#[deprecated]
pub type RTDPremiumSourceSettingsBuilder = PremiumSourceSettingsBuilder;

impl PremiumSourceSettingsBuilder {
    pub fn build(&self) -> PremiumSourceSettings {
        self.inner.clone()
    }
}

impl AsRef<PremiumSourceSettings> for PremiumSourceSettings {
    fn as_ref(&self) -> &PremiumSourceSettings {
        self
    }
}

impl AsRef<PremiumSourceSettings> for PremiumSourceSettingsBuilder {
    fn as_ref(&self) -> &PremiumSourceSettings {
        &self.inner
    }
}

/// A user tried to use a Premium story feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceStoryFeature {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The used feature

    #[serde(skip_serializing_if = "PremiumStoryFeature::_is_default")]
    feature: PremiumStoryFeature,
}

impl RObject for PremiumSourceStoryFeature {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumSource for PremiumSourceStoryFeature {}

impl PremiumSourceStoryFeature {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumSourceStoryFeatureBuilder {
        let mut inner = PremiumSourceStoryFeature::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumSourceStoryFeatureBuilder { inner }
    }

    pub fn feature(&self) -> &PremiumStoryFeature {
        &self.feature
    }
}

#[doc(hidden)]
pub struct PremiumSourceStoryFeatureBuilder {
    inner: PremiumSourceStoryFeature,
}

#[deprecated]
pub type RTDPremiumSourceStoryFeatureBuilder = PremiumSourceStoryFeatureBuilder;

impl PremiumSourceStoryFeatureBuilder {
    pub fn build(&self) -> PremiumSourceStoryFeature {
        self.inner.clone()
    }

    pub fn feature<T: AsRef<PremiumStoryFeature>>(&mut self, feature: T) -> &mut Self {
        self.inner.feature = feature.as_ref().clone();
        self
    }
}

impl AsRef<PremiumSourceStoryFeature> for PremiumSourceStoryFeature {
    fn as_ref(&self) -> &PremiumSourceStoryFeature {
        self
    }
}

impl AsRef<PremiumSourceStoryFeature> for PremiumSourceStoryFeatureBuilder {
    fn as_ref(&self) -> &PremiumSourceStoryFeature {
        &self.inner
    }
}
