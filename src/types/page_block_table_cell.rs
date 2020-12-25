
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a cell of a table
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockTableCell {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Cell text; may be null. If the text is null, then the cell should be invisible
  text: Option<RichText>,
  /// True, if it is a header cell
  is_header: bool,
  /// The number of columns the cell should span
  colspan: i64,
  /// The number of rows the cell should span
  rowspan: i64,
  /// Horizontal cell content alignment
  align: PageBlockHorizontalAlignment,
  /// Vertical cell content alignment
  valign: PageBlockVerticalAlignment,
  
}

impl RObject for PageBlockTableCell {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockTableCell" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PageBlockTableCell {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockTableCellBuilder {
    let mut inner = PageBlockTableCell::default();
    inner.td_name = "pageBlockTableCell".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockTableCellBuilder { inner }
  }

  pub fn text(&self) -> &Option<RichText> { &self.text }

  pub fn is_header(&self) -> bool { self.is_header }

  pub fn colspan(&self) -> i64 { self.colspan }

  pub fn rowspan(&self) -> i64 { self.rowspan }

  pub fn align(&self) -> &PageBlockHorizontalAlignment { &self.align }

  pub fn valign(&self) -> &PageBlockVerticalAlignment { &self.valign }

}

#[doc(hidden)]
pub struct RTDPageBlockTableCellBuilder {
  inner: PageBlockTableCell
}

impl RTDPageBlockTableCellBuilder {
  pub fn build(&self) -> PageBlockTableCell { self.inner.clone() }

   
  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = Some(text.as_ref().clone());
    self
  }

   
  pub fn is_header(&mut self, is_header: bool) -> &mut Self {
    self.inner.is_header = is_header;
    self
  }

   
  pub fn colspan(&mut self, colspan: i64) -> &mut Self {
    self.inner.colspan = colspan;
    self
  }

   
  pub fn rowspan(&mut self, rowspan: i64) -> &mut Self {
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
  fn as_ref(&self) -> &PageBlockTableCell { self }
}

impl AsRef<PageBlockTableCell> for RTDPageBlockTableCellBuilder {
  fn as_ref(&self) -> &PageBlockTableCell { &self.inner }
}



