use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the received vector of strings; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of strings to return

    #[serde(default)]
    x: Vec<String>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestCallVectorString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestCallVectorString {}

impl TestCallVectorString {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestCallVectorStringBuilder {
        let mut inner = TestCallVectorString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testCallVectorString".to_string();

        TestCallVectorStringBuilder { inner }
    }

    pub fn x(&self) -> &Vec<String> {
        &self.x
    }
}

#[doc(hidden)]
pub struct TestCallVectorStringBuilder {
    inner: TestCallVectorString,
}

#[deprecated]
pub type RTDTestCallVectorStringBuilder = TestCallVectorStringBuilder;

impl TestCallVectorStringBuilder {
    pub fn build(&self) -> TestCallVectorString {
        self.inner.clone()
    }

    pub fn x(&mut self, x: Vec<String>) -> &mut Self {
        self.inner.x = x;
        self
    }
}

impl AsRef<TestCallVectorString> for TestCallVectorString {
    fn as_ref(&self) -> &TestCallVectorString {
        self
    }
}

impl AsRef<TestCallVectorString> for TestCallVectorStringBuilder {
    fn as_ref(&self) -> &TestCallVectorString {
        &self.inner
    }
}
