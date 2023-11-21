use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a feature available to Premium users
pub trait TDPremiumFeature: Debug + RObject {}

/// Describes a feature available to Premium users
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum PremiumFeature {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The ability to choose accent color
    #[serde(rename = "premiumFeatureAccentColor")]
    AccentColor(PremiumFeatureAccentColor),
    /// Ability to change position of the main chat list, archive and mute all new chats from non-contacts, and completely disable notifications about the user's contacts joined Telegram
    #[serde(rename = "premiumFeatureAdvancedChatManagement")]
    AdvancedChatManagement(PremiumFeatureAdvancedChatManagement),
    /// Profile photo animation on message and chat screens
    #[serde(rename = "premiumFeatureAnimatedProfilePhoto")]
    AnimatedProfilePhoto(PremiumFeatureAnimatedProfilePhoto),
    /// Allowed to set a premium application icons
    #[serde(rename = "premiumFeatureAppIcons")]
    AppIcons(PremiumFeatureAppIcons),
    /// The ability to boost chats
    #[serde(rename = "premiumFeatureChatBoost")]
    ChatBoost(PremiumFeatureChatBoost),
    /// Allowed to use custom emoji stickers in message texts and captions
    #[serde(rename = "premiumFeatureCustomEmoji")]
    CustomEmoji(PremiumFeatureCustomEmoji),
    /// Disabled ads
    #[serde(rename = "premiumFeatureDisabledAds")]
    DisabledAds(PremiumFeatureDisabledAds),
    /// An emoji status shown along with the user's name
    #[serde(rename = "premiumFeatureEmojiStatus")]
    EmojiStatus(PremiumFeatureEmojiStatus),
    /// The ability to set a custom emoji as a forum topic icon
    #[serde(rename = "premiumFeatureForumTopicIcon")]
    ForumTopicIcon(PremiumFeatureForumTopicIcon),
    /// Improved download speed
    #[serde(rename = "premiumFeatureImprovedDownloadSpeed")]
    ImprovedDownloadSpeed(PremiumFeatureImprovedDownloadSpeed),
    /// Increased limits
    #[serde(rename = "premiumFeatureIncreasedLimits")]
    IncreasedLimits(PremiumFeatureIncreasedLimits),
    /// Increased maximum upload file size
    #[serde(rename = "premiumFeatureIncreasedUploadFileSize")]
    IncreasedUploadFileSize(PremiumFeatureIncreasedUploadFileSize),
    /// A badge in the user's profile
    #[serde(rename = "premiumFeatureProfileBadge")]
    ProfileBadge(PremiumFeatureProfileBadge),
    /// Allowed to translate chat messages real-time
    #[serde(rename = "premiumFeatureRealTimeChatTranslation")]
    RealTimeChatTranslation(PremiumFeatureRealTimeChatTranslation),
    /// Allowed to use more reactions
    #[serde(rename = "premiumFeatureUniqueReactions")]
    UniqueReactions(PremiumFeatureUniqueReactions),
    /// Allowed to use premium stickers with unique effects
    #[serde(rename = "premiumFeatureUniqueStickers")]
    UniqueStickers(PremiumFeatureUniqueStickers),
    /// Allowed to use many additional features for stories
    #[serde(rename = "premiumFeatureUpgradedStories")]
    UpgradedStories(PremiumFeatureUpgradedStories),
    /// The ability to convert voice notes to text
    #[serde(rename = "premiumFeatureVoiceRecognition")]
    VoiceRecognition(PremiumFeatureVoiceRecognition),
}

impl RObject for PremiumFeature {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumFeature::AccentColor(t) => t.extra(),
            PremiumFeature::AdvancedChatManagement(t) => t.extra(),
            PremiumFeature::AnimatedProfilePhoto(t) => t.extra(),
            PremiumFeature::AppIcons(t) => t.extra(),
            PremiumFeature::ChatBoost(t) => t.extra(),
            PremiumFeature::CustomEmoji(t) => t.extra(),
            PremiumFeature::DisabledAds(t) => t.extra(),
            PremiumFeature::EmojiStatus(t) => t.extra(),
            PremiumFeature::ForumTopicIcon(t) => t.extra(),
            PremiumFeature::ImprovedDownloadSpeed(t) => t.extra(),
            PremiumFeature::IncreasedLimits(t) => t.extra(),
            PremiumFeature::IncreasedUploadFileSize(t) => t.extra(),
            PremiumFeature::ProfileBadge(t) => t.extra(),
            PremiumFeature::RealTimeChatTranslation(t) => t.extra(),
            PremiumFeature::UniqueReactions(t) => t.extra(),
            PremiumFeature::UniqueStickers(t) => t.extra(),
            PremiumFeature::UpgradedStories(t) => t.extra(),
            PremiumFeature::VoiceRecognition(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumFeature::AccentColor(t) => t.client_id(),
            PremiumFeature::AdvancedChatManagement(t) => t.client_id(),
            PremiumFeature::AnimatedProfilePhoto(t) => t.client_id(),
            PremiumFeature::AppIcons(t) => t.client_id(),
            PremiumFeature::ChatBoost(t) => t.client_id(),
            PremiumFeature::CustomEmoji(t) => t.client_id(),
            PremiumFeature::DisabledAds(t) => t.client_id(),
            PremiumFeature::EmojiStatus(t) => t.client_id(),
            PremiumFeature::ForumTopicIcon(t) => t.client_id(),
            PremiumFeature::ImprovedDownloadSpeed(t) => t.client_id(),
            PremiumFeature::IncreasedLimits(t) => t.client_id(),
            PremiumFeature::IncreasedUploadFileSize(t) => t.client_id(),
            PremiumFeature::ProfileBadge(t) => t.client_id(),
            PremiumFeature::RealTimeChatTranslation(t) => t.client_id(),
            PremiumFeature::UniqueReactions(t) => t.client_id(),
            PremiumFeature::UniqueStickers(t) => t.client_id(),
            PremiumFeature::UpgradedStories(t) => t.client_id(),
            PremiumFeature::VoiceRecognition(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumFeature {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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

/// The ability to choose accent color
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureAccentColor {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureAccentColor {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureAccentColor {}

impl PremiumFeatureAccentColor {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureAccentColorBuilder {
        let mut inner = PremiumFeatureAccentColor::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureAccentColorBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureAccentColorBuilder {
    inner: PremiumFeatureAccentColor,
}

#[deprecated]
pub type RTDPremiumFeatureAccentColorBuilder = PremiumFeatureAccentColorBuilder;

impl PremiumFeatureAccentColorBuilder {
    pub fn build(&self) -> PremiumFeatureAccentColor {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureAccentColor> for PremiumFeatureAccentColor {
    fn as_ref(&self) -> &PremiumFeatureAccentColor {
        self
    }
}

impl AsRef<PremiumFeatureAccentColor> for PremiumFeatureAccentColorBuilder {
    fn as_ref(&self) -> &PremiumFeatureAccentColor {
        &self.inner
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureAdvancedChatManagementBuilder {
        let mut inner = PremiumFeatureAdvancedChatManagement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureAdvancedChatManagementBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureAdvancedChatManagementBuilder {
    inner: PremiumFeatureAdvancedChatManagement,
}

#[deprecated]
pub type RTDPremiumFeatureAdvancedChatManagementBuilder =
    PremiumFeatureAdvancedChatManagementBuilder;

impl PremiumFeatureAdvancedChatManagementBuilder {
    pub fn build(&self) -> PremiumFeatureAdvancedChatManagement {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureAdvancedChatManagement> for PremiumFeatureAdvancedChatManagement {
    fn as_ref(&self) -> &PremiumFeatureAdvancedChatManagement {
        self
    }
}

impl AsRef<PremiumFeatureAdvancedChatManagement> for PremiumFeatureAdvancedChatManagementBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureAnimatedProfilePhotoBuilder {
        let mut inner = PremiumFeatureAnimatedProfilePhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureAnimatedProfilePhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureAnimatedProfilePhotoBuilder {
    inner: PremiumFeatureAnimatedProfilePhoto,
}

#[deprecated]
pub type RTDPremiumFeatureAnimatedProfilePhotoBuilder = PremiumFeatureAnimatedProfilePhotoBuilder;

impl PremiumFeatureAnimatedProfilePhotoBuilder {
    pub fn build(&self) -> PremiumFeatureAnimatedProfilePhoto {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureAnimatedProfilePhoto> for PremiumFeatureAnimatedProfilePhoto {
    fn as_ref(&self) -> &PremiumFeatureAnimatedProfilePhoto {
        self
    }
}

impl AsRef<PremiumFeatureAnimatedProfilePhoto> for PremiumFeatureAnimatedProfilePhotoBuilder {
    fn as_ref(&self) -> &PremiumFeatureAnimatedProfilePhoto {
        &self.inner
    }
}

/// Allowed to set a premium application icons
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureAppIconsBuilder {
        let mut inner = PremiumFeatureAppIcons::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureAppIconsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureAppIconsBuilder {
    inner: PremiumFeatureAppIcons,
}

#[deprecated]
pub type RTDPremiumFeatureAppIconsBuilder = PremiumFeatureAppIconsBuilder;

impl PremiumFeatureAppIconsBuilder {
    pub fn build(&self) -> PremiumFeatureAppIcons {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureAppIcons> for PremiumFeatureAppIcons {
    fn as_ref(&self) -> &PremiumFeatureAppIcons {
        self
    }
}

impl AsRef<PremiumFeatureAppIcons> for PremiumFeatureAppIconsBuilder {
    fn as_ref(&self) -> &PremiumFeatureAppIcons {
        &self.inner
    }
}

/// The ability to boost chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureChatBoost {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureChatBoost {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureChatBoost {}

impl PremiumFeatureChatBoost {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureChatBoostBuilder {
        let mut inner = PremiumFeatureChatBoost::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureChatBoostBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureChatBoostBuilder {
    inner: PremiumFeatureChatBoost,
}

#[deprecated]
pub type RTDPremiumFeatureChatBoostBuilder = PremiumFeatureChatBoostBuilder;

impl PremiumFeatureChatBoostBuilder {
    pub fn build(&self) -> PremiumFeatureChatBoost {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureChatBoost> for PremiumFeatureChatBoost {
    fn as_ref(&self) -> &PremiumFeatureChatBoost {
        self
    }
}

impl AsRef<PremiumFeatureChatBoost> for PremiumFeatureChatBoostBuilder {
    fn as_ref(&self) -> &PremiumFeatureChatBoost {
        &self.inner
    }
}

/// Allowed to use custom emoji stickers in message texts and captions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureCustomEmoji {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureCustomEmoji {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureCustomEmoji {}

impl PremiumFeatureCustomEmoji {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureCustomEmojiBuilder {
        let mut inner = PremiumFeatureCustomEmoji::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureCustomEmojiBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureCustomEmojiBuilder {
    inner: PremiumFeatureCustomEmoji,
}

#[deprecated]
pub type RTDPremiumFeatureCustomEmojiBuilder = PremiumFeatureCustomEmojiBuilder;

impl PremiumFeatureCustomEmojiBuilder {
    pub fn build(&self) -> PremiumFeatureCustomEmoji {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureCustomEmoji> for PremiumFeatureCustomEmoji {
    fn as_ref(&self) -> &PremiumFeatureCustomEmoji {
        self
    }
}

impl AsRef<PremiumFeatureCustomEmoji> for PremiumFeatureCustomEmojiBuilder {
    fn as_ref(&self) -> &PremiumFeatureCustomEmoji {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureDisabledAdsBuilder {
        let mut inner = PremiumFeatureDisabledAds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureDisabledAdsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureDisabledAdsBuilder {
    inner: PremiumFeatureDisabledAds,
}

#[deprecated]
pub type RTDPremiumFeatureDisabledAdsBuilder = PremiumFeatureDisabledAdsBuilder;

impl PremiumFeatureDisabledAdsBuilder {
    pub fn build(&self) -> PremiumFeatureDisabledAds {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureDisabledAds> for PremiumFeatureDisabledAds {
    fn as_ref(&self) -> &PremiumFeatureDisabledAds {
        self
    }
}

impl AsRef<PremiumFeatureDisabledAds> for PremiumFeatureDisabledAdsBuilder {
    fn as_ref(&self) -> &PremiumFeatureDisabledAds {
        &self.inner
    }
}

/// An emoji status shown along with the user's name
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureEmojiStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureEmojiStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureEmojiStatus {}

impl PremiumFeatureEmojiStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureEmojiStatusBuilder {
        let mut inner = PremiumFeatureEmojiStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureEmojiStatusBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureEmojiStatusBuilder {
    inner: PremiumFeatureEmojiStatus,
}

#[deprecated]
pub type RTDPremiumFeatureEmojiStatusBuilder = PremiumFeatureEmojiStatusBuilder;

impl PremiumFeatureEmojiStatusBuilder {
    pub fn build(&self) -> PremiumFeatureEmojiStatus {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureEmojiStatus> for PremiumFeatureEmojiStatus {
    fn as_ref(&self) -> &PremiumFeatureEmojiStatus {
        self
    }
}

impl AsRef<PremiumFeatureEmojiStatus> for PremiumFeatureEmojiStatusBuilder {
    fn as_ref(&self) -> &PremiumFeatureEmojiStatus {
        &self.inner
    }
}

/// The ability to set a custom emoji as a forum topic icon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureForumTopicIcon {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureForumTopicIcon {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureForumTopicIcon {}

impl PremiumFeatureForumTopicIcon {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureForumTopicIconBuilder {
        let mut inner = PremiumFeatureForumTopicIcon::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureForumTopicIconBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureForumTopicIconBuilder {
    inner: PremiumFeatureForumTopicIcon,
}

#[deprecated]
pub type RTDPremiumFeatureForumTopicIconBuilder = PremiumFeatureForumTopicIconBuilder;

impl PremiumFeatureForumTopicIconBuilder {
    pub fn build(&self) -> PremiumFeatureForumTopicIcon {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureForumTopicIcon> for PremiumFeatureForumTopicIcon {
    fn as_ref(&self) -> &PremiumFeatureForumTopicIcon {
        self
    }
}

impl AsRef<PremiumFeatureForumTopicIcon> for PremiumFeatureForumTopicIconBuilder {
    fn as_ref(&self) -> &PremiumFeatureForumTopicIcon {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureImprovedDownloadSpeedBuilder {
        let mut inner = PremiumFeatureImprovedDownloadSpeed::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureImprovedDownloadSpeedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureImprovedDownloadSpeedBuilder {
    inner: PremiumFeatureImprovedDownloadSpeed,
}

#[deprecated]
pub type RTDPremiumFeatureImprovedDownloadSpeedBuilder = PremiumFeatureImprovedDownloadSpeedBuilder;

impl PremiumFeatureImprovedDownloadSpeedBuilder {
    pub fn build(&self) -> PremiumFeatureImprovedDownloadSpeed {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureImprovedDownloadSpeed> for PremiumFeatureImprovedDownloadSpeed {
    fn as_ref(&self) -> &PremiumFeatureImprovedDownloadSpeed {
        self
    }
}

impl AsRef<PremiumFeatureImprovedDownloadSpeed> for PremiumFeatureImprovedDownloadSpeedBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureIncreasedLimitsBuilder {
        let mut inner = PremiumFeatureIncreasedLimits::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureIncreasedLimitsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureIncreasedLimitsBuilder {
    inner: PremiumFeatureIncreasedLimits,
}

#[deprecated]
pub type RTDPremiumFeatureIncreasedLimitsBuilder = PremiumFeatureIncreasedLimitsBuilder;

impl PremiumFeatureIncreasedLimitsBuilder {
    pub fn build(&self) -> PremiumFeatureIncreasedLimits {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureIncreasedLimits> for PremiumFeatureIncreasedLimits {
    fn as_ref(&self) -> &PremiumFeatureIncreasedLimits {
        self
    }
}

impl AsRef<PremiumFeatureIncreasedLimits> for PremiumFeatureIncreasedLimitsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureIncreasedUploadFileSizeBuilder {
        let mut inner = PremiumFeatureIncreasedUploadFileSize::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureIncreasedUploadFileSizeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureIncreasedUploadFileSizeBuilder {
    inner: PremiumFeatureIncreasedUploadFileSize,
}

#[deprecated]
pub type RTDPremiumFeatureIncreasedUploadFileSizeBuilder =
    PremiumFeatureIncreasedUploadFileSizeBuilder;

impl PremiumFeatureIncreasedUploadFileSizeBuilder {
    pub fn build(&self) -> PremiumFeatureIncreasedUploadFileSize {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureIncreasedUploadFileSize> for PremiumFeatureIncreasedUploadFileSize {
    fn as_ref(&self) -> &PremiumFeatureIncreasedUploadFileSize {
        self
    }
}

impl AsRef<PremiumFeatureIncreasedUploadFileSize> for PremiumFeatureIncreasedUploadFileSizeBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureProfileBadgeBuilder {
        let mut inner = PremiumFeatureProfileBadge::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureProfileBadgeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureProfileBadgeBuilder {
    inner: PremiumFeatureProfileBadge,
}

#[deprecated]
pub type RTDPremiumFeatureProfileBadgeBuilder = PremiumFeatureProfileBadgeBuilder;

impl PremiumFeatureProfileBadgeBuilder {
    pub fn build(&self) -> PremiumFeatureProfileBadge {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureProfileBadge> for PremiumFeatureProfileBadge {
    fn as_ref(&self) -> &PremiumFeatureProfileBadge {
        self
    }
}

impl AsRef<PremiumFeatureProfileBadge> for PremiumFeatureProfileBadgeBuilder {
    fn as_ref(&self) -> &PremiumFeatureProfileBadge {
        &self.inner
    }
}

/// Allowed to translate chat messages real-time
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureRealTimeChatTranslation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureRealTimeChatTranslation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureRealTimeChatTranslation {}

impl PremiumFeatureRealTimeChatTranslation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureRealTimeChatTranslationBuilder {
        let mut inner = PremiumFeatureRealTimeChatTranslation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureRealTimeChatTranslationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureRealTimeChatTranslationBuilder {
    inner: PremiumFeatureRealTimeChatTranslation,
}

#[deprecated]
pub type RTDPremiumFeatureRealTimeChatTranslationBuilder =
    PremiumFeatureRealTimeChatTranslationBuilder;

impl PremiumFeatureRealTimeChatTranslationBuilder {
    pub fn build(&self) -> PremiumFeatureRealTimeChatTranslation {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureRealTimeChatTranslation> for PremiumFeatureRealTimeChatTranslation {
    fn as_ref(&self) -> &PremiumFeatureRealTimeChatTranslation {
        self
    }
}

impl AsRef<PremiumFeatureRealTimeChatTranslation> for PremiumFeatureRealTimeChatTranslationBuilder {
    fn as_ref(&self) -> &PremiumFeatureRealTimeChatTranslation {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureUniqueReactionsBuilder {
        let mut inner = PremiumFeatureUniqueReactions::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureUniqueReactionsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureUniqueReactionsBuilder {
    inner: PremiumFeatureUniqueReactions,
}

#[deprecated]
pub type RTDPremiumFeatureUniqueReactionsBuilder = PremiumFeatureUniqueReactionsBuilder;

impl PremiumFeatureUniqueReactionsBuilder {
    pub fn build(&self) -> PremiumFeatureUniqueReactions {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureUniqueReactions> for PremiumFeatureUniqueReactions {
    fn as_ref(&self) -> &PremiumFeatureUniqueReactions {
        self
    }
}

impl AsRef<PremiumFeatureUniqueReactions> for PremiumFeatureUniqueReactionsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureUniqueStickersBuilder {
        let mut inner = PremiumFeatureUniqueStickers::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureUniqueStickersBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureUniqueStickersBuilder {
    inner: PremiumFeatureUniqueStickers,
}

#[deprecated]
pub type RTDPremiumFeatureUniqueStickersBuilder = PremiumFeatureUniqueStickersBuilder;

impl PremiumFeatureUniqueStickersBuilder {
    pub fn build(&self) -> PremiumFeatureUniqueStickers {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureUniqueStickers> for PremiumFeatureUniqueStickers {
    fn as_ref(&self) -> &PremiumFeatureUniqueStickers {
        self
    }
}

impl AsRef<PremiumFeatureUniqueStickers> for PremiumFeatureUniqueStickersBuilder {
    fn as_ref(&self) -> &PremiumFeatureUniqueStickers {
        &self.inner
    }
}

/// Allowed to use many additional features for stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumFeatureUpgradedStories {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumFeatureUpgradedStories {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumFeature for PremiumFeatureUpgradedStories {}

impl PremiumFeatureUpgradedStories {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureUpgradedStoriesBuilder {
        let mut inner = PremiumFeatureUpgradedStories::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureUpgradedStoriesBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureUpgradedStoriesBuilder {
    inner: PremiumFeatureUpgradedStories,
}

#[deprecated]
pub type RTDPremiumFeatureUpgradedStoriesBuilder = PremiumFeatureUpgradedStoriesBuilder;

impl PremiumFeatureUpgradedStoriesBuilder {
    pub fn build(&self) -> PremiumFeatureUpgradedStories {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureUpgradedStories> for PremiumFeatureUpgradedStories {
    fn as_ref(&self) -> &PremiumFeatureUpgradedStories {
        self
    }
}

impl AsRef<PremiumFeatureUpgradedStories> for PremiumFeatureUpgradedStoriesBuilder {
    fn as_ref(&self) -> &PremiumFeatureUpgradedStories {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumFeatureVoiceRecognitionBuilder {
        let mut inner = PremiumFeatureVoiceRecognition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumFeatureVoiceRecognitionBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumFeatureVoiceRecognitionBuilder {
    inner: PremiumFeatureVoiceRecognition,
}

#[deprecated]
pub type RTDPremiumFeatureVoiceRecognitionBuilder = PremiumFeatureVoiceRecognitionBuilder;

impl PremiumFeatureVoiceRecognitionBuilder {
    pub fn build(&self) -> PremiumFeatureVoiceRecognition {
        self.inner.clone()
    }
}

impl AsRef<PremiumFeatureVoiceRecognition> for PremiumFeatureVoiceRecognition {
    fn as_ref(&self) -> &PremiumFeatureVoiceRecognition {
        self
    }
}

impl AsRef<PremiumFeatureVoiceRecognition> for PremiumFeatureVoiceRecognitionBuilder {
    fn as_ref(&self) -> &PremiumFeatureVoiceRecognition {
        &self.inner
    }
}
