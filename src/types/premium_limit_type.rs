use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a limit, increased for Premium users
pub trait TDPremiumLimitType: Debug + RObject {}

/// Describes type of a limit, increased for Premium users
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PremiumLimitType {
    #[doc(hidden)]
    _Default,
    /// The maximum length of the user's bio
    #[serde(rename(deserialize = "premiumLimitTypeBioLength"))]
    BioLength(PremiumLimitTypeBioLength),
    /// The maximum length of sent media caption
    #[serde(rename(deserialize = "premiumLimitTypeCaptionLength"))]
    CaptionLength(PremiumLimitTypeCaptionLength),
    /// The maximum number of pinned and always included, or always excluded chats in a chat filter
    #[serde(rename(deserialize = "premiumLimitTypeChatFilterChosenChatCount"))]
    ChatFilterChosenChatCount(PremiumLimitTypeChatFilterChosenChatCount),
    /// The maximum number of chat filters
    #[serde(rename(deserialize = "premiumLimitTypeChatFilterCount"))]
    ChatFilterCount(PremiumLimitTypeChatFilterCount),
    /// The maximum number of created public chats
    #[serde(rename(deserialize = "premiumLimitTypeCreatedPublicChatCount"))]
    CreatedPublicChatCount(PremiumLimitTypeCreatedPublicChatCount),
    /// The maximum number of favorite stickers
    #[serde(rename(deserialize = "premiumLimitTypeFavoriteStickerCount"))]
    FavoriteStickerCount(PremiumLimitTypeFavoriteStickerCount),
    /// The maximum number of pinned chats in the archive chat list
    #[serde(rename(deserialize = "premiumLimitTypePinnedArchivedChatCount"))]
    PinnedArchivedChatCount(PremiumLimitTypePinnedArchivedChatCount),
    /// The maximum number of pinned chats in the main chat list
    #[serde(rename(deserialize = "premiumLimitTypePinnedChatCount"))]
    PinnedChatCount(PremiumLimitTypePinnedChatCount),
    /// The maximum number of saved animations
    #[serde(rename(deserialize = "premiumLimitTypeSavedAnimationCount"))]
    SavedAnimationCount(PremiumLimitTypeSavedAnimationCount),
    /// The maximum number of joined supergroups and channels
    #[serde(rename(deserialize = "premiumLimitTypeSupergroupCount"))]
    SupergroupCount(PremiumLimitTypeSupergroupCount),
}

impl Default for PremiumLimitType {
    fn default() -> Self {
        PremiumLimitType::_Default
    }
}

impl RObject for PremiumLimitType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumLimitType::BioLength(t) => t.extra(),
            PremiumLimitType::CaptionLength(t) => t.extra(),
            PremiumLimitType::ChatFilterChosenChatCount(t) => t.extra(),
            PremiumLimitType::ChatFilterCount(t) => t.extra(),
            PremiumLimitType::CreatedPublicChatCount(t) => t.extra(),
            PremiumLimitType::FavoriteStickerCount(t) => t.extra(),
            PremiumLimitType::PinnedArchivedChatCount(t) => t.extra(),
            PremiumLimitType::PinnedChatCount(t) => t.extra(),
            PremiumLimitType::SavedAnimationCount(t) => t.extra(),
            PremiumLimitType::SupergroupCount(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumLimitType::BioLength(t) => t.client_id(),
            PremiumLimitType::CaptionLength(t) => t.client_id(),
            PremiumLimitType::ChatFilterChosenChatCount(t) => t.client_id(),
            PremiumLimitType::ChatFilterCount(t) => t.client_id(),
            PremiumLimitType::CreatedPublicChatCount(t) => t.client_id(),
            PremiumLimitType::FavoriteStickerCount(t) => t.client_id(),
            PremiumLimitType::PinnedArchivedChatCount(t) => t.client_id(),
            PremiumLimitType::PinnedChatCount(t) => t.client_id(),
            PremiumLimitType::SavedAnimationCount(t) => t.client_id(),
            PremiumLimitType::SupergroupCount(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumLimitType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PremiumLimitType::_Default)
    }
}

impl AsRef<PremiumLimitType> for PremiumLimitType {
    fn as_ref(&self) -> &PremiumLimitType {
        self
    }
}

/// The maximum length of the user's bio
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeBioLength {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeBioLength {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeBioLength {}

impl PremiumLimitTypeBioLength {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeBioLengthBuilder {
        let mut inner = PremiumLimitTypeBioLength::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeBioLengthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeBioLengthBuilder {
    inner: PremiumLimitTypeBioLength,
}

impl RTDPremiumLimitTypeBioLengthBuilder {
    pub fn build(&self) -> PremiumLimitTypeBioLength {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeBioLength> for PremiumLimitTypeBioLength {
    fn as_ref(&self) -> &PremiumLimitTypeBioLength {
        self
    }
}

impl AsRef<PremiumLimitTypeBioLength> for RTDPremiumLimitTypeBioLengthBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeBioLength {
        &self.inner
    }
}

/// The maximum length of sent media caption
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeCaptionLength {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeCaptionLength {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeCaptionLength {}

impl PremiumLimitTypeCaptionLength {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeCaptionLengthBuilder {
        let mut inner = PremiumLimitTypeCaptionLength::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeCaptionLengthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeCaptionLengthBuilder {
    inner: PremiumLimitTypeCaptionLength,
}

impl RTDPremiumLimitTypeCaptionLengthBuilder {
    pub fn build(&self) -> PremiumLimitTypeCaptionLength {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeCaptionLength> for PremiumLimitTypeCaptionLength {
    fn as_ref(&self) -> &PremiumLimitTypeCaptionLength {
        self
    }
}

impl AsRef<PremiumLimitTypeCaptionLength> for RTDPremiumLimitTypeCaptionLengthBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeCaptionLength {
        &self.inner
    }
}

/// The maximum number of pinned and always included, or always excluded chats in a chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFilterChosenChatCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeChatFilterChosenChatCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFilterChosenChatCount {}

impl PremiumLimitTypeChatFilterChosenChatCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeChatFilterChosenChatCountBuilder {
        let mut inner = PremiumLimitTypeChatFilterChosenChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeChatFilterChosenChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeChatFilterChosenChatCountBuilder {
    inner: PremiumLimitTypeChatFilterChosenChatCount,
}

impl RTDPremiumLimitTypeChatFilterChosenChatCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeChatFilterChosenChatCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeChatFilterChosenChatCount>
    for PremiumLimitTypeChatFilterChosenChatCount
{
    fn as_ref(&self) -> &PremiumLimitTypeChatFilterChosenChatCount {
        self
    }
}

impl AsRef<PremiumLimitTypeChatFilterChosenChatCount>
    for RTDPremiumLimitTypeChatFilterChosenChatCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypeChatFilterChosenChatCount {
        &self.inner
    }
}

/// The maximum number of chat filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFilterCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeChatFilterCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFilterCount {}

impl PremiumLimitTypeChatFilterCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeChatFilterCountBuilder {
        let mut inner = PremiumLimitTypeChatFilterCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeChatFilterCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeChatFilterCountBuilder {
    inner: PremiumLimitTypeChatFilterCount,
}

impl RTDPremiumLimitTypeChatFilterCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeChatFilterCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeChatFilterCount> for PremiumLimitTypeChatFilterCount {
    fn as_ref(&self) -> &PremiumLimitTypeChatFilterCount {
        self
    }
}

impl AsRef<PremiumLimitTypeChatFilterCount> for RTDPremiumLimitTypeChatFilterCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeChatFilterCount {
        &self.inner
    }
}

/// The maximum number of created public chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeCreatedPublicChatCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeCreatedPublicChatCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeCreatedPublicChatCount {}

impl PremiumLimitTypeCreatedPublicChatCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeCreatedPublicChatCountBuilder {
        let mut inner = PremiumLimitTypeCreatedPublicChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeCreatedPublicChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeCreatedPublicChatCountBuilder {
    inner: PremiumLimitTypeCreatedPublicChatCount,
}

impl RTDPremiumLimitTypeCreatedPublicChatCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeCreatedPublicChatCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeCreatedPublicChatCount> for PremiumLimitTypeCreatedPublicChatCount {
    fn as_ref(&self) -> &PremiumLimitTypeCreatedPublicChatCount {
        self
    }
}

impl AsRef<PremiumLimitTypeCreatedPublicChatCount>
    for RTDPremiumLimitTypeCreatedPublicChatCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypeCreatedPublicChatCount {
        &self.inner
    }
}

/// The maximum number of favorite stickers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeFavoriteStickerCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeFavoriteStickerCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeFavoriteStickerCount {}

impl PremiumLimitTypeFavoriteStickerCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeFavoriteStickerCountBuilder {
        let mut inner = PremiumLimitTypeFavoriteStickerCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeFavoriteStickerCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeFavoriteStickerCountBuilder {
    inner: PremiumLimitTypeFavoriteStickerCount,
}

impl RTDPremiumLimitTypeFavoriteStickerCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeFavoriteStickerCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeFavoriteStickerCount> for PremiumLimitTypeFavoriteStickerCount {
    fn as_ref(&self) -> &PremiumLimitTypeFavoriteStickerCount {
        self
    }
}

impl AsRef<PremiumLimitTypeFavoriteStickerCount>
    for RTDPremiumLimitTypeFavoriteStickerCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypeFavoriteStickerCount {
        &self.inner
    }
}

/// The maximum number of pinned chats in the archive chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypePinnedArchivedChatCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypePinnedArchivedChatCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypePinnedArchivedChatCount {}

impl PremiumLimitTypePinnedArchivedChatCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypePinnedArchivedChatCountBuilder {
        let mut inner = PremiumLimitTypePinnedArchivedChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypePinnedArchivedChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypePinnedArchivedChatCountBuilder {
    inner: PremiumLimitTypePinnedArchivedChatCount,
}

impl RTDPremiumLimitTypePinnedArchivedChatCountBuilder {
    pub fn build(&self) -> PremiumLimitTypePinnedArchivedChatCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypePinnedArchivedChatCount> for PremiumLimitTypePinnedArchivedChatCount {
    fn as_ref(&self) -> &PremiumLimitTypePinnedArchivedChatCount {
        self
    }
}

impl AsRef<PremiumLimitTypePinnedArchivedChatCount>
    for RTDPremiumLimitTypePinnedArchivedChatCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypePinnedArchivedChatCount {
        &self.inner
    }
}

/// The maximum number of pinned chats in the main chat list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypePinnedChatCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypePinnedChatCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypePinnedChatCount {}

impl PremiumLimitTypePinnedChatCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypePinnedChatCountBuilder {
        let mut inner = PremiumLimitTypePinnedChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypePinnedChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypePinnedChatCountBuilder {
    inner: PremiumLimitTypePinnedChatCount,
}

impl RTDPremiumLimitTypePinnedChatCountBuilder {
    pub fn build(&self) -> PremiumLimitTypePinnedChatCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypePinnedChatCount> for PremiumLimitTypePinnedChatCount {
    fn as_ref(&self) -> &PremiumLimitTypePinnedChatCount {
        self
    }
}

impl AsRef<PremiumLimitTypePinnedChatCount> for RTDPremiumLimitTypePinnedChatCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypePinnedChatCount {
        &self.inner
    }
}

/// The maximum number of saved animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeSavedAnimationCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeSavedAnimationCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeSavedAnimationCount {}

impl PremiumLimitTypeSavedAnimationCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeSavedAnimationCountBuilder {
        let mut inner = PremiumLimitTypeSavedAnimationCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeSavedAnimationCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeSavedAnimationCountBuilder {
    inner: PremiumLimitTypeSavedAnimationCount,
}

impl RTDPremiumLimitTypeSavedAnimationCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeSavedAnimationCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeSavedAnimationCount> for PremiumLimitTypeSavedAnimationCount {
    fn as_ref(&self) -> &PremiumLimitTypeSavedAnimationCount {
        self
    }
}

impl AsRef<PremiumLimitTypeSavedAnimationCount> for RTDPremiumLimitTypeSavedAnimationCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeSavedAnimationCount {
        &self.inner
    }
}

/// The maximum number of joined supergroups and channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeSupergroupCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeSupergroupCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeSupergroupCount {}

impl PremiumLimitTypeSupergroupCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPremiumLimitTypeSupergroupCountBuilder {
        let mut inner = PremiumLimitTypeSupergroupCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPremiumLimitTypeSupergroupCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPremiumLimitTypeSupergroupCountBuilder {
    inner: PremiumLimitTypeSupergroupCount,
}

impl RTDPremiumLimitTypeSupergroupCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeSupergroupCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeSupergroupCount> for PremiumLimitTypeSupergroupCount {
    fn as_ref(&self) -> &PremiumLimitTypeSupergroupCount {
        self
    }
}

impl AsRef<PremiumLimitTypeSupergroupCount> for RTDPremiumLimitTypeSupergroupCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeSupergroupCount {
        &self.inner
    }
}
