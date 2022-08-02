use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes the reason why a chat is reported
pub trait TDChatReportReason: Debug + RObject {}

/// Describes the reason why a chat is reported
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum ChatReportReason {
    #[doc(hidden)]
    _Default,
    /// The chat has child abuse related content
    #[serde(rename = "chatReportReasonChildAbuse")]
    ChildAbuse(ChatReportReasonChildAbuse),
    /// The chat contains copyrighted content
    #[serde(rename = "chatReportReasonCopyright")]
    Copyright(ChatReportReasonCopyright),
    /// A custom reason provided by the user
    #[serde(rename = "chatReportReasonCustom")]
    Custom(ChatReportReasonCustom),
    /// The chat represents a fake account
    #[serde(rename = "chatReportReasonFake")]
    Fake(ChatReportReasonFake),
    /// The chat contains pornographic messages
    #[serde(rename = "chatReportReasonPornography")]
    Pornography(ChatReportReasonPornography),
    /// The chat contains spam messages
    #[serde(rename = "chatReportReasonSpam")]
    Spam(ChatReportReasonSpam),
    /// The location-based chat is unrelated to its stated location
    #[serde(rename = "chatReportReasonUnrelatedLocation")]
    UnrelatedLocation(ChatReportReasonUnrelatedLocation),
    /// The chat promotes violence
    #[serde(rename = "chatReportReasonViolence")]
    Violence(ChatReportReasonViolence),
}

impl Default for ChatReportReason {
    fn default() -> Self {
        ChatReportReason::_Default
    }
}

impl RObject for ChatReportReason {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            ChatReportReason::ChildAbuse(t) => t.extra(),
            ChatReportReason::Copyright(t) => t.extra(),
            ChatReportReason::Custom(t) => t.extra(),
            ChatReportReason::Fake(t) => t.extra(),
            ChatReportReason::Pornography(t) => t.extra(),
            ChatReportReason::Spam(t) => t.extra(),
            ChatReportReason::UnrelatedLocation(t) => t.extra(),
            ChatReportReason::Violence(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            ChatReportReason::ChildAbuse(t) => t.client_id(),
            ChatReportReason::Copyright(t) => t.client_id(),
            ChatReportReason::Custom(t) => t.client_id(),
            ChatReportReason::Fake(t) => t.client_id(),
            ChatReportReason::Pornography(t) => t.client_id(),
            ChatReportReason::Spam(t) => t.client_id(),
            ChatReportReason::UnrelatedLocation(t) => t.client_id(),
            ChatReportReason::Violence(t) => t.client_id(),

            _ => None,
        }
    }
}

impl ChatReportReason {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, ChatReportReason::_Default)
    }
}

impl AsRef<ChatReportReason> for ChatReportReason {
    fn as_ref(&self) -> &ChatReportReason {
        self
    }
}

/// The chat has child abuse related content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonChildAbuse {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonChildAbuse {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonChildAbuse {}

impl ChatReportReasonChildAbuse {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonChildAbuseBuilder {
        let mut inner = ChatReportReasonChildAbuse::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonChildAbuseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonChildAbuseBuilder {
    inner: ChatReportReasonChildAbuse,
}

#[deprecated]
pub type RTDChatReportReasonChildAbuseBuilder = ChatReportReasonChildAbuseBuilder;

impl ChatReportReasonChildAbuseBuilder {
    pub fn build(&self) -> ChatReportReasonChildAbuse {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonChildAbuse> for ChatReportReasonChildAbuse {
    fn as_ref(&self) -> &ChatReportReasonChildAbuse {
        self
    }
}

impl AsRef<ChatReportReasonChildAbuse> for ChatReportReasonChildAbuseBuilder {
    fn as_ref(&self) -> &ChatReportReasonChildAbuse {
        &self.inner
    }
}

/// The chat contains copyrighted content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonCopyright {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonCopyright {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonCopyright {}

impl ChatReportReasonCopyright {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonCopyrightBuilder {
        let mut inner = ChatReportReasonCopyright::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonCopyrightBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonCopyrightBuilder {
    inner: ChatReportReasonCopyright,
}

#[deprecated]
pub type RTDChatReportReasonCopyrightBuilder = ChatReportReasonCopyrightBuilder;

impl ChatReportReasonCopyrightBuilder {
    pub fn build(&self) -> ChatReportReasonCopyright {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonCopyright> for ChatReportReasonCopyright {
    fn as_ref(&self) -> &ChatReportReasonCopyright {
        self
    }
}

impl AsRef<ChatReportReasonCopyright> for ChatReportReasonCopyrightBuilder {
    fn as_ref(&self) -> &ChatReportReasonCopyright {
        &self.inner
    }
}

/// A custom reason provided by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonCustom {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonCustom {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonCustom {}

impl ChatReportReasonCustom {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonCustomBuilder {
        let mut inner = ChatReportReasonCustom::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonCustomBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonCustomBuilder {
    inner: ChatReportReasonCustom,
}

#[deprecated]
pub type RTDChatReportReasonCustomBuilder = ChatReportReasonCustomBuilder;

impl ChatReportReasonCustomBuilder {
    pub fn build(&self) -> ChatReportReasonCustom {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonCustom> for ChatReportReasonCustom {
    fn as_ref(&self) -> &ChatReportReasonCustom {
        self
    }
}

impl AsRef<ChatReportReasonCustom> for ChatReportReasonCustomBuilder {
    fn as_ref(&self) -> &ChatReportReasonCustom {
        &self.inner
    }
}

/// The chat represents a fake account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonFake {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonFake {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonFake {}

impl ChatReportReasonFake {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonFakeBuilder {
        let mut inner = ChatReportReasonFake::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonFakeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonFakeBuilder {
    inner: ChatReportReasonFake,
}

#[deprecated]
pub type RTDChatReportReasonFakeBuilder = ChatReportReasonFakeBuilder;

impl ChatReportReasonFakeBuilder {
    pub fn build(&self) -> ChatReportReasonFake {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonFake> for ChatReportReasonFake {
    fn as_ref(&self) -> &ChatReportReasonFake {
        self
    }
}

impl AsRef<ChatReportReasonFake> for ChatReportReasonFakeBuilder {
    fn as_ref(&self) -> &ChatReportReasonFake {
        &self.inner
    }
}

/// The chat contains pornographic messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonPornography {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonPornography {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonPornography {}

impl ChatReportReasonPornography {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonPornographyBuilder {
        let mut inner = ChatReportReasonPornography::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonPornographyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonPornographyBuilder {
    inner: ChatReportReasonPornography,
}

#[deprecated]
pub type RTDChatReportReasonPornographyBuilder = ChatReportReasonPornographyBuilder;

impl ChatReportReasonPornographyBuilder {
    pub fn build(&self) -> ChatReportReasonPornography {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonPornography> for ChatReportReasonPornography {
    fn as_ref(&self) -> &ChatReportReasonPornography {
        self
    }
}

impl AsRef<ChatReportReasonPornography> for ChatReportReasonPornographyBuilder {
    fn as_ref(&self) -> &ChatReportReasonPornography {
        &self.inner
    }
}

/// The chat contains spam messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonSpam {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonSpam {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonSpam {}

impl ChatReportReasonSpam {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonSpamBuilder {
        let mut inner = ChatReportReasonSpam::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonSpamBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonSpamBuilder {
    inner: ChatReportReasonSpam,
}

#[deprecated]
pub type RTDChatReportReasonSpamBuilder = ChatReportReasonSpamBuilder;

impl ChatReportReasonSpamBuilder {
    pub fn build(&self) -> ChatReportReasonSpam {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonSpam> for ChatReportReasonSpam {
    fn as_ref(&self) -> &ChatReportReasonSpam {
        self
    }
}

impl AsRef<ChatReportReasonSpam> for ChatReportReasonSpamBuilder {
    fn as_ref(&self) -> &ChatReportReasonSpam {
        &self.inner
    }
}

/// The location-based chat is unrelated to its stated location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonUnrelatedLocation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonUnrelatedLocation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonUnrelatedLocation {}

impl ChatReportReasonUnrelatedLocation {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonUnrelatedLocationBuilder {
        let mut inner = ChatReportReasonUnrelatedLocation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonUnrelatedLocationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonUnrelatedLocationBuilder {
    inner: ChatReportReasonUnrelatedLocation,
}

#[deprecated]
pub type RTDChatReportReasonUnrelatedLocationBuilder = ChatReportReasonUnrelatedLocationBuilder;

impl ChatReportReasonUnrelatedLocationBuilder {
    pub fn build(&self) -> ChatReportReasonUnrelatedLocation {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonUnrelatedLocation> for ChatReportReasonUnrelatedLocation {
    fn as_ref(&self) -> &ChatReportReasonUnrelatedLocation {
        self
    }
}

impl AsRef<ChatReportReasonUnrelatedLocation> for ChatReportReasonUnrelatedLocationBuilder {
    fn as_ref(&self) -> &ChatReportReasonUnrelatedLocation {
        &self.inner
    }
}

/// The chat promotes violence
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportReasonViolence {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for ChatReportReasonViolence {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDChatReportReason for ChatReportReasonViolence {}

impl ChatReportReasonViolence {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatReportReasonViolenceBuilder {
        let mut inner = ChatReportReasonViolence::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatReportReasonViolenceBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ChatReportReasonViolenceBuilder {
    inner: ChatReportReasonViolence,
}

#[deprecated]
pub type RTDChatReportReasonViolenceBuilder = ChatReportReasonViolenceBuilder;

impl ChatReportReasonViolenceBuilder {
    pub fn build(&self) -> ChatReportReasonViolence {
        self.inner.clone()
    }
}

impl AsRef<ChatReportReasonViolence> for ChatReportReasonViolence {
    fn as_ref(&self) -> &ChatReportReasonViolence {
        self
    }
}

impl AsRef<ChatReportReasonViolence> for ChatReportReasonViolenceBuilder {
    fn as_ref(&self) -> &ChatReportReasonViolence {
        &self.inner
    }
}
