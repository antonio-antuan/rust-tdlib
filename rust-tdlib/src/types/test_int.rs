use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestIntBuilder {
        let mut inner = TestInt::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTestIntBuilder { inner }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

#[doc(hidden)]
pub struct RTDTestIntBuilder {
    inner: TestInt,
}

impl RTDTestIntBuilder {
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

impl AsRef<TestInt> for RTDTestIntBuilder {
    fn as_ref(&self) -> &TestInt {
        &self.inner
    }
}
