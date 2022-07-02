use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the received string; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// String to return

    #[serde(default)]
    x: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestCallString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestCallString {}

impl TestCallString {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestCallStringBuilder {
        let mut inner = TestCallString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testCallString".to_string();

        TestCallStringBuilder { inner }
    }

    pub fn x(&self) -> &String {
        &self.x
    }
}

#[doc(hidden)]
pub struct TestCallStringBuilder {
    inner: TestCallString,
}

#[deprecated]
pub type RTDTestCallStringBuilder = TestCallStringBuilder;

impl TestCallStringBuilder {
    pub fn build(&self) -> TestCallString {
        self.inner.clone()
    }

    pub fn x<T: AsRef<str>>(&mut self, x: T) -> &mut Self {
        self.inner.x = x.as_ref().to_string();
        self
    }
}

impl AsRef<TestCallString> for TestCallString {
    fn as_ref(&self) -> &TestCallString {
        self
    }
}

impl AsRef<TestCallString> for TestCallStringBuilder {
    fn as_ref(&self) -> &TestCallString {
        &self.inner
    }
}
