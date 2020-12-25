
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains some text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Text {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text
  text: String,
  
}

impl RObject for Text {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "text" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Text {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextBuilder {
    let mut inner = Text::default();
    inner.td_name = "text".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextBuilder { inner }
  }

  pub fn text(&self) -> &String { &self.text }

}

#[doc(hidden)]
pub struct RTDTextBuilder {
  inner: Text
}

impl RTDTextBuilder {
  pub fn build(&self) -> Text { self.inner.clone() }

   
  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

}

impl AsRef<Text> for Text {
  fn as_ref(&self) -> &Text { self }
}

impl AsRef<Text> for RTDTextBuilder {
  fn as_ref(&self) -> &Text { &self.inner }
}



