use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a Vertical alignment of a table cell content
pub trait TDPageBlockVerticalAlignment: Debug + RObject {}

/// Describes a Vertical alignment of a table cell content
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PageBlockVerticalAlignment {
    #[doc(hidden)]
    _Default,
    /// The content should be bottom-aligned
    #[serde(rename(
        serialize = "pageBlockVerticalAlignmentBottom",
        deserialize = "pageBlockVerticalAlignmentBottom"
    ))]
    Bottom(PageBlockVerticalAlignmentBottom),
    /// The content should be middle-aligned
    #[serde(rename(
        serialize = "pageBlockVerticalAlignmentMiddle",
        deserialize = "pageBlockVerticalAlignmentMiddle"
    ))]
    Middle(PageBlockVerticalAlignmentMiddle),
    /// The content should be top-aligned
    #[serde(rename(
        serialize = "pageBlockVerticalAlignmentTop",
        deserialize = "pageBlockVerticalAlignmentTop"
    ))]
    Top(PageBlockVerticalAlignmentTop),
}

impl Default for PageBlockVerticalAlignment {
    fn default() -> Self {
        PageBlockVerticalAlignment::_Default
    }
}

impl RObject for PageBlockVerticalAlignment {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PageBlockVerticalAlignment::Bottom(t) => t.extra(),
            PageBlockVerticalAlignment::Middle(t) => t.extra(),
            PageBlockVerticalAlignment::Top(t) => t.extra(),

            _ => None,
        }
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
        matches!(self, PageBlockVerticalAlignment::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockVerticalAlignmentBottom {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentBottom {}

impl PageBlockVerticalAlignmentBottom {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVerticalAlignmentBottomBuilder {
        let mut inner = PageBlockVerticalAlignmentBottom::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockVerticalAlignmentMiddle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentMiddle {}

impl PageBlockVerticalAlignmentMiddle {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVerticalAlignmentMiddleBuilder {
        let mut inner = PageBlockVerticalAlignmentMiddle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PageBlockVerticalAlignmentTop {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPageBlockVerticalAlignment for PageBlockVerticalAlignmentTop {}

impl PageBlockVerticalAlignmentTop {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockVerticalAlignmentTopBuilder {
        let mut inner = PageBlockVerticalAlignmentTop::default();
        inner.extra = Some(Uuid::new_v4().to_string());

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
