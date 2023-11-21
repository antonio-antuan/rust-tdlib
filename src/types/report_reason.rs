use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the reason why a chat is reported
pub trait TDReportReason: Debug + RObject {}

/// Describes the reason why a chat is reported
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum ReportReason {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The chat has child abuse related content
    #[serde(rename = "reportReasonChildAbuse")]
    ChildAbuse(ReportReasonChildAbuse),
    /// The chat contains copyrighted content
    #[serde(rename = "reportReasonCopyright")]
    Copyright(ReportReasonCopyright),
    /// A custom reason provided by the user
    #[serde(rename = "reportReasonCustom")]
    Custom(ReportReasonCustom),
    /// The chat represents a fake account
    #[serde(rename = "reportReasonFake")]
    Fake(ReportReasonFake),
    /// The chat has illegal drugs related content
    #[serde(rename = "reportReasonIllegalDrugs")]
    IllegalDrugs(ReportReasonIllegalDrugs),
    /// The chat contains messages with personal details
    #[serde(rename = "reportReasonPersonalDetails")]
    PersonalDetails(ReportReasonPersonalDetails),
    /// The chat contains pornographic messages
    #[serde(rename = "reportReasonPornography")]
    Pornography(ReportReasonPornography),
    /// The chat contains spam messages
    #[serde(rename = "reportReasonSpam")]
    Spam(ReportReasonSpam),
    /// The location-based chat is unrelated to its stated location
    #[serde(rename = "reportReasonUnrelatedLocation")]
    UnrelatedLocation(ReportReasonUnrelatedLocation),
    /// The chat promotes violence
    #[serde(rename = "reportReasonViolence")]
    Violence(ReportReasonViolence),
}

impl RObject for ReportReason {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ReportReason::ChildAbuse(t) => t.extra(),
            ReportReason::Copyright(t) => t.extra(),
            ReportReason::Custom(t) => t.extra(),
            ReportReason::Fake(t) => t.extra(),
            ReportReason::IllegalDrugs(t) => t.extra(),
            ReportReason::PersonalDetails(t) => t.extra(),
            ReportReason::Pornography(t) => t.extra(),
            ReportReason::Spam(t) => t.extra(),
            ReportReason::UnrelatedLocation(t) => t.extra(),
            ReportReason::Violence(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ReportReason::ChildAbuse(t) => t.client_id(),
            ReportReason::Copyright(t) => t.client_id(),
            ReportReason::Custom(t) => t.client_id(),
            ReportReason::Fake(t) => t.client_id(),
            ReportReason::IllegalDrugs(t) => t.client_id(),
            ReportReason::PersonalDetails(t) => t.client_id(),
            ReportReason::Pornography(t) => t.client_id(),
            ReportReason::Spam(t) => t.client_id(),
            ReportReason::UnrelatedLocation(t) => t.client_id(),
            ReportReason::Violence(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ReportReason {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ReportReason::_Default)
    }
}

impl AsRef<ReportReason> for ReportReason {
    fn as_ref(&self) -> &ReportReason {
        self
    }
}

/// The chat has child abuse related content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonChildAbuse {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonChildAbuse {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonChildAbuse {}

impl ReportReasonChildAbuse {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonChildAbuseBuilder {
        let mut inner = ReportReasonChildAbuse::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonChildAbuseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonChildAbuseBuilder {
    inner: ReportReasonChildAbuse,
}

#[deprecated]
pub type RTDReportReasonChildAbuseBuilder = ReportReasonChildAbuseBuilder;

impl ReportReasonChildAbuseBuilder {
    pub fn build(&self) -> ReportReasonChildAbuse {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonChildAbuse> for ReportReasonChildAbuse {
    fn as_ref(&self) -> &ReportReasonChildAbuse {
        self
    }
}

impl AsRef<ReportReasonChildAbuse> for ReportReasonChildAbuseBuilder {
    fn as_ref(&self) -> &ReportReasonChildAbuse {
        &self.inner
    }
}

/// The chat contains copyrighted content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonCopyright {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonCopyright {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonCopyright {}

impl ReportReasonCopyright {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonCopyrightBuilder {
        let mut inner = ReportReasonCopyright::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonCopyrightBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonCopyrightBuilder {
    inner: ReportReasonCopyright,
}

#[deprecated]
pub type RTDReportReasonCopyrightBuilder = ReportReasonCopyrightBuilder;

impl ReportReasonCopyrightBuilder {
    pub fn build(&self) -> ReportReasonCopyright {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonCopyright> for ReportReasonCopyright {
    fn as_ref(&self) -> &ReportReasonCopyright {
        self
    }
}

impl AsRef<ReportReasonCopyright> for ReportReasonCopyrightBuilder {
    fn as_ref(&self) -> &ReportReasonCopyright {
        &self.inner
    }
}

/// A custom reason provided by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonCustom {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonCustom {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonCustom {}

impl ReportReasonCustom {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonCustomBuilder {
        let mut inner = ReportReasonCustom::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonCustomBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonCustomBuilder {
    inner: ReportReasonCustom,
}

#[deprecated]
pub type RTDReportReasonCustomBuilder = ReportReasonCustomBuilder;

impl ReportReasonCustomBuilder {
    pub fn build(&self) -> ReportReasonCustom {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonCustom> for ReportReasonCustom {
    fn as_ref(&self) -> &ReportReasonCustom {
        self
    }
}

impl AsRef<ReportReasonCustom> for ReportReasonCustomBuilder {
    fn as_ref(&self) -> &ReportReasonCustom {
        &self.inner
    }
}

/// The chat represents a fake account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonFake {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonFake {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonFake {}

impl ReportReasonFake {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonFakeBuilder {
        let mut inner = ReportReasonFake::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonFakeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonFakeBuilder {
    inner: ReportReasonFake,
}

#[deprecated]
pub type RTDReportReasonFakeBuilder = ReportReasonFakeBuilder;

impl ReportReasonFakeBuilder {
    pub fn build(&self) -> ReportReasonFake {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonFake> for ReportReasonFake {
    fn as_ref(&self) -> &ReportReasonFake {
        self
    }
}

impl AsRef<ReportReasonFake> for ReportReasonFakeBuilder {
    fn as_ref(&self) -> &ReportReasonFake {
        &self.inner
    }
}

/// The chat has illegal drugs related content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonIllegalDrugs {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonIllegalDrugs {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonIllegalDrugs {}

impl ReportReasonIllegalDrugs {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonIllegalDrugsBuilder {
        let mut inner = ReportReasonIllegalDrugs::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonIllegalDrugsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonIllegalDrugsBuilder {
    inner: ReportReasonIllegalDrugs,
}

#[deprecated]
pub type RTDReportReasonIllegalDrugsBuilder = ReportReasonIllegalDrugsBuilder;

impl ReportReasonIllegalDrugsBuilder {
    pub fn build(&self) -> ReportReasonIllegalDrugs {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonIllegalDrugs> for ReportReasonIllegalDrugs {
    fn as_ref(&self) -> &ReportReasonIllegalDrugs {
        self
    }
}

impl AsRef<ReportReasonIllegalDrugs> for ReportReasonIllegalDrugsBuilder {
    fn as_ref(&self) -> &ReportReasonIllegalDrugs {
        &self.inner
    }
}

/// The chat contains messages with personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonPersonalDetails {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonPersonalDetails {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonPersonalDetails {}

impl ReportReasonPersonalDetails {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonPersonalDetailsBuilder {
        let mut inner = ReportReasonPersonalDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonPersonalDetailsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonPersonalDetailsBuilder {
    inner: ReportReasonPersonalDetails,
}

#[deprecated]
pub type RTDReportReasonPersonalDetailsBuilder = ReportReasonPersonalDetailsBuilder;

impl ReportReasonPersonalDetailsBuilder {
    pub fn build(&self) -> ReportReasonPersonalDetails {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonPersonalDetails> for ReportReasonPersonalDetails {
    fn as_ref(&self) -> &ReportReasonPersonalDetails {
        self
    }
}

impl AsRef<ReportReasonPersonalDetails> for ReportReasonPersonalDetailsBuilder {
    fn as_ref(&self) -> &ReportReasonPersonalDetails {
        &self.inner
    }
}

/// The chat contains pornographic messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonPornography {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonPornography {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonPornography {}

impl ReportReasonPornography {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonPornographyBuilder {
        let mut inner = ReportReasonPornography::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonPornographyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonPornographyBuilder {
    inner: ReportReasonPornography,
}

#[deprecated]
pub type RTDReportReasonPornographyBuilder = ReportReasonPornographyBuilder;

impl ReportReasonPornographyBuilder {
    pub fn build(&self) -> ReportReasonPornography {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonPornography> for ReportReasonPornography {
    fn as_ref(&self) -> &ReportReasonPornography {
        self
    }
}

impl AsRef<ReportReasonPornography> for ReportReasonPornographyBuilder {
    fn as_ref(&self) -> &ReportReasonPornography {
        &self.inner
    }
}

/// The chat contains spam messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonSpam {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonSpam {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonSpam {}

impl ReportReasonSpam {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonSpamBuilder {
        let mut inner = ReportReasonSpam::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonSpamBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonSpamBuilder {
    inner: ReportReasonSpam,
}

#[deprecated]
pub type RTDReportReasonSpamBuilder = ReportReasonSpamBuilder;

impl ReportReasonSpamBuilder {
    pub fn build(&self) -> ReportReasonSpam {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonSpam> for ReportReasonSpam {
    fn as_ref(&self) -> &ReportReasonSpam {
        self
    }
}

impl AsRef<ReportReasonSpam> for ReportReasonSpamBuilder {
    fn as_ref(&self) -> &ReportReasonSpam {
        &self.inner
    }
}

/// The location-based chat is unrelated to its stated location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonUnrelatedLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonUnrelatedLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonUnrelatedLocation {}

impl ReportReasonUnrelatedLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonUnrelatedLocationBuilder {
        let mut inner = ReportReasonUnrelatedLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonUnrelatedLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonUnrelatedLocationBuilder {
    inner: ReportReasonUnrelatedLocation,
}

#[deprecated]
pub type RTDReportReasonUnrelatedLocationBuilder = ReportReasonUnrelatedLocationBuilder;

impl ReportReasonUnrelatedLocationBuilder {
    pub fn build(&self) -> ReportReasonUnrelatedLocation {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonUnrelatedLocation> for ReportReasonUnrelatedLocation {
    fn as_ref(&self) -> &ReportReasonUnrelatedLocation {
        self
    }
}

impl AsRef<ReportReasonUnrelatedLocation> for ReportReasonUnrelatedLocationBuilder {
    fn as_ref(&self) -> &ReportReasonUnrelatedLocation {
        &self.inner
    }
}

/// The chat promotes violence
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonViolence {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ReportReasonViolence {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDReportReason for ReportReasonViolence {}

impl ReportReasonViolence {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ReportReasonViolenceBuilder {
        let mut inner = ReportReasonViolence::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ReportReasonViolenceBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ReportReasonViolenceBuilder {
    inner: ReportReasonViolence,
}

#[deprecated]
pub type RTDReportReasonViolenceBuilder = ReportReasonViolenceBuilder;

impl ReportReasonViolenceBuilder {
    pub fn build(&self) -> ReportReasonViolence {
        self.inner.clone()
    }
}

impl AsRef<ReportReasonViolence> for ReportReasonViolence {
    fn as_ref(&self) -> &ReportReasonViolence {
        self
    }
}

impl AsRef<ReportReasonViolence> for ReportReasonViolenceBuilder {
    fn as_ref(&self) -> &ReportReasonViolence {
        &self.inner
    }
}
