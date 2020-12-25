
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A simple object containing a vector of objects that hold a string; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorStringObject {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Vector of objects
  value: Vec<TestString>,
  
}

impl RObject for TestVectorStringObject {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorStringObject" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestVectorStringObject {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestVectorStringObjectBuilder {
    let mut inner = TestVectorStringObject::default();
    inner.td_name = "testVectorStringObject".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestVectorStringObjectBuilder { inner }
  }

  pub fn value(&self) -> &Vec<TestString> { &self.value }

}

#[doc(hidden)]
pub struct RTDTestVectorStringObjectBuilder {
  inner: TestVectorStringObject
}

impl RTDTestVectorStringObjectBuilder {
  pub fn build(&self) -> TestVectorStringObject { self.inner.clone() }

   
  pub fn value(&mut self, value: Vec<TestString>) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<TestVectorStringObject> for TestVectorStringObject {
  fn as_ref(&self) -> &TestVectorStringObject { self }
}

impl AsRef<TestVectorStringObject> for RTDTestVectorStringObjectBuilder {
  fn as_ref(&self) -> &TestVectorStringObject { &self.inner }
}



