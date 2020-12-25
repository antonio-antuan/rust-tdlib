
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a part of the text that needs to be formatted in some unusual way
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntity {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Offset of the entity in UTF-16 code units
  offset: i64,
  /// Length of the entity, in UTF-16 code units
  length: i64,
  /// Type of the entity
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: TextEntityType,
  
}

impl RObject for TextEntity {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntity" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TextEntity {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityBuilder {
    let mut inner = TextEntity::default();
    inner.td_name = "textEntity".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityBuilder { inner }
  }

  pub fn offset(&self) -> i64 { self.offset }

  pub fn length(&self) -> i64 { self.length }

  pub fn type_(&self) -> &TextEntityType { &self.type_ }

}

#[doc(hidden)]
pub struct RTDTextEntityBuilder {
  inner: TextEntity
}

impl RTDTextEntityBuilder {
  pub fn build(&self) -> TextEntity { self.inner.clone() }

   
  pub fn offset(&mut self, offset: i64) -> &mut Self {
    self.inner.offset = offset;
    self
  }

   
  pub fn length(&mut self, length: i64) -> &mut Self {
    self.inner.length = length;
    self
  }

   
  pub fn type_<T: AsRef<TextEntityType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

}

impl AsRef<TextEntity> for TextEntity {
  fn as_ref(&self) -> &TextEntity { self }
}

impl AsRef<TextEntity> for RTDTextEntityBuilder {
  fn as_ref(&self) -> &TextEntity { &self.inner }
}



