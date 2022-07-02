use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a sequence of bytes; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestBytes {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Bytes

    #[serde(default)]
    value: String,
}

impl RObject for TestBytes {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TestBytes {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestBytesBuilder {
        let mut inner = TestBytes::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TestBytesBuilder { inner }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[doc(hidden)]
pub struct TestBytesBuilder {
    inner: TestBytes,
}

#[deprecated]
pub type RTDTestBytesBuilder = TestBytesBuilder;

impl TestBytesBuilder {
    pub fn build(&self) -> TestBytes {
        self.inner.clone()
    }

    pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().to_string();
        self
    }
}

impl AsRef<TestBytes> for TestBytes {
    fn as_ref(&self) -> &TestBytes {
        self
    }
}

impl AsRef<TestBytes> for TestBytesBuilder {
    fn as_ref(&self) -> &TestBytes {
        &self.inner
    }
}
