use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a source from which the Premium features screen is opened
pub trait TDPremiumSource: Debug + RObject {}

/// Describes a source from which the Premium features screen is opened
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumSource {
    #[doc(hidden)]
    _Default,
    /// A user tried to use a Premium feature
    #[serde(rename(deserialize = "premiumSourceFeature"))]
    Feature(PremiumSourceFeature),
    /// A limit was exceeded
    #[serde(rename(deserialize = "premiumSourceLimitExceeded"))]
    LimitExceeded(PremiumSourceLimitExceeded),
    /// A user opened an internal link of the type internalLinkTypePremiumFeatures
    #[serde(rename(deserialize = "premiumSourceLink"))]
    Link(PremiumSourceLink),
    /// A user opened the Premium features screen from settings
    #[serde(rename(deserialize = "premiumSourceSettings"))]
    Settings(PremiumSourceSettings),
}

impl Default for PremiumSource {
    fn default() -> Self {
        PremiumSource::_Default
    }
}

impl RObject for PremiumSource {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumSource::Feature(t) => t.extra(),
            PremiumSource::LimitExceeded(t) => t.extra(),
            PremiumSource::Link(t) => t.extra(),
            PremiumSource::Settings(t) => t.extra(),

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

            _ => None,
        }
    }
}

impl PremiumSource {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumSourceFeatureBuilder {
        let mut inner = PremiumSourceFeature::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumSourceFeatureBuilder { inner }
    }

    pub fn feature(&self) -> &PremiumFeature {
        &self.feature
    }
}

#[doc(hidden)]
pub struct RTDPremiumSourceFeatureBuilder {
    inner: PremiumSourceFeature,
}

impl RTDPremiumSourceFeatureBuilder {
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

impl AsRef<PremiumSourceFeature> for RTDPremiumSourceFeatureBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumSourceLimitExceededBuilder {
        let mut inner = PremiumSourceLimitExceeded::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumSourceLimitExceededBuilder { inner }
    }

    pub fn limit_type(&self) -> &PremiumLimitType {
        &self.limit_type
    }
}

#[doc(hidden)]
pub struct RTDPremiumSourceLimitExceededBuilder {
    inner: PremiumSourceLimitExceeded,
}

impl RTDPremiumSourceLimitExceededBuilder {
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

impl AsRef<PremiumSourceLimitExceeded> for RTDPremiumSourceLimitExceededBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumSourceLinkBuilder {
        let mut inner = PremiumSourceLink::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumSourceLinkBuilder { inner }
    }

    pub fn referrer(&self) -> &String {
        &self.referrer
    }
}

#[doc(hidden)]
pub struct RTDPremiumSourceLinkBuilder {
    inner: PremiumSourceLink,
}

impl RTDPremiumSourceLinkBuilder {
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

impl AsRef<PremiumSourceLink> for RTDPremiumSourceLinkBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumSourceSettingsBuilder {
        let mut inner = PremiumSourceSettings::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumSourceSettingsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumSourceSettingsBuilder {
    inner: PremiumSourceSettings,
}

impl RTDPremiumSourceSettingsBuilder {
    pub fn build(&self) -> PremiumSourceSettings {
        self.inner.clone()
    }
}

impl AsRef<PremiumSourceSettings> for PremiumSourceSettings {
    fn as_ref(&self) -> &PremiumSourceSettings {
        self
    }
}

impl AsRef<PremiumSourceSettings> for RTDPremiumSourceSettingsBuilder {
    fn as_ref(&self) -> &PremiumSourceSettings {
        &self.inner
    }
}
