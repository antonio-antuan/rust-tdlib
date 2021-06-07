use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the type of a URL linking to an internal Telegram entity
pub trait TDTMeUrlType: Debug + RObject {}

/// Describes the type of a URL linking to an internal Telegram entity
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum TMeUrlType {
    #[doc(hidden)]
    _Default,
    /// A chat invite link
    #[serde(rename(
        serialize = "tMeUrlTypeChatInvite",
        deserialize = "tMeUrlTypeChatInvite"
    ))]
    ChatInvite(TMeUrlTypeChatInvite),
    /// A URL linking to a sticker set
    #[serde(rename(
        serialize = "tMeUrlTypeStickerSet",
        deserialize = "tMeUrlTypeStickerSet"
    ))]
    StickerSet(TMeUrlTypeStickerSet),
    /// A URL linking to a public supergroup or channel
    #[serde(rename(
        serialize = "tMeUrlTypeSupergroup",
        deserialize = "tMeUrlTypeSupergroup"
    ))]
    Supergroup(TMeUrlTypeSupergroup),
    /// A URL linking to a user
    #[serde(rename(serialize = "tMeUrlTypeUser", deserialize = "tMeUrlTypeUser"))]
    User(TMeUrlTypeUser),
}

impl Default for TMeUrlType {
    fn default() -> Self {
        TMeUrlType::_Default
    }
}

impl RObject for TMeUrlType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            TMeUrlType::ChatInvite(t) => t.extra(),
            TMeUrlType::StickerSet(t) => t.extra(),
            TMeUrlType::Supergroup(t) => t.extra(),
            TMeUrlType::User(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            TMeUrlType::ChatInvite(t) => t.client_id(),
            TMeUrlType::StickerSet(t) => t.client_id(),
            TMeUrlType::Supergroup(t) => t.client_id(),
            TMeUrlType::User(t) => t.client_id(),

            _ => None,
        }
    }
}

impl TMeUrlType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, TMeUrlType::_Default)
    }
}

impl AsRef<TMeUrlType> for TMeUrlType {
    fn as_ref(&self) -> &TMeUrlType {
        self
    }
}

/// A chat invite link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeChatInvite {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat invite link info
    info: ChatInviteLinkInfo,
}

impl RObject for TMeUrlTypeChatInvite {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTMeUrlType for TMeUrlTypeChatInvite {}

impl TMeUrlTypeChatInvite {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTMeUrlTypeChatInviteBuilder {
        let mut inner = TMeUrlTypeChatInvite::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTMeUrlTypeChatInviteBuilder { inner }
    }

    pub fn info(&self) -> &ChatInviteLinkInfo {
        &self.info
    }
}

#[doc(hidden)]
pub struct RTDTMeUrlTypeChatInviteBuilder {
    inner: TMeUrlTypeChatInvite,
}

impl RTDTMeUrlTypeChatInviteBuilder {
    pub fn build(&self) -> TMeUrlTypeChatInvite {
        self.inner.clone()
    }

    pub fn info<T: AsRef<ChatInviteLinkInfo>>(&mut self, info: T) -> &mut Self {
        self.inner.info = info.as_ref().clone();
        self
    }
}

impl AsRef<TMeUrlTypeChatInvite> for TMeUrlTypeChatInvite {
    fn as_ref(&self) -> &TMeUrlTypeChatInvite {
        self
    }
}

impl AsRef<TMeUrlTypeChatInvite> for RTDTMeUrlTypeChatInviteBuilder {
    fn as_ref(&self) -> &TMeUrlTypeChatInvite {
        &self.inner
    }
}

/// A URL linking to a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeStickerSet {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the sticker set

    #[serde(deserialize_with = "super::_common::number_from_string")]
    sticker_set_id: i64,
}

impl RObject for TMeUrlTypeStickerSet {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTMeUrlType for TMeUrlTypeStickerSet {}

impl TMeUrlTypeStickerSet {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTMeUrlTypeStickerSetBuilder {
        let mut inner = TMeUrlTypeStickerSet::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTMeUrlTypeStickerSetBuilder { inner }
    }

    pub fn sticker_set_id(&self) -> i64 {
        self.sticker_set_id
    }
}

#[doc(hidden)]
pub struct RTDTMeUrlTypeStickerSetBuilder {
    inner: TMeUrlTypeStickerSet,
}

impl RTDTMeUrlTypeStickerSetBuilder {
    pub fn build(&self) -> TMeUrlTypeStickerSet {
        self.inner.clone()
    }

    pub fn sticker_set_id(&mut self, sticker_set_id: i64) -> &mut Self {
        self.inner.sticker_set_id = sticker_set_id;
        self
    }
}

impl AsRef<TMeUrlTypeStickerSet> for TMeUrlTypeStickerSet {
    fn as_ref(&self) -> &TMeUrlTypeStickerSet {
        self
    }
}

impl AsRef<TMeUrlTypeStickerSet> for RTDTMeUrlTypeStickerSetBuilder {
    fn as_ref(&self) -> &TMeUrlTypeStickerSet {
        &self.inner
    }
}

/// A URL linking to a public supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeSupergroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel
    supergroup_id: i64,
}

impl RObject for TMeUrlTypeSupergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTMeUrlType for TMeUrlTypeSupergroup {}

impl TMeUrlTypeSupergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTMeUrlTypeSupergroupBuilder {
        let mut inner = TMeUrlTypeSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTMeUrlTypeSupergroupBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct RTDTMeUrlTypeSupergroupBuilder {
    inner: TMeUrlTypeSupergroup,
}

impl RTDTMeUrlTypeSupergroupBuilder {
    pub fn build(&self) -> TMeUrlTypeSupergroup {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<TMeUrlTypeSupergroup> for TMeUrlTypeSupergroup {
    fn as_ref(&self) -> &TMeUrlTypeSupergroup {
        self
    }
}

impl AsRef<TMeUrlTypeSupergroup> for RTDTMeUrlTypeSupergroupBuilder {
    fn as_ref(&self) -> &TMeUrlTypeSupergroup {
        &self.inner
    }
}

/// A URL linking to a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TMeUrlTypeUser {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user
    user_id: i32,
}

impl RObject for TMeUrlTypeUser {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDTMeUrlType for TMeUrlTypeUser {}

impl TMeUrlTypeUser {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTMeUrlTypeUserBuilder {
        let mut inner = TMeUrlTypeUser::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTMeUrlTypeUserBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDTMeUrlTypeUserBuilder {
    inner: TMeUrlTypeUser,
}

impl RTDTMeUrlTypeUserBuilder {
    pub fn build(&self) -> TMeUrlTypeUser {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<TMeUrlTypeUser> for TMeUrlTypeUser {
    fn as_ref(&self) -> &TMeUrlTypeUser {
        self
    }
}

impl AsRef<TMeUrlTypeUser> for RTDTMeUrlTypeUserBuilder {
    fn as_ref(&self) -> &TMeUrlTypeUser {
        &self.inner
    }
}
