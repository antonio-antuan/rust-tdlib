use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a Vertical alignment of a table cell content
pub trait TDPageBlockVerticalAlignment: Debug + RObject {}

/// Describes a Vertical alignment of a table cell content
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PageBlockVerticalAlignment {
    #[doc(hidden)]
    _Default(()),
    /// The content should be bottom-aligned
    Bottom(PageBlockVerticalAlignmentBottom),
    /// The content should be middle-aligned
    Middle(PageBlockVerticalAlignmentMiddle),
    /// The content should be top-aligned
    Top(PageBlockVerticalAlignmentTop),
}

impl Default for PageBlockVerticalAlignment {
    fn default() -> Self {
        PageBlockVerticalAlignment::_Default(())
    }
}

impl<'de> Deserialize<'de> for PageBlockVerticalAlignment {
    fn deserialize<D>(deserializer: D) -> Result<PageBlockVerticalAlignment, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          PageBlockVerticalAlignment,
          (pageBlockVerticalAlignmentBottom, Bottom);
          (pageBlockVerticalAlignmentMiddle, Middle);
          (pageBlockVerticalAlignmentTop, Top);

        )(deserializer)
    }
}

impl RObject for PageBlockVerticalAlignment {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            PageBlockVerticalAlignment::Bottom(t) => t.td_name(),
            PageBlockVerticalAlignment::Middle(t) => t.td_name(),
            PageBlockVerticalAlignment::Top(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            PageBlockVerticalAlignment::Bottom(t) => t.extra(),
            PageBlockVerticalAlignment::Middle(t) => t.extra(),
            PageBlockVerticalAlignment::Top(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PageBlockVerticalAlignment::Bottom(t) => t.client_id(),
            PageBlockVerticalAlignment::Middle(t) => t.client_id(),
            PageBlockVerticalAlignment::Top(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PageBlockVerticalAlignment {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PageBlockVerticalAlignment::_Default(_))
    }
}

impl AsRef<PageBlockVerticalAlignment> for PageBlockVerticalAlignment {
    fn as_ref(&self) -> &PageBlockVerticalAlignment {
        self
    }
}

/// The content should be bottom-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentBottom {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockVerticalAlignmentBottom {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "pageBlockVerticalAlignmentBottom"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentBottom {}

impl PageBlockVerticalAlignmentBottom {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVerticalAlignmentBottomBuilder {
        let mut inner = PageBlockVerticalAlignmentBottom::default();
        inner.td_name = "pageBlockVerticalAlignmentBottom".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDPageBlockVerticalAlignmentBottomBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPageBlockVerticalAlignmentBottomBuilder {
    inner: PageBlockVerticalAlignmentBottom,
}

impl RTDPageBlockVerticalAlignmentBottomBuilder {
    pub fn build(&self) -> PageBlockVerticalAlignmentBottom {
        self.inner.clone()
    }
}

impl AsRef<PageBlockVerticalAlignmentBottom> for PageBlockVerticalAlignmentBottom {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentBottom {
        self
    }
}

impl AsRef<PageBlockVerticalAlignmentBottom> for RTDPageBlockVerticalAlignmentBottomBuilder {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentBottom {
        &self.inner
    }
}

/// The content should be middle-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentMiddle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockVerticalAlignmentMiddle {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "pageBlockVerticalAlignmentMiddle"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentMiddle {}

impl PageBlockVerticalAlignmentMiddle {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVerticalAlignmentMiddleBuilder {
        let mut inner = PageBlockVerticalAlignmentMiddle::default();
        inner.td_name = "pageBlockVerticalAlignmentMiddle".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDPageBlockVerticalAlignmentMiddleBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPageBlockVerticalAlignmentMiddleBuilder {
    inner: PageBlockVerticalAlignmentMiddle,
}

impl RTDPageBlockVerticalAlignmentMiddleBuilder {
    pub fn build(&self) -> PageBlockVerticalAlignmentMiddle {
        self.inner.clone()
    }
}

impl AsRef<PageBlockVerticalAlignmentMiddle> for PageBlockVerticalAlignmentMiddle {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentMiddle {
        self
    }
}

impl AsRef<PageBlockVerticalAlignmentMiddle> for RTDPageBlockVerticalAlignmentMiddleBuilder {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentMiddle {
        &self.inner
    }
}

/// The content should be top-aligned
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockVerticalAlignmentTop {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockVerticalAlignmentTop {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "pageBlockVerticalAlignmentTop"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentTop {}

impl PageBlockVerticalAlignmentTop {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVerticalAlignmentTopBuilder {
        let mut inner = PageBlockVerticalAlignmentTop::default();
        inner.td_name = "pageBlockVerticalAlignmentTop".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDPageBlockVerticalAlignmentTopBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPageBlockVerticalAlignmentTopBuilder {
    inner: PageBlockVerticalAlignmentTop,
}

impl RTDPageBlockVerticalAlignmentTopBuilder {
    pub fn build(&self) -> PageBlockVerticalAlignmentTop {
        self.inner.clone()
    }
}

impl AsRef<PageBlockVerticalAlignmentTop> for PageBlockVerticalAlignmentTop {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentTop {
        self
    }
}

impl AsRef<PageBlockVerticalAlignmentTop> for RTDPageBlockVerticalAlignmentTopBuilder {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentTop {
        &self.inner
    }
}
