use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a string; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// String
    value: String,
}

impl RObject for TestString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TestString {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestStringBuilder {
        let mut inner = TestString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTestStringBuilder { inner }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDTestStringBuilder {
    inner: TestString,
}

impl RTDTestStringBuilder {
    pub fn build(&self) -> TestString {
        self.inner.clone()
    }

    pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().to_string();
        self
    }
}

impl AsRef<TestString> for TestString {
    fn as_ref(&self) -> &TestString {
        self
    }
}

impl AsRef<TestString> for RTDTestStringBuilder {
    fn as_ref(&self) -> &TestString {
        &self.inner
    }
}
