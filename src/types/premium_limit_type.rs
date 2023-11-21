use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of a limit, increased for Premium users
pub trait TDPremiumLimitType: Debug + RObject {}

/// Describes type of a limit, increased for Premium users
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum PremiumLimitType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The maximum number of active stories
    #[serde(rename = "premiumLimitTypeActiveStoryCount")]
    ActiveStoryCount(PremiumLimitTypeActiveStoryCount),
    /// The maximum length of the user's bio
    #[serde(rename = "premiumLimitTypeBioLength")]
    BioLength(PremiumLimitTypeBioLength),
    /// The maximum length of sent media caption
    #[serde(rename = "premiumLimitTypeCaptionLength")]
    CaptionLength(PremiumLimitTypeCaptionLength),
    /// The maximum number of pinned and always included, or always excluded chats in a chat folder
    #[serde(rename = "premiumLimitTypeChatFolderChosenChatCount")]
    ChatFolderChosenChatCount(PremiumLimitTypeChatFolderChosenChatCount),
    /// The maximum number of chat folders
    #[serde(rename = "premiumLimitTypeChatFolderCount")]
    ChatFolderCount(PremiumLimitTypeChatFolderCount),
    /// The maximum number of invite links for a chat folder
    #[serde(rename = "premiumLimitTypeChatFolderInviteLinkCount")]
    ChatFolderInviteLinkCount(PremiumLimitTypeChatFolderInviteLinkCount),
    /// The maximum number of created public chats
    #[serde(rename = "premiumLimitTypeCreatedPublicChatCount")]
    CreatedPublicChatCount(PremiumLimitTypeCreatedPublicChatCount),
    /// The maximum number of favorite stickers
    #[serde(rename = "premiumLimitTypeFavoriteStickerCount")]
    FavoriteStickerCount(PremiumLimitTypeFavoriteStickerCount),
    /// The maximum number of stories sent per month
    #[serde(rename = "premiumLimitTypeMonthlySentStoryCount")]
    MonthlySentStoryCount(PremiumLimitTypeMonthlySentStoryCount),
    /// The maximum number of pinned chats in the archive chat list
    #[serde(rename = "premiumLimitTypePinnedArchivedChatCount")]
    PinnedArchivedChatCount(PremiumLimitTypePinnedArchivedChatCount),
    /// The maximum number of pinned chats in the main chat list
    #[serde(rename = "premiumLimitTypePinnedChatCount")]
    PinnedChatCount(PremiumLimitTypePinnedChatCount),
    /// The maximum number of saved animations
    #[serde(rename = "premiumLimitTypeSavedAnimationCount")]
    SavedAnimationCount(PremiumLimitTypeSavedAnimationCount),
    /// The maximum number of added shareable chat folders
    #[serde(rename = "premiumLimitTypeShareableChatFolderCount")]
    ShareableChatFolderCount(PremiumLimitTypeShareableChatFolderCount),
    /// The maximum length of captions of sent stories
    #[serde(rename = "premiumLimitTypeStoryCaptionLength")]
    StoryCaptionLength(PremiumLimitTypeStoryCaptionLength),
    /// The maximum number of suggested reaction areas on a story
    #[serde(rename = "premiumLimitTypeStorySuggestedReactionAreaCount")]
    StorySuggestedReactionAreaCount(PremiumLimitTypeStorySuggestedReactionAreaCount),
    /// The maximum number of joined supergroups and channels
    #[serde(rename = "premiumLimitTypeSupergroupCount")]
    SupergroupCount(PremiumLimitTypeSupergroupCount),
    /// The maximum number of stories sent per week
    #[serde(rename = "premiumLimitTypeWeeklySentStoryCount")]
    WeeklySentStoryCount(PremiumLimitTypeWeeklySentStoryCount),
}

impl RObject for PremiumLimitType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumLimitType::ActiveStoryCount(t) => t.extra(),
            PremiumLimitType::BioLength(t) => t.extra(),
            PremiumLimitType::CaptionLength(t) => t.extra(),
            PremiumLimitType::ChatFolderChosenChatCount(t) => t.extra(),
            PremiumLimitType::ChatFolderCount(t) => t.extra(),
            PremiumLimitType::ChatFolderInviteLinkCount(t) => t.extra(),
            PremiumLimitType::CreatedPublicChatCount(t) => t.extra(),
            PremiumLimitType::FavoriteStickerCount(t) => t.extra(),
            PremiumLimitType::MonthlySentStoryCount(t) => t.extra(),
            PremiumLimitType::PinnedArchivedChatCount(t) => t.extra(),
            PremiumLimitType::PinnedChatCount(t) => t.extra(),
            PremiumLimitType::SavedAnimationCount(t) => t.extra(),
            PremiumLimitType::ShareableChatFolderCount(t) => t.extra(),
            PremiumLimitType::StoryCaptionLength(t) => t.extra(),
            PremiumLimitType::StorySuggestedReactionAreaCount(t) => t.extra(),
            PremiumLimitType::SupergroupCount(t) => t.extra(),
            PremiumLimitType::WeeklySentStoryCount(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumLimitType::ActiveStoryCount(t) => t.client_id(),
            PremiumLimitType::BioLength(t) => t.client_id(),
            PremiumLimitType::CaptionLength(t) => t.client_id(),
            PremiumLimitType::ChatFolderChosenChatCount(t) => t.client_id(),
            PremiumLimitType::ChatFolderCount(t) => t.client_id(),
            PremiumLimitType::ChatFolderInviteLinkCount(t) => t.client_id(),
            PremiumLimitType::CreatedPublicChatCount(t) => t.client_id(),
            PremiumLimitType::FavoriteStickerCount(t) => t.client_id(),
            PremiumLimitType::MonthlySentStoryCount(t) => t.client_id(),
            PremiumLimitType::PinnedArchivedChatCount(t) => t.client_id(),
            PremiumLimitType::PinnedChatCount(t) => t.client_id(),
            PremiumLimitType::SavedAnimationCount(t) => t.client_id(),
            PremiumLimitType::ShareableChatFolderCount(t) => t.client_id(),
            PremiumLimitType::StoryCaptionLength(t) => t.client_id(),
            PremiumLimitType::StorySuggestedReactionAreaCount(t) => t.client_id(),
            PremiumLimitType::SupergroupCount(t) => t.client_id(),
            PremiumLimitType::WeeklySentStoryCount(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumLimitType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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

/// The maximum number of active stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeActiveStoryCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeActiveStoryCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeActiveStoryCount {}

impl PremiumLimitTypeActiveStoryCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeActiveStoryCountBuilder {
        let mut inner = PremiumLimitTypeActiveStoryCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeActiveStoryCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeActiveStoryCountBuilder {
    inner: PremiumLimitTypeActiveStoryCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeActiveStoryCountBuilder = PremiumLimitTypeActiveStoryCountBuilder;

impl PremiumLimitTypeActiveStoryCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeActiveStoryCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeActiveStoryCount> for PremiumLimitTypeActiveStoryCount {
    fn as_ref(&self) -> &PremiumLimitTypeActiveStoryCount {
        self
    }
}

impl AsRef<PremiumLimitTypeActiveStoryCount> for PremiumLimitTypeActiveStoryCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeActiveStoryCount {
        &self.inner
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeBioLengthBuilder {
        let mut inner = PremiumLimitTypeBioLength::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeBioLengthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeBioLengthBuilder {
    inner: PremiumLimitTypeBioLength,
}

#[deprecated]
pub type RTDPremiumLimitTypeBioLengthBuilder = PremiumLimitTypeBioLengthBuilder;

impl PremiumLimitTypeBioLengthBuilder {
    pub fn build(&self) -> PremiumLimitTypeBioLength {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeBioLength> for PremiumLimitTypeBioLength {
    fn as_ref(&self) -> &PremiumLimitTypeBioLength {
        self
    }
}

impl AsRef<PremiumLimitTypeBioLength> for PremiumLimitTypeBioLengthBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeCaptionLengthBuilder {
        let mut inner = PremiumLimitTypeCaptionLength::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeCaptionLengthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeCaptionLengthBuilder {
    inner: PremiumLimitTypeCaptionLength,
}

#[deprecated]
pub type RTDPremiumLimitTypeCaptionLengthBuilder = PremiumLimitTypeCaptionLengthBuilder;

impl PremiumLimitTypeCaptionLengthBuilder {
    pub fn build(&self) -> PremiumLimitTypeCaptionLength {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeCaptionLength> for PremiumLimitTypeCaptionLength {
    fn as_ref(&self) -> &PremiumLimitTypeCaptionLength {
        self
    }
}

impl AsRef<PremiumLimitTypeCaptionLength> for PremiumLimitTypeCaptionLengthBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeCaptionLength {
        &self.inner
    }
}

/// The maximum number of pinned and always included, or always excluded chats in a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFolderChosenChatCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeChatFolderChosenChatCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFolderChosenChatCount {}

impl PremiumLimitTypeChatFolderChosenChatCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeChatFolderChosenChatCountBuilder {
        let mut inner = PremiumLimitTypeChatFolderChosenChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeChatFolderChosenChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeChatFolderChosenChatCountBuilder {
    inner: PremiumLimitTypeChatFolderChosenChatCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeChatFolderChosenChatCountBuilder =
    PremiumLimitTypeChatFolderChosenChatCountBuilder;

impl PremiumLimitTypeChatFolderChosenChatCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeChatFolderChosenChatCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeChatFolderChosenChatCount>
    for PremiumLimitTypeChatFolderChosenChatCount
{
    fn as_ref(&self) -> &PremiumLimitTypeChatFolderChosenChatCount {
        self
    }
}

impl AsRef<PremiumLimitTypeChatFolderChosenChatCount>
    for PremiumLimitTypeChatFolderChosenChatCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypeChatFolderChosenChatCount {
        &self.inner
    }
}

/// The maximum number of chat folders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFolderCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeChatFolderCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFolderCount {}

impl PremiumLimitTypeChatFolderCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeChatFolderCountBuilder {
        let mut inner = PremiumLimitTypeChatFolderCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeChatFolderCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeChatFolderCountBuilder {
    inner: PremiumLimitTypeChatFolderCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeChatFolderCountBuilder = PremiumLimitTypeChatFolderCountBuilder;

impl PremiumLimitTypeChatFolderCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeChatFolderCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeChatFolderCount> for PremiumLimitTypeChatFolderCount {
    fn as_ref(&self) -> &PremiumLimitTypeChatFolderCount {
        self
    }
}

impl AsRef<PremiumLimitTypeChatFolderCount> for PremiumLimitTypeChatFolderCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeChatFolderCount {
        &self.inner
    }
}

/// The maximum number of invite links for a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeChatFolderInviteLinkCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeChatFolderInviteLinkCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeChatFolderInviteLinkCount {}

impl PremiumLimitTypeChatFolderInviteLinkCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeChatFolderInviteLinkCountBuilder {
        let mut inner = PremiumLimitTypeChatFolderInviteLinkCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeChatFolderInviteLinkCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeChatFolderInviteLinkCountBuilder {
    inner: PremiumLimitTypeChatFolderInviteLinkCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeChatFolderInviteLinkCountBuilder =
    PremiumLimitTypeChatFolderInviteLinkCountBuilder;

impl PremiumLimitTypeChatFolderInviteLinkCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeChatFolderInviteLinkCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeChatFolderInviteLinkCount>
    for PremiumLimitTypeChatFolderInviteLinkCount
{
    fn as_ref(&self) -> &PremiumLimitTypeChatFolderInviteLinkCount {
        self
    }
}

impl AsRef<PremiumLimitTypeChatFolderInviteLinkCount>
    for PremiumLimitTypeChatFolderInviteLinkCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypeChatFolderInviteLinkCount {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeCreatedPublicChatCountBuilder {
        let mut inner = PremiumLimitTypeCreatedPublicChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeCreatedPublicChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeCreatedPublicChatCountBuilder {
    inner: PremiumLimitTypeCreatedPublicChatCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeCreatedPublicChatCountBuilder =
    PremiumLimitTypeCreatedPublicChatCountBuilder;

impl PremiumLimitTypeCreatedPublicChatCountBuilder {
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
    for PremiumLimitTypeCreatedPublicChatCountBuilder
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeFavoriteStickerCountBuilder {
        let mut inner = PremiumLimitTypeFavoriteStickerCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeFavoriteStickerCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeFavoriteStickerCountBuilder {
    inner: PremiumLimitTypeFavoriteStickerCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeFavoriteStickerCountBuilder =
    PremiumLimitTypeFavoriteStickerCountBuilder;

impl PremiumLimitTypeFavoriteStickerCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeFavoriteStickerCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeFavoriteStickerCount> for PremiumLimitTypeFavoriteStickerCount {
    fn as_ref(&self) -> &PremiumLimitTypeFavoriteStickerCount {
        self
    }
}

impl AsRef<PremiumLimitTypeFavoriteStickerCount> for PremiumLimitTypeFavoriteStickerCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeFavoriteStickerCount {
        &self.inner
    }
}

/// The maximum number of stories sent per month
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeMonthlySentStoryCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeMonthlySentStoryCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeMonthlySentStoryCount {}

impl PremiumLimitTypeMonthlySentStoryCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeMonthlySentStoryCountBuilder {
        let mut inner = PremiumLimitTypeMonthlySentStoryCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeMonthlySentStoryCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeMonthlySentStoryCountBuilder {
    inner: PremiumLimitTypeMonthlySentStoryCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeMonthlySentStoryCountBuilder =
    PremiumLimitTypeMonthlySentStoryCountBuilder;

impl PremiumLimitTypeMonthlySentStoryCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeMonthlySentStoryCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeMonthlySentStoryCount> for PremiumLimitTypeMonthlySentStoryCount {
    fn as_ref(&self) -> &PremiumLimitTypeMonthlySentStoryCount {
        self
    }
}

impl AsRef<PremiumLimitTypeMonthlySentStoryCount> for PremiumLimitTypeMonthlySentStoryCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeMonthlySentStoryCount {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypePinnedArchivedChatCountBuilder {
        let mut inner = PremiumLimitTypePinnedArchivedChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypePinnedArchivedChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypePinnedArchivedChatCountBuilder {
    inner: PremiumLimitTypePinnedArchivedChatCount,
}

#[deprecated]
pub type RTDPremiumLimitTypePinnedArchivedChatCountBuilder =
    PremiumLimitTypePinnedArchivedChatCountBuilder;

impl PremiumLimitTypePinnedArchivedChatCountBuilder {
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
    for PremiumLimitTypePinnedArchivedChatCountBuilder
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypePinnedChatCountBuilder {
        let mut inner = PremiumLimitTypePinnedChatCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypePinnedChatCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypePinnedChatCountBuilder {
    inner: PremiumLimitTypePinnedChatCount,
}

#[deprecated]
pub type RTDPremiumLimitTypePinnedChatCountBuilder = PremiumLimitTypePinnedChatCountBuilder;

impl PremiumLimitTypePinnedChatCountBuilder {
    pub fn build(&self) -> PremiumLimitTypePinnedChatCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypePinnedChatCount> for PremiumLimitTypePinnedChatCount {
    fn as_ref(&self) -> &PremiumLimitTypePinnedChatCount {
        self
    }
}

impl AsRef<PremiumLimitTypePinnedChatCount> for PremiumLimitTypePinnedChatCountBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeSavedAnimationCountBuilder {
        let mut inner = PremiumLimitTypeSavedAnimationCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeSavedAnimationCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeSavedAnimationCountBuilder {
    inner: PremiumLimitTypeSavedAnimationCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeSavedAnimationCountBuilder = PremiumLimitTypeSavedAnimationCountBuilder;

impl PremiumLimitTypeSavedAnimationCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeSavedAnimationCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeSavedAnimationCount> for PremiumLimitTypeSavedAnimationCount {
    fn as_ref(&self) -> &PremiumLimitTypeSavedAnimationCount {
        self
    }
}

impl AsRef<PremiumLimitTypeSavedAnimationCount> for PremiumLimitTypeSavedAnimationCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeSavedAnimationCount {
        &self.inner
    }
}

/// The maximum number of added shareable chat folders
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeShareableChatFolderCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeShareableChatFolderCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeShareableChatFolderCount {}

impl PremiumLimitTypeShareableChatFolderCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeShareableChatFolderCountBuilder {
        let mut inner = PremiumLimitTypeShareableChatFolderCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeShareableChatFolderCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeShareableChatFolderCountBuilder {
    inner: PremiumLimitTypeShareableChatFolderCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeShareableChatFolderCountBuilder =
    PremiumLimitTypeShareableChatFolderCountBuilder;

impl PremiumLimitTypeShareableChatFolderCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeShareableChatFolderCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeShareableChatFolderCount> for PremiumLimitTypeShareableChatFolderCount {
    fn as_ref(&self) -> &PremiumLimitTypeShareableChatFolderCount {
        self
    }
}

impl AsRef<PremiumLimitTypeShareableChatFolderCount>
    for PremiumLimitTypeShareableChatFolderCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypeShareableChatFolderCount {
        &self.inner
    }
}

/// The maximum length of captions of sent stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeStoryCaptionLength {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeStoryCaptionLength {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeStoryCaptionLength {}

impl PremiumLimitTypeStoryCaptionLength {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeStoryCaptionLengthBuilder {
        let mut inner = PremiumLimitTypeStoryCaptionLength::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeStoryCaptionLengthBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeStoryCaptionLengthBuilder {
    inner: PremiumLimitTypeStoryCaptionLength,
}

#[deprecated]
pub type RTDPremiumLimitTypeStoryCaptionLengthBuilder = PremiumLimitTypeStoryCaptionLengthBuilder;

impl PremiumLimitTypeStoryCaptionLengthBuilder {
    pub fn build(&self) -> PremiumLimitTypeStoryCaptionLength {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeStoryCaptionLength> for PremiumLimitTypeStoryCaptionLength {
    fn as_ref(&self) -> &PremiumLimitTypeStoryCaptionLength {
        self
    }
}

impl AsRef<PremiumLimitTypeStoryCaptionLength> for PremiumLimitTypeStoryCaptionLengthBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeStoryCaptionLength {
        &self.inner
    }
}

/// The maximum number of suggested reaction areas on a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeStorySuggestedReactionAreaCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeStorySuggestedReactionAreaCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeStorySuggestedReactionAreaCount {}

impl PremiumLimitTypeStorySuggestedReactionAreaCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeStorySuggestedReactionAreaCountBuilder {
        let mut inner = PremiumLimitTypeStorySuggestedReactionAreaCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeStorySuggestedReactionAreaCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeStorySuggestedReactionAreaCountBuilder {
    inner: PremiumLimitTypeStorySuggestedReactionAreaCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeStorySuggestedReactionAreaCountBuilder =
    PremiumLimitTypeStorySuggestedReactionAreaCountBuilder;

impl PremiumLimitTypeStorySuggestedReactionAreaCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeStorySuggestedReactionAreaCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeStorySuggestedReactionAreaCount>
    for PremiumLimitTypeStorySuggestedReactionAreaCount
{
    fn as_ref(&self) -> &PremiumLimitTypeStorySuggestedReactionAreaCount {
        self
    }
}

impl AsRef<PremiumLimitTypeStorySuggestedReactionAreaCount>
    for PremiumLimitTypeStorySuggestedReactionAreaCountBuilder
{
    fn as_ref(&self) -> &PremiumLimitTypeStorySuggestedReactionAreaCount {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeSupergroupCountBuilder {
        let mut inner = PremiumLimitTypeSupergroupCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeSupergroupCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeSupergroupCountBuilder {
    inner: PremiumLimitTypeSupergroupCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeSupergroupCountBuilder = PremiumLimitTypeSupergroupCountBuilder;

impl PremiumLimitTypeSupergroupCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeSupergroupCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeSupergroupCount> for PremiumLimitTypeSupergroupCount {
    fn as_ref(&self) -> &PremiumLimitTypeSupergroupCount {
        self
    }
}

impl AsRef<PremiumLimitTypeSupergroupCount> for PremiumLimitTypeSupergroupCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeSupergroupCount {
        &self.inner
    }
}

/// The maximum number of stories sent per week
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumLimitTypeWeeklySentStoryCount {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumLimitTypeWeeklySentStoryCount {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumLimitType for PremiumLimitTypeWeeklySentStoryCount {}

impl PremiumLimitTypeWeeklySentStoryCount {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumLimitTypeWeeklySentStoryCountBuilder {
        let mut inner = PremiumLimitTypeWeeklySentStoryCount::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumLimitTypeWeeklySentStoryCountBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumLimitTypeWeeklySentStoryCountBuilder {
    inner: PremiumLimitTypeWeeklySentStoryCount,
}

#[deprecated]
pub type RTDPremiumLimitTypeWeeklySentStoryCountBuilder =
    PremiumLimitTypeWeeklySentStoryCountBuilder;

impl PremiumLimitTypeWeeklySentStoryCountBuilder {
    pub fn build(&self) -> PremiumLimitTypeWeeklySentStoryCount {
        self.inner.clone()
    }
}

impl AsRef<PremiumLimitTypeWeeklySentStoryCount> for PremiumLimitTypeWeeklySentStoryCount {
    fn as_ref(&self) -> &PremiumLimitTypeWeeklySentStoryCount {
        self
    }
}

impl AsRef<PremiumLimitTypeWeeklySentStoryCount> for PremiumLimitTypeWeeklySentStoryCountBuilder {
    fn as_ref(&self) -> &PremiumLimitTypeWeeklySentStoryCount {
        &self.inner
    }
}
