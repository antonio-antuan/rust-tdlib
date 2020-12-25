
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a chat background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Background {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique background identifier
  id: isize,
  /// True, if this is one of default backgrounds
  is_default: bool,
  /// True, if the background is dark and is recommended to be used with dark theme
  is_dark: bool,
  /// Unique background name
  name: String,
  /// Document with the background; may be null. Null only for filled backgrounds
  document: Option<Document>,
  /// Type of the background
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: BackgroundType,
  
}

impl RObject for Background {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "background" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Background {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundBuilder {
    let mut inner = Background::default();
    inner.td_name = "background".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBackgroundBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn is_default(&self) -> bool { self.is_default }

  pub fn is_dark(&self) -> bool { self.is_dark }

  pub fn name(&self) -> &String { &self.name }

  pub fn document(&self) -> &Option<Document> { &self.document }

  pub fn type_(&self) -> &BackgroundType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDBackgroundBuilder {
  inner: Background
}

impl RTDBackgroundBuilder {
  pub fn build(&self) -> Background { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn is_default(&mut self, is_default: bool) -> &mut Self {
    self.inner.is_default = is_default;
    self
  }

   
  pub fn is_dark(&mut self, is_dark: bool) -> &mut Self {
    self.inner.is_dark = is_dark;
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn document<T: AsRef<Document>>(&mut self, document: T) -> &mut Self {
    self.inner.document = Some(document.as_ref().clone());
    self
  }

   
  pub fn type_<T: AsRef<BackgroundType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<Background> for Background {
  fn as_ref(&self) -> &Background { self }
}

impl AsRef<Background> for RTDBackgroundBuilder {
  fn as_ref(&self) -> &Background { &self.inner }
}



