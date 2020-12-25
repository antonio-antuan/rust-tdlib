
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a caption of an instant view web page block, consisting of a text and a trailing credit
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PageBlockCaption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Content of the caption
  text: RichText,
  /// Block credit (like HTML tag <cite>)
  credit: RichText,
  
}

impl RObject for PageBlockCaption {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockCaption" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl PageBlockCaption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPageBlockCaptionBuilder {
    let mut inner = PageBlockCaption::default();
    inner.td_name = "pageBlockCaption".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDPageBlockCaptionBuilder { inner }
  }

  pub fn text(&self) -> &RichText { &self.text }

  pub fn credit(&self) -> &RichText { &self.credit }

}

#[doc(hidden)]
pub struct RTDPageBlockCaptionBuilder {
  inner: PageBlockCaption
}

impl RTDPageBlockCaptionBuilder {
  pub fn build(&self) -> PageBlockCaption { self.inner.clone() }

   
  pub fn text<T: AsRef<RichText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

   
  pub fn credit<T: AsRef<RichText>>(&mut self, credit: T) -> &mut Self {
    self.inner.credit = credit.as_ref().clone();
    self
  }

}

impl AsRef<PageBlockCaption> for PageBlockCaption {
  fn as_ref(&self) -> &PageBlockCaption { self }
}

impl AsRef<PageBlockCaption> for RTDPageBlockCaptionBuilder {
  fn as_ref(&self) -> &PageBlockCaption { &self.inner }
}



