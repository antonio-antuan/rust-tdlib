
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A simple object containing a vector of numbers; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorInt {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Vector of numbers
  value: Vec<i64>,
  
}

impl RObject for TestVectorInt {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testVectorInt" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestVectorInt {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestVectorIntBuilder {
    let mut inner = TestVectorInt::default();
    inner.td_name = "testVectorInt".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestVectorIntBuilder { inner }
  }

  pub fn value(&self) -> &Vec<i64> { &self.value }

}

#[doc(hidden)]
pub struct RTDTestVectorIntBuilder {
  inner: TestVectorInt
}

impl RTDTestVectorIntBuilder {
  pub fn build(&self) -> TestVectorInt { self.inner.clone() }

   
  pub fn value(&mut self, value: Vec<i64>) -> &mut Self {
    self.inner.value = value;
    self
  }

}

impl AsRef<TestVectorInt> for TestVectorInt {
  fn as_ref(&self) -> &TestVectorInt { self }
}

impl AsRef<TestVectorInt> for RTDTestVectorIntBuilder {
  fn as_ref(&self) -> &TestVectorInt { &self.inner }
}



