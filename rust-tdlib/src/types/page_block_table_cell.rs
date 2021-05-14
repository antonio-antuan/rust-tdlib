use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a cell of a table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockTableCell {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Cell text; may be null. If the text is null, then the cell should be invisible
    text: Option<RichText>,
    /// True, if it is a header cell
    is_header: bool,
    /// The number of columns the cell should span
    colspan: i32,
    /// The number of rows the cell should span
    rowspan: i32,
    /// Horizontal cell content alignment

    #[serde(skip_serializing_if = "PageBlockHorizontalAlignment::_is_default")]
    align: PageBlockHorizontalAlignment,
    /// Vertical cell content alignment

    #[serde(skip_serializing_if = "PageBlockVerticalAlignment::_is_default")]
    valign: PageBlockVerticalAlignment,
}

impl RObject for PageBlockTableCell {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PageBlockTableCell {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPageBlockTableCellBuilder {
        let mut inner = PageBlockTableCell::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPageBlockTableCellBuilder { inner }
    }

    pub fn text(&self) -> &Option<RichText> {
        &self.text
    }

    pub fn is_header(&self) -> bool {
        self.is_header
    }

    pub fn colspan(&self) -> i32 {
        self.colspan
    }

    pub fn rowspan(&self) -> i32 {
        self.rowspan
    }

    pub fn align(&self) -> &PageBlockHorizontalAlignment {
        &self.align
    }

    pub fn valign(&self) -> &PageBlockVerticalAlignment {
        &self.valign
    }
}

#[doc(hidden)]
pub struct RTDPageBlockTableCellBuilder {
    inner: PageBlockTableCell,
}

impl RTDPageBlockTableCellBuilder {
    pub fn build(&self) -> PageBlockTableCell {
        self.inner.clone()
    }

    pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = Some(text.as_ref().clone());
        self
    }

    pub fn is_header(&mut self, is_header: bool) -> &mut Self {
        self.inner.is_header = is_header;
        self
    }

    pub fn colspan(&mut self, colspan: i32) -> &mut Self {
        self.inner.colspan = colspan;
        self
    }

    pub fn rowspan(&mut self, rowspan: i32) -> &mut Self {
        self.inner.rowspan = rowspan;
        self
    }

    pub fn align<T: AsRef<PageBlockHorizontalAlignment>>(&mut self, align: T) -> &mut Self {
        self.inner.align = align.as_ref().clone();
        self
    }

    pub fn valign<T: AsRef<PageBlockVerticalAlignment>>(&mut self, valign: T) -> &mut Self {
        self.inner.valign = valign.as_ref().clone();
        self
    }
}

impl AsRef<PageBlockTableCell> for PageBlockTableCell {
    fn as_ref(&self) -> &PageBlockTableCell {
        self
    }
}

impl AsRef<PageBlockTableCell> for RTDPageBlockTableCellBuilder {
    fn as_ref(&self) -> &PageBlockTableCell {
        &self.inner
    }
}
