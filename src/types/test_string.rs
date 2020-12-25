
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A simple object containing a string; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// String
  value: String,
  
}

impl RObject for TestString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testString" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestString {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestStringBuilder {
    let mut inner = TestString::default();
    inner.td_name = "testString".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestStringBuilder { inner }
  }

  pub fn value(&self) -> &String { &self.value }

}

#[doc(hidden)]
pub struct RTDTestStringBuilder {
  inner: TestString
}

impl RTDTestStringBuilder {
  pub fn build(&self) -> TestString { self.inner.clone() }

   
  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }

}

impl AsRef<TestString> for TestString {
  fn as_ref(&self) -> &TestString { self }
}

impl AsRef<TestString> for RTDTestStringBuilder {
  fn as_ref(&self) -> &TestString { &self.inner }
}



