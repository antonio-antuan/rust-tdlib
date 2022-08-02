use crate::errors::Result;
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
    /// The content must be bottom-aligned
    #[serde(rename = "pageBlockVerticalAlignmentBottom")]
    Bottom(PageBlockVerticalAlignmentBottom),
    /// The content must be middle-aligned
    #[serde(rename = "pageBlockVerticalAlignmentMiddle")]
    Middle(PageBlockVerticalAlignmentMiddle),
    /// The content must be top-aligned
    #[serde(rename = "pageBlockVerticalAlignmentTop")]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
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

/// The content must be bottom-aligned
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockVerticalAlignmentBottomBuilder {
        let mut inner = PageBlockVerticalAlignmentBottom::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockVerticalAlignmentBottomBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PageBlockVerticalAlignmentBottomBuilder {
    inner: PageBlockVerticalAlignmentBottom,
}

#[deprecated]
pub type RTDPageBlockVerticalAlignmentBottomBuilder = PageBlockVerticalAlignmentBottomBuilder;

impl PageBlockVerticalAlignmentBottomBuilder {
    pub fn build(&self) -> PageBlockVerticalAlignmentBottom {
        self.inner.clone()
    }
}

impl AsRef<PageBlockVerticalAlignmentBottom> for PageBlockVerticalAlignmentBottom {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentBottom {
        self
    }
}

impl AsRef<PageBlockVerticalAlignmentBottom> for PageBlockVerticalAlignmentBottomBuilder {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentBottom {
        &self.inner
    }
}

/// The content must be middle-aligned
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockVerticalAlignmentMiddleBuilder {
        let mut inner = PageBlockVerticalAlignmentMiddle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockVerticalAlignmentMiddleBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PageBlockVerticalAlignmentMiddleBuilder {
    inner: PageBlockVerticalAlignmentMiddle,
}

#[deprecated]
pub type RTDPageBlockVerticalAlignmentMiddleBuilder = PageBlockVerticalAlignmentMiddleBuilder;

impl PageBlockVerticalAlignmentMiddleBuilder {
    pub fn build(&self) -> PageBlockVerticalAlignmentMiddle {
        self.inner.clone()
    }
}

impl AsRef<PageBlockVerticalAlignmentMiddle> for PageBlockVerticalAlignmentMiddle {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentMiddle {
        self
    }
}

impl AsRef<PageBlockVerticalAlignmentMiddle> for PageBlockVerticalAlignmentMiddleBuilder {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentMiddle {
        &self.inner
    }
}

/// The content must be top-aligned
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PageBlockVerticalAlignmentTopBuilder {
        let mut inner = PageBlockVerticalAlignmentTop::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PageBlockVerticalAlignmentTopBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PageBlockVerticalAlignmentTopBuilder {
    inner: PageBlockVerticalAlignmentTop,
}

#[deprecated]
pub type RTDPageBlockVerticalAlignmentTopBuilder = PageBlockVerticalAlignmentTopBuilder;

impl PageBlockVerticalAlignmentTopBuilder {
    pub fn build(&self) -> PageBlockVerticalAlignmentTop {
        self.inner.clone()
    }
}

impl AsRef<PageBlockVerticalAlignmentTop> for PageBlockVerticalAlignmentTop {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentTop {
        self
    }
}

impl AsRef<PageBlockVerticalAlignmentTop> for PageBlockVerticalAlignmentTopBuilder {
    fn as_ref(&self) -> &PageBlockVerticalAlignmentTop {
        &self.inner
    }
}
