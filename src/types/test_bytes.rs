
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// A simple object containing a sequence of bytes; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestBytes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Bytes
  value: String,
  
}

impl RObject for TestBytes {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "testBytes" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TestBytes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTestBytesBuilder {
    let mut inner = TestBytes::default();
    inner.td_name = "testBytes".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTestBytesBuilder { inner }
  }

  pub fn value(&self) -> &String { &self.value }

}

#[doc(hidden)]
pub struct RTDTestBytesBuilder {
  inner: TestBytes
}

impl RTDTestBytesBuilder {
  pub fn build(&self) -> TestBytes { self.inner.clone() }

   
  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }

}

impl AsRef<TestBytes> for TestBytes {
  fn as_ref(&self) -> &TestBytes { self }
}

impl AsRef<TestBytes> for RTDTestBytesBuilder {
  fn as_ref(&self) -> &TestBytes { &self.inner }
}



