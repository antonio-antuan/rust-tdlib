use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a feature available to Premium users
pub trait TDPremiumFeature: Debug + RObject {}

/// Describes a feature available to Premium users
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumFeature {
    #[doc(hidden)]
    _Default,
    /// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
    #[serde(rename(deserialize = "premiumFeatureAdvancedChatManagement"))]
    AdvancedChatManagement(PremiumFeatureAdvancedChatManagement),
    /// Profile photo animation on message and chat screens
    #[serde(rename(deserialize = "premiumFeatureAnimatedProfilePhoto"))]
    AnimatedProfilePhoto(PremiumFeatureAnimatedProfilePhoto),
    /// Allowed to set a premium appllication icons
    #[serde(rename(deserialize = "premiumFeatureAppIcons"))]
    AppIcons(PremiumFeatureAppIcons),
    /// Disabled ads
    #[serde(rename(deserialize = "premiumFeatureDisabledAds"))]
    DisabledAds(PremiumFeatureDisabledAds),
    /// Improved download speed
    #[serde(rename(deserialize = "premiumFeatureImprovedDownloadSpeed"))]
    ImprovedDownloadSpeed(PremiumFeatureImprovedDownloadSpeed),
    /// Increased limits
    #[serde(rename(deserialize = "premiumFeatureIncreasedLimits"))]
    IncreasedLimits(PremiumFeatureIncreasedLimits),
    /// Increased maximum upload file size
    #[serde(rename(deserialize = "premiumFeatureIncreasedUploadFileSize"))]
    IncreasedUploadFileSize(PremiumFeatureIncreasedUploadFileSize),
    /// A badge in the user's profile
    #[serde(rename(deserialize = "premiumFeatureProfileBadge"))]
    ProfileBadge(PremiumFeatureProfileBadge),
    /// Allowed to use more reactions
    #[serde(rename(deserialize = "premiumFeatureUniqueReactions"))]
    UniqueReactions(PremiumFeatureUniqueReactions),
    /// Allowed to use premium stickers with unique effects
    #[serde(rename(deserialize = "premiumFeatureUniqueStickers"))]
    UniqueStickers(PremiumFeatureUniqueStickers),
    /// The ability to convert voice notes to text
    #[serde(rename(deserialize = "premiumFeatureVoiceRecognition"))]
    VoiceRecognition(PremiumFeatureVoiceRecognition),
}

impl Default for PremiumFeature {
    fn default() -> Self {
        PremiumFeature::_Default
    }
}

impl RObject for PremiumFeature {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumFeature::AdvancedChatManagement(t) => t.extra(),
            PremiumFeature::AnimatedProfilePhoto(t) => t.extra(),
            PremiumFeature::AppIcons(t) => t.extra(),
            PremiumFeature::DisabledAds(t) => t.extra(),
            PremiumFeature::ImprovedDownloadSpeed(t) => t.extra(),
            PremiumFeature::IncreasedLimits(t) => t.extra(),
            PremiumFeature::IncreasedUploadFileSize(t) => t.extra(),
            PremiumFeature::ProfileBadge(t) => t.extra(),
            PremiumFeature::UniqueReactions(t) => t.extra(),
            PremiumFeature::UniqueStickers(t) => t.extra(),
            PremiumFeature::VoiceRecognition(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumFeature::AdvancedChatManagement(t) => t.client_id(),
            PremiumFeature::AnimatedProfilePhoto(t) => t.client_id(),
            PremiumFeature::AppIcons(t) => t.client_id(),
            PremiumFeature::DisabledAds(t) => t.client_id(),
            PremiumFeature::ImprovedDownloadSpeed(t) => t.client_id(),
            PremiumFeature::IncreasedLimits(t) => t.client_id(),
            PremiumFeature::IncreasedUploadFileSize(t) => t.client_id(),
            PremiumFeature::ProfileBadge(t) => t.client_id(),
            PremiumFeature::UniqueReactions(t) => t.client_id(),
            PremiumFeature::UniqueStickers(t) => t.client_id(),
            PremiumFeature::VoiceRecognition(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumFeature {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PremiumFeature::_Default)
    }
}

impl AsRef<PremiumFeature> for PremiumFeature {
    fn as_ref(&self) -> &PremiumFeature {
        self
    }
}

/// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAdvancedChatManagement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureAdvancedChatManagement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureAdvancedChatManagement {}

impl PremiumFeatureAdvancedChatManagement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureAdvancedChatManagementBuilder {
        let mut inner = PremiumFeatureAdvancedChatManagement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureAdvancedChatManagementBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureAdvancedChatManagementBuilder {
    inner: PremiumFeatureAdvancedChatManagement,
}

impl RTDPremiumFeatureAdvancedChatManagementBuilder {
    pub fn build(&self) -> PremiumFeatureAdvancedChatManagement {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureAdvancedChatManagement> for PremiumFeatureAdvancedChatManagement {
    fn as_ref(&self) -> &PremiumFeatureAdvancedChatManagement {
        self
    }
}

impl AsRef<PremiumFeatureAdvancedChatManagement>
    for RTDPremiumFeatureAdvancedChatManagementBuilder
{
    fn as_ref(&self) -> &PremiumFeatureAdvancedChatManagement {
        &self.inner
    }
}

/// Profile photo animation on message and chat screens
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAnimatedProfilePhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureAnimatedProfilePhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureAnimatedProfilePhoto {}

impl PremiumFeatureAnimatedProfilePhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureAnimatedProfilePhotoBuilder {
        let mut inner = PremiumFeatureAnimatedProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureAnimatedProfilePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureAnimatedProfilePhotoBuilder {
    inner: PremiumFeatureAnimatedProfilePhoto,
}

impl RTDPremiumFeatureAnimatedProfilePhotoBuilder {
    pub fn build(&self) -> PremiumFeatureAnimatedProfilePhoto {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureAnimatedProfilePhoto> for PremiumFeatureAnimatedProfilePhoto {
    fn as_ref(&self) -> &PremiumFeatureAnimatedProfilePhoto {
        self
    }
}

impl AsRef<PremiumFeatureAnimatedProfilePhoto> for RTDPremiumFeatureAnimatedProfilePhotoBuilder {
    fn as_ref(&self) -> &PremiumFeatureAnimatedProfilePhoto {
        &self.inner
    }
}

/// Allowed to set a premium appllication icons
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAppIcons {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureAppIcons {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureAppIcons {}

impl PremiumFeatureAppIcons {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureAppIconsBuilder {
        let mut inner = PremiumFeatureAppIcons::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureAppIconsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureAppIconsBuilder {
    inner: PremiumFeatureAppIcons,
}

impl RTDPremiumFeatureAppIconsBuilder {
    pub fn build(&self) -> PremiumFeatureAppIcons {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureAppIcons> for PremiumFeatureAppIcons {
    fn as_ref(&self) -> &PremiumFeatureAppIcons {
        self
    }
}

impl AsRef<PremiumFeatureAppIcons> for RTDPremiumFeatureAppIconsBuilder {
    fn as_ref(&self) -> &PremiumFeatureAppIcons {
        &self.inner
    }
}

/// Disabled ads
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureDisabledAds {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureDisabledAds {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureDisabledAds {}

impl PremiumFeatureDisabledAds {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureDisabledAdsBuilder {
        let mut inner = PremiumFeatureDisabledAds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureDisabledAdsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureDisabledAdsBuilder {
    inner: PremiumFeatureDisabledAds,
}

impl RTDPremiumFeatureDisabledAdsBuilder {
    pub fn build(&self) -> PremiumFeatureDisabledAds {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureDisabledAds> for PremiumFeatureDisabledAds {
    fn as_ref(&self) -> &PremiumFeatureDisabledAds {
        self
    }
}

impl AsRef<PremiumFeatureDisabledAds> for RTDPremiumFeatureDisabledAdsBuilder {
    fn as_ref(&self) -> &PremiumFeatureDisabledAds {
        &self.inner
    }
}

/// Improved download speed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureImprovedDownloadSpeed {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureImprovedDownloadSpeed {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureImprovedDownloadSpeed {}

impl PremiumFeatureImprovedDownloadSpeed {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureImprovedDownloadSpeedBuilder {
        let mut inner = PremiumFeatureImprovedDownloadSpeed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureImprovedDownloadSpeedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureImprovedDownloadSpeedBuilder {
    inner: PremiumFeatureImprovedDownloadSpeed,
}

impl RTDPremiumFeatureImprovedDownloadSpeedBuilder {
    pub fn build(&self) -> PremiumFeatureImprovedDownloadSpeed {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureImprovedDownloadSpeed> for PremiumFeatureImprovedDownloadSpeed {
    fn as_ref(&self) -> &PremiumFeatureImprovedDownloadSpeed {
        self
    }
}

impl AsRef<PremiumFeatureImprovedDownloadSpeed> for RTDPremiumFeatureImprovedDownloadSpeedBuilder {
    fn as_ref(&self) -> &PremiumFeatureImprovedDownloadSpeed {
        &self.inner
    }
}

/// Increased limits
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureIncreasedLimits {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureIncreasedLimits {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureIncreasedLimits {}

impl PremiumFeatureIncreasedLimits {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureIncreasedLimitsBuilder {
        let mut inner = PremiumFeatureIncreasedLimits::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureIncreasedLimitsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureIncreasedLimitsBuilder {
    inner: PremiumFeatureIncreasedLimits,
}

impl RTDPremiumFeatureIncreasedLimitsBuilder {
    pub fn build(&self) -> PremiumFeatureIncreasedLimits {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureIncreasedLimits> for PremiumFeatureIncreasedLimits {
    fn as_ref(&self) -> &PremiumFeatureIncreasedLimits {
        self
    }
}

impl AsRef<PremiumFeatureIncreasedLimits> for RTDPremiumFeatureIncreasedLimitsBuilder {
    fn as_ref(&self) -> &PremiumFeatureIncreasedLimits {
        &self.inner
    }
}

/// Increased maximum upload file size
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureIncreasedUploadFileSize {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureIncreasedUploadFileSize {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureIncreasedUploadFileSize {}

impl PremiumFeatureIncreasedUploadFileSize {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureIncreasedUploadFileSizeBuilder {
        let mut inner = PremiumFeatureIncreasedUploadFileSize::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureIncreasedUploadFileSizeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureIncreasedUploadFileSizeBuilder {
    inner: PremiumFeatureIncreasedUploadFileSize,
}

impl RTDPremiumFeatureIncreasedUploadFileSizeBuilder {
    pub fn build(&self) -> PremiumFeatureIncreasedUploadFileSize {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureIncreasedUploadFileSize> for PremiumFeatureIncreasedUploadFileSize {
    fn as_ref(&self) -> &PremiumFeatureIncreasedUploadFileSize {
        self
    }
}

impl AsRef<PremiumFeatureIncreasedUploadFileSize>
    for RTDPremiumFeatureIncreasedUploadFileSizeBuilder
{
    fn as_ref(&self) -> &PremiumFeatureIncreasedUploadFileSize {
        &self.inner
    }
}

/// A badge in the user's profile
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureProfileBadge {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureProfileBadge {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureProfileBadge {}

impl PremiumFeatureProfileBadge {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureProfileBadgeBuilder {
        let mut inner = PremiumFeatureProfileBadge::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureProfileBadgeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureProfileBadgeBuilder {
    inner: PremiumFeatureProfileBadge,
}

impl RTDPremiumFeatureProfileBadgeBuilder {
    pub fn build(&self) -> PremiumFeatureProfileBadge {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureProfileBadge> for PremiumFeatureProfileBadge {
    fn as_ref(&self) -> &PremiumFeatureProfileBadge {
        self
    }
}

impl AsRef<PremiumFeatureProfileBadge> for RTDPremiumFeatureProfileBadgeBuilder {
    fn as_ref(&self) -> &PremiumFeatureProfileBadge {
        &self.inner
    }
}

/// Allowed to use more reactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureUniqueReactions {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureUniqueReactions {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureUniqueReactions {}

impl PremiumFeatureUniqueReactions {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureUniqueReactionsBuilder {
        let mut inner = PremiumFeatureUniqueReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureUniqueReactionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureUniqueReactionsBuilder {
    inner: PremiumFeatureUniqueReactions,
}

impl RTDPremiumFeatureUniqueReactionsBuilder {
    pub fn build(&self) -> PremiumFeatureUniqueReactions {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureUniqueReactions> for PremiumFeatureUniqueReactions {
    fn as_ref(&self) -> &PremiumFeatureUniqueReactions {
        self
    }
}

impl AsRef<PremiumFeatureUniqueReactions> for RTDPremiumFeatureUniqueReactionsBuilder {
    fn as_ref(&self) -> &PremiumFeatureUniqueReactions {
        &self.inner
    }
}

/// Allowed to use premium stickers with unique effects
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureUniqueStickers {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureUniqueStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureUniqueStickers {}

impl PremiumFeatureUniqueStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureUniqueStickersBuilder {
        let mut inner = PremiumFeatureUniqueStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureUniqueStickersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureUniqueStickersBuilder {
    inner: PremiumFeatureUniqueStickers,
}

impl RTDPremiumFeatureUniqueStickersBuilder {
    pub fn build(&self) -> PremiumFeatureUniqueStickers {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureUniqueStickers> for PremiumFeatureUniqueStickers {
    fn as_ref(&self) -> &PremiumFeatureUniqueStickers {
        self
    }
}

impl AsRef<PremiumFeatureUniqueStickers> for RTDPremiumFeatureUniqueStickersBuilder {
    fn as_ref(&self) -> &PremiumFeatureUniqueStickers {
        &self.inner
    }
}

/// The ability to convert voice notes to text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureVoiceRecognition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureVoiceRecognition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureVoiceRecognition {}

impl PremiumFeatureVoiceRecognition {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumFeatureVoiceRecognitionBuilder {
        let mut inner = PremiumFeatureVoiceRecognition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumFeatureVoiceRecognitionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumFeatureVoiceRecognitionBuilder {
    inner: PremiumFeatureVoiceRecognition,
}

impl RTDPremiumFeatureVoiceRecognitionBuilder {
    pub fn build(&self) -> PremiumFeatureVoiceRecognition {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureVoiceRecognition> for PremiumFeatureVoiceRecognition {
    fn as_ref(&self) -> &PremiumFeatureVoiceRecognition {
        self
    }
}

impl AsRef<PremiumFeatureVoiceRecognition> for RTDPremiumFeatureVoiceRecognitionBuilder {
    fn as_ref(&self) -> &PremiumFeatureVoiceRecognition {
        &self.inner
    }
}
