
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents one member of a JSON object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonObjectMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Member's key
  key: String,
  /// Member's value
  value: JsonValue,
  
}

impl RObject for JsonObjectMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "jsonObjectMember" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl JsonObjectMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDJsonObjectMemberBuilder {
    let mut inner = JsonObjectMember::default();
    inner.td_name = "jsonObjectMember".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDJsonObjectMemberBuilder { inner }
  }

  pub fn key(&self) -> &String { &self.key }

  pub fn value(&self) -> &JsonValue { &self.value }

}

#[doc(hidden)]
pub struct RTDJsonObjectMemberBuilder {
  inner: JsonObjectMember
}

impl RTDJsonObjectMemberBuilder {
  pub fn build(&self) -> JsonObjectMember { self.inner.clone() }

   
  pub fn key<T: AsRef<str>>(&mut self, key: T) -> &mut Self {
    self.inner.key = key.as_ref().to_string();
    self
  }

   
  pub fn value<T: AsRef<JsonValue>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().clone();
    self
  }

}

impl AsRef<JsonObjectMember> for JsonObjectMember {
  fn as_ref(&self) -> &JsonObjectMember { self }
}

impl AsRef<JsonObjectMember> for RTDJsonObjectMemberBuilder {
  fn as_ref(&self) -> &JsonObjectMember { &self.inner }
}



