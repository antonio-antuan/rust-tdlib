
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes an instant view page for a web page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WebPageInstantView {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Content of the web page
  page_blocks: Vec<PageBlock>,
  /// Version of the instant view, currently can be 1 or 2
  version: i64,
  /// Instant view URL; may be different from WebPage.url and must be used for the correct anchors handling
  url: String,
  /// True, if the instant view must be shown from right to left
  is_rtl: bool,
  /// True, if the instant view contains the full page. A network request might be needed to get the full web page instant view
  is_full: bool,
  
}

impl RObject for WebPageInstantView {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "webPageInstantView" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl WebPageInstantView {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDWebPageInstantViewBuilder {
    let mut inner = WebPageInstantView::default();
    inner.td_name = "webPageInstantView".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDWebPageInstantViewBuilder { inner }
  }

  pub fn page_blocks(&self) -> &Vec<PageBlock> { &self.page_blocks }

  pub fn version(&self) -> i64 { self.version }

  pub fn url(&self) -> &String { &self.url }

  pub fn is_rtl(&self) -> bool { self.is_rtl }

  pub fn is_full(&self) -> bool { self.is_full }

}

#[doc(hidden)]
pub struct RTDWebPageInstantViewBuilder {
  inner: WebPageInstantView
}

impl RTDWebPageInstantViewBuilder {
  pub fn build(&self) -> WebPageInstantView { self.inner.clone() }

   
  pub fn page_blocks(&mut self, page_blocks: Vec<PageBlock>) -> &mut Self {
    self.inner.page_blocks = page_blocks;
    self
  }

   
  pub fn version(&mut self, version: i64) -> &mut Self {
    self.inner.version = version;
    self
  }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

   
  pub fn is_rtl(&mut self, is_rtl: bool) -> &mut Self {
    self.inner.is_rtl = is_rtl;
    self
  }

   
  pub fn is_full(&mut self, is_full: bool) -> &mut Self {
    self.inner.is_full = is_full;
    self
  }

}

impl AsRef<WebPageInstantView> for WebPageInstantView {
  fn as_ref(&self) -> &WebPageInstantView { self }
}

impl AsRef<WebPageInstantView> for RTDWebPageInstantViewBuilder {
  fn as_ref(&self) -> &WebPageInstantView { &self.inner }
}



