use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes type of an emoji category
pub trait TDEmojiCategoryType: Debug + RObject {}

/// Describes type of an emoji category
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum EmojiCategoryType {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The category must be used for chat photo emoji selection
    #[serde(rename = "emojiCategoryTypeChatPhoto")]
    ChatPhoto(EmojiCategoryTypeChatPhoto),
    /// The category must be used by default
    #[serde(rename = "emojiCategoryTypeDefault")]
    Default(EmojiCategoryTypeDefault),
    /// The category must be used for emoji status selection
    #[serde(rename = "emojiCategoryTypeEmojiStatus")]
    EmojiStatus(EmojiCategoryTypeEmojiStatus),
}

impl RObject for EmojiCategoryType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            EmojiCategoryType::ChatPhoto(t) => t.extra(),
            EmojiCategoryType::Default(t) => t.extra(),
            EmojiCategoryType::EmojiStatus(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            EmojiCategoryType::ChatPhoto(t) => t.client_id(),
            EmojiCategoryType::Default(t) => t.client_id(),
            EmojiCategoryType::EmojiStatus(t) => t.client_id(),

            _ => None,
        }
    }
}

impl EmojiCategoryType {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, EmojiCategoryType::_Default)
    }
}

impl AsRef<EmojiCategoryType> for EmojiCategoryType {
    fn as_ref(&self) -> &EmojiCategoryType {
        self
    }
}

/// The category must be used for chat photo emoji selection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategoryTypeChatPhoto {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for EmojiCategoryTypeChatPhoto {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmojiCategoryType for EmojiCategoryTypeChatPhoto {}

impl EmojiCategoryTypeChatPhoto {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojiCategoryTypeChatPhotoBuilder {
        let mut inner = EmojiCategoryTypeChatPhoto::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojiCategoryTypeChatPhotoBuilder { inner }
    }
}

#[doc(hidden)]
pub struct EmojiCategoryTypeChatPhotoBuilder {
    inner: EmojiCategoryTypeChatPhoto,
}

#[deprecated]
pub type RTDEmojiCategoryTypeChatPhotoBuilder = EmojiCategoryTypeChatPhotoBuilder;

impl EmojiCategoryTypeChatPhotoBuilder {
    pub fn build(&self) -> EmojiCategoryTypeChatPhoto {
        self.inner.clone()
    }
}

impl AsRef<EmojiCategoryTypeChatPhoto> for EmojiCategoryTypeChatPhoto {
    fn as_ref(&self) -> &EmojiCategoryTypeChatPhoto {
        self
    }
}

impl AsRef<EmojiCategoryTypeChatPhoto> for EmojiCategoryTypeChatPhotoBuilder {
    fn as_ref(&self) -> &EmojiCategoryTypeChatPhoto {
        &self.inner
    }
}

/// The category must be used by default
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategoryTypeDefault {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for EmojiCategoryTypeDefault {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmojiCategoryType for EmojiCategoryTypeDefault {}

impl EmojiCategoryTypeDefault {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojiCategoryTypeDefaultBuilder {
        let mut inner = EmojiCategoryTypeDefault::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojiCategoryTypeDefaultBuilder { inner }
    }
}

#[doc(hidden)]
pub struct EmojiCategoryTypeDefaultBuilder {
    inner: EmojiCategoryTypeDefault,
}

#[deprecated]
pub type RTDEmojiCategoryTypeDefaultBuilder = EmojiCategoryTypeDefaultBuilder;

impl EmojiCategoryTypeDefaultBuilder {
    pub fn build(&self) -> EmojiCategoryTypeDefault {
        self.inner.clone()
    }
}

impl AsRef<EmojiCategoryTypeDefault> for EmojiCategoryTypeDefault {
    fn as_ref(&self) -> &EmojiCategoryTypeDefault {
        self
    }
}

impl AsRef<EmojiCategoryTypeDefault> for EmojiCategoryTypeDefaultBuilder {
    fn as_ref(&self) -> &EmojiCategoryTypeDefault {
        &self.inner
    }
}

/// The category must be used for emoji status selection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategoryTypeEmojiStatus {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for EmojiCategoryTypeEmojiStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDEmojiCategoryType for EmojiCategoryTypeEmojiStatus {}

impl EmojiCategoryTypeEmojiStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojiCategoryTypeEmojiStatusBuilder {
        let mut inner = EmojiCategoryTypeEmojiStatus::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojiCategoryTypeEmojiStatusBuilder { inner }
    }
}

#[doc(hidden)]
pub struct EmojiCategoryTypeEmojiStatusBuilder {
    inner: EmojiCategoryTypeEmojiStatus,
}

#[deprecated]
pub type RTDEmojiCategoryTypeEmojiStatusBuilder = EmojiCategoryTypeEmojiStatusBuilder;

impl EmojiCategoryTypeEmojiStatusBuilder {
    pub fn build(&self) -> EmojiCategoryTypeEmojiStatus {
        self.inner.clone()
    }
}

impl AsRef<EmojiCategoryTypeEmojiStatus> for EmojiCategoryTypeEmojiStatus {
    fn as_ref(&self) -> &EmojiCategoryTypeEmojiStatus {
        self
    }
}

impl AsRef<EmojiCategoryTypeEmojiStatus> for EmojiCategoryTypeEmojiStatusBuilder {
    fn as_ref(&self) -> &EmojiCategoryTypeEmojiStatus {
        &self.inner
    }
}
