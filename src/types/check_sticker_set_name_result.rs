use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents result of checking whether a name can be used for a new sticker set
pub trait TDCheckStickerSetNameResult: Debug + RObject {}

/// Represents result of checking whether a name can be used for a new sticker set
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum CheckStickerSetNameResult {
    #[doc(hidden)]
    _Default,
    /// Checks whether a name can be used for a new sticker set
    #[serde(rename = "checkStickerSetName")]
    CheckStickerSetName(CheckStickerSetName),
    /// The name is invalid
    #[serde(rename = "checkStickerSetNameResultNameInvalid")]
    NameInvalid(CheckStickerSetNameResultNameInvalid),
    /// The name is occupied
    #[serde(rename = "checkStickerSetNameResultNameOccupied")]
    NameOccupied(CheckStickerSetNameResultNameOccupied),
    /// The name can be set
    #[serde(rename = "checkStickerSetNameResultOk")]
    Ok(CheckStickerSetNameResultOk),
}

impl Default for CheckStickerSetNameResult {
    fn default() -> Self {
        CheckStickerSetNameResult::_Default
    }
}

impl RObject for CheckStickerSetNameResult {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            CheckStickerSetNameResult::CheckStickerSetName(t) => t.extra(),
            CheckStickerSetNameResult::NameInvalid(t) => t.extra(),
            CheckStickerSetNameResult::NameOccupied(t) => t.extra(),
            CheckStickerSetNameResult::Ok(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            CheckStickerSetNameResult::CheckStickerSetName(t) => t.client_id(),
            CheckStickerSetNameResult::NameInvalid(t) => t.client_id(),
            CheckStickerSetNameResult::NameOccupied(t) => t.client_id(),
            CheckStickerSetNameResult::Ok(t) => t.client_id(),

            _ => None,
        }
    }
}

impl CheckStickerSetNameResult {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, CheckStickerSetNameResult::_Default)
    }
}

impl AsRef<CheckStickerSetNameResult> for CheckStickerSetNameResult {
    fn as_ref(&self) -> &CheckStickerSetNameResult {
        self
    }
}

/// The name is invalid
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckStickerSetNameResultNameInvalid {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckStickerSetNameResultNameInvalid {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckStickerSetNameResult for CheckStickerSetNameResultNameInvalid {}

impl CheckStickerSetNameResultNameInvalid {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckStickerSetNameResultNameInvalidBuilder {
        let mut inner = CheckStickerSetNameResultNameInvalid::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckStickerSetNameResultNameInvalidBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckStickerSetNameResultNameInvalidBuilder {
    inner: CheckStickerSetNameResultNameInvalid,
}

#[deprecated]
pub type RTDCheckStickerSetNameResultNameInvalidBuilder =
    CheckStickerSetNameResultNameInvalidBuilder;

impl CheckStickerSetNameResultNameInvalidBuilder {
    pub fn build(&self) -> CheckStickerSetNameResultNameInvalid {
        self.inner.clone()
    }
}

impl AsRef<CheckStickerSetNameResultNameInvalid> for CheckStickerSetNameResultNameInvalid {
    fn as_ref(&self) -> &CheckStickerSetNameResultNameInvalid {
        self
    }
}

impl AsRef<CheckStickerSetNameResultNameInvalid> for CheckStickerSetNameResultNameInvalidBuilder {
    fn as_ref(&self) -> &CheckStickerSetNameResultNameInvalid {
        &self.inner
    }
}

/// The name is occupied
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckStickerSetNameResultNameOccupied {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckStickerSetNameResultNameOccupied {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckStickerSetNameResult for CheckStickerSetNameResultNameOccupied {}

impl CheckStickerSetNameResultNameOccupied {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckStickerSetNameResultNameOccupiedBuilder {
        let mut inner = CheckStickerSetNameResultNameOccupied::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckStickerSetNameResultNameOccupiedBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckStickerSetNameResultNameOccupiedBuilder {
    inner: CheckStickerSetNameResultNameOccupied,
}

#[deprecated]
pub type RTDCheckStickerSetNameResultNameOccupiedBuilder =
    CheckStickerSetNameResultNameOccupiedBuilder;

impl CheckStickerSetNameResultNameOccupiedBuilder {
    pub fn build(&self) -> CheckStickerSetNameResultNameOccupied {
        self.inner.clone()
    }
}

impl AsRef<CheckStickerSetNameResultNameOccupied> for CheckStickerSetNameResultNameOccupied {
    fn as_ref(&self) -> &CheckStickerSetNameResultNameOccupied {
        self
    }
}

impl AsRef<CheckStickerSetNameResultNameOccupied> for CheckStickerSetNameResultNameOccupiedBuilder {
    fn as_ref(&self) -> &CheckStickerSetNameResultNameOccupied {
        &self.inner
    }
}

/// The name can be set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CheckStickerSetNameResultOk {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for CheckStickerSetNameResultOk {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCheckStickerSetNameResult for CheckStickerSetNameResultOk {}

impl CheckStickerSetNameResultOk {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CheckStickerSetNameResultOkBuilder {
        let mut inner = CheckStickerSetNameResultOk::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CheckStickerSetNameResultOkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CheckStickerSetNameResultOkBuilder {
    inner: CheckStickerSetNameResultOk,
}

#[deprecated]
pub type RTDCheckStickerSetNameResultOkBuilder = CheckStickerSetNameResultOkBuilder;

impl CheckStickerSetNameResultOkBuilder {
    pub fn build(&self) -> CheckStickerSetNameResultOk {
        self.inner.clone()
    }
}

impl AsRef<CheckStickerSetNameResultOk> for CheckStickerSetNameResultOk {
    fn as_ref(&self) -> &CheckStickerSetNameResultOk {
        self
    }
}

impl AsRef<CheckStickerSetNameResultOk> for CheckStickerSetNameResultOkBuilder {
    fn as_ref(&self) -> &CheckStickerSetNameResultOk {
        &self.inner
    }
}
