use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a number; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestInt {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number

    #[serde(default)]
    value: i32,
}

impl RObject for TestInt {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TestInt {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestIntBuilder {
        let mut inner = TestInt::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TestIntBuilder { inner }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[doc(hidden)]
pub struct TestIntBuilder {
    inner: TestInt,
}

#[deprecated]
pub type RTDTestIntBuilder = TestIntBuilder;

impl TestIntBuilder {
    pub fn build(&self) -> TestInt {
        self.inner.clone()
    }

    pub fn value(&mut self, value: i32) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<TestInt> for TestInt {
    fn as_ref(&self) -> &TestInt {
        self
    }
}

impl AsRef<TestInt> for TestIntBuilder {
    fn as_ref(&self) -> &TestInt {
        &self.inner
    }
}
