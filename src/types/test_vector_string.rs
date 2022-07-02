use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a vector of strings; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of strings

    #[serde(default)]
    value: Vec<String>,
}

impl RObject for TestVectorString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TestVectorString {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestVectorStringBuilder {
        let mut inner = TestVectorString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TestVectorStringBuilder { inner }
    }

    pub fn value(&self) -> &Vec<String> {
        &self.value
    }
}

#[doc(hidden)]
pub struct TestVectorStringBuilder {
    inner: TestVectorString,
}

#[deprecated]
pub type RTDTestVectorStringBuilder = TestVectorStringBuilder;

impl TestVectorStringBuilder {
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

impl AsRef<TestVectorString> for TestVectorStringBuilder {
    fn as_ref(&self) -> &TestVectorString {
        &self.inner
    }
}
