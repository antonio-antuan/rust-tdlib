use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a vector of strings; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of strings
    value: Vec<String>,
}

impl RObject for TestVectorString {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "testVectorString"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TestVectorString {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestVectorStringBuilder {
        let mut inner = TestVectorString::default();
        inner.td_name = "testVectorString".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDTestVectorStringBuilder { inner }
    }

    pub fn value(&self) -> &Vec<String> {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDTestVectorStringBuilder {
    inner: TestVectorString,
}

impl RTDTestVectorStringBuilder {
    pub fn build(&self) -> TestVectorString {
        self.inner.clone()
    }

    pub fn value(&mut self, value: Vec<String>) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<TestVectorString> for TestVectorString {
    fn as_ref(&self) -> &TestVectorString {
        self
    }
}

impl AsRef<TestVectorString> for RTDTestVectorStringBuilder {
    fn as_ref(&self) -> &TestVectorString {
        &self.inner
    }
}
