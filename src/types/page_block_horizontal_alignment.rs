use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a horizontal alignment of a table cell content
pub trait TDPageBlockHorizontalAlignment: Debug + RObject {}

/// Describes a horizontal alignment of a table cell content
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageBlockHorizontalAlignment {
    #[doc(hidden)]
    _Default(()),
    /// The content should be center-aligned
    Center(PageBlockHorizontalAlignmentCenter),
    /// The content should be left-aligned
    Left(PageBlockHorizontalAlignmentLeft),
    /// The content should be right-aligned
    Right(PageBlockHorizontalAlignmentRight),
}

impl Default for PageBlockHorizontalAlignment {
    fn default() -> Self {
        PageBlockHorizontalAlignment::_Default(())
    }
}

impl<'de> Deserialize<'de> for PageBlockHorizontalAlignment {
    fn deserialize<D>(deserializer: D) -> Result<PageBlockHorizontalAlignment, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          PageBlockHorizontalAlignment,
          (pageBlockHorizontalAlignmentCenter, Center);
          (pageBlockHorizontalAlignmentLeft, Left);
          (pageBlockHorizontalAlignmentRight, Right);

        )(deserializer)
    }
}

impl RObject for PageBlockHorizontalAlignment {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            PageBlockHorizontalAlignment::Center(t) => t.td_name(),
            PageBlockHorizontalAlignment::Left(t) => t.td_name(),
            PageBlockHorizontalAlignment::Right(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            PageBlockHorizontalAlignment::Center(t) => t.extra(),
            PageBlockHorizontalAlignment::Left(t) => t.extra(),
            PageBlockHorizontalAlignment::Right(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl PageBlockHorizontalAlignment {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PageBlockHorizontalAlignment::_Default(_))
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PageBlockHorizontalAlignmentCenter {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "pageBlockHorizontalAlignmentCenter"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentCenter {}

impl PageBlockHorizontalAlignmentCenter {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockHorizontalAlignmentCenterBuilder {
        let mut inner = PageBlockHorizontalAlignmentCenter::default();
        inner.td_name = "pageBlockHorizontalAlignmentCenter".to_string();
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PageBlockHorizontalAlignmentLeft {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "pageBlockHorizontalAlignmentLeft"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentLeft {}

impl PageBlockHorizontalAlignmentLeft {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockHorizontalAlignmentLeftBuilder {
        let mut inner = PageBlockHorizontalAlignmentLeft::default();
        inner.td_name = "pageBlockHorizontalAlignmentLeft".to_string();
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
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PageBlockHorizontalAlignmentRight {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "pageBlockHorizontalAlignmentRight"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPageBlockHorizontalAlignment for PageBlockHorizontalAlignmentRight {}

impl PageBlockHorizontalAlignmentRight {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockHorizontalAlignmentRightBuilder {
        let mut inner = PageBlockHorizontalAlignmentRight::default();
        inner.td_name = "pageBlockHorizontalAlignmentRight".to_string();
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
