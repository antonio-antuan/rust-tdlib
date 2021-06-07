use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a horizontal alignment of a table cell content
pub trait TDPageBlockHorizontalAlignment: Debug + RObject {}

/// Describes a horizontal alignment of a table cell content
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockHorizontalAlignment {
    #[doc(hidden)]
    _Default,
    /// The content should be center-aligned
    #[serde(rename(
        serialize = "pageBlockHorizontalAlignmentCenter",
        deserialize = "pageBlockHorizontalAlignmentCenter"
    ))]
    Center(PageBlockHorizontalAlignmentCenter),
    /// The content should be left-aligned
    #[serde(rename(
        serialize = "pageBlockHorizontalAlignmentLeft",
        deserialize = "pageBlockHorizontalAlignmentLeft"
    ))]
    Left(PageBlockHorizontalAlignmentLeft),
    /// The content should be right-aligned
    #[serde(rename(
        serialize = "pageBlockHorizontalAlignmentRight",
        deserialize = "pageBlockHorizontalAlignmentRight"
    ))]
    Right(PageBlockHorizontalAlignmentRight),
}

impl Default for PageBlockHorizontalAlignment {
    fn default() -> Self {
        PageBlockHorizontalAlignment::_Default
    }
}

impl RObject for PageBlockHorizontalAlignment {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PageBlockHorizontalAlignment::Center(t) => t.extra(),
            PageBlockHorizontalAlignment::Left(t) => t.extra(),
            PageBlockHorizontalAlignment::Right(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PageBlockHorizontalAlignment::Center(t) => t.client_id(),
            PageBlockHorizontalAlignment::Left(t) => t.client_id(),
            PageBlockHorizontalAlignment::Right(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PageBlockHorizontalAlignment {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PageBlockHorizontalAlignment::_Default)
    }
}

impl AsRef<PageBlockHorizontalAlignment> for PageBlockHorizontalAlignment {
    fn as_ref(&self) -> &PageBlockHorizontalAlignment {
        self
    }
}

/// The content should be center-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentCenter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockHorizontalAlignmentCenter {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentCenter {}

impl PageBlockHorizontalAlignmentCenter {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockHorizontalAlignmentCenterBuilder {
        let mut inner = PageBlockHorizontalAlignmentCenter::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockHorizontalAlignmentCenterBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPageBlockHorizontalAlignmentCenterBuilder {
    inner: PageBlockHorizontalAlignmentCenter,
}

impl RTDPageBlockHorizontalAlignmentCenterBuilder {
    pub fn build(&self) -> PageBlockHorizontalAlignmentCenter {
        self.inner.clone()
    }
}

impl AsRef<PageBlockHorizontalAlignmentCenter> for PageBlockHorizontalAlignmentCenter {
    fn as_ref(&self) -> &PageBlockHorizontalAlignmentCenter {
        self
    }
}

impl AsRef<PageBlockHorizontalAlignmentCenter> for RTDPageBlockHorizontalAlignmentCenterBuilder {
    fn as_ref(&self) -> &PageBlockHorizontalAlignmentCenter {
        &self.inner
    }
}

/// The content should be left-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentLeft {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockHorizontalAlignmentLeft {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentLeft {}

impl PageBlockHorizontalAlignmentLeft {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockHorizontalAlignmentLeftBuilder {
        let mut inner = PageBlockHorizontalAlignmentLeft::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockHorizontalAlignmentLeftBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPageBlockHorizontalAlignmentLeftBuilder {
    inner: PageBlockHorizontalAlignmentLeft,
}

impl RTDPageBlockHorizontalAlignmentLeftBuilder {
    pub fn build(&self) -> PageBlockHorizontalAlignmentLeft {
        self.inner.clone()
    }
}

impl AsRef<PageBlockHorizontalAlignmentLeft> for PageBlockHorizontalAlignmentLeft {
    fn as_ref(&self) -> &PageBlockHorizontalAlignmentLeft {
        self
    }
}

impl AsRef<PageBlockHorizontalAlignmentLeft> for RTDPageBlockHorizontalAlignmentLeftBuilder {
    fn as_ref(&self) -> &PageBlockHorizontalAlignmentLeft {
        &self.inner
    }
}

/// The content should be right-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockHorizontalAlignmentRight {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockHorizontalAlignmentRight {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentRight {}

impl PageBlockHorizontalAlignmentRight {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockHorizontalAlignmentRightBuilder {
        let mut inner = PageBlockHorizontalAlignmentRight::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockHorizontalAlignmentRightBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPageBlockHorizontalAlignmentRightBuilder {
    inner: PageBlockHorizontalAlignmentRight,
}

impl RTDPageBlockHorizontalAlignmentRightBuilder {
    pub fn build(&self) -> PageBlockHorizontalAlignmentRight {
        self.inner.clone()
    }
}

impl AsRef<PageBlockHorizontalAlignmentRight> for PageBlockHorizontalAlignmentRight {
    fn as_ref(&self) -> &PageBlockHorizontalAlignmentRight {
        self
    }
}

impl AsRef<PageBlockHorizontalAlignmentRight> for RTDPageBlockHorizontalAlignmentRightBuilder {
    fn as_ref(&self) -> &PageBlockHorizontalAlignmentRight {
        &self.inner
    }
}
