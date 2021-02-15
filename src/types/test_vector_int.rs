use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a vector of numbers; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorInt {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of numbers
    value: Vec<i32>,
}

impl RObject for TestVectorInt {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TestVectorInt {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestVectorIntBuilder {
        let mut inner = TestVectorInt::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTestVectorIntBuilder { inner }
    }

    pub fn value(&self) -> &Vec<i32> {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDTestVectorIntBuilder {
    inner: TestVectorInt,
}

impl RTDTestVectorIntBuilder {
    pub fn build(&self) -> TestVectorInt {
        self.inner.clone()
    }

    pub fn value(&mut self, value: Vec<i32>) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<TestVectorInt> for TestVectorInt {
    fn as_ref(&self) -> &TestVectorInt {
        self
    }
}

impl AsRef<TestVectorInt> for RTDTestVectorIntBuilder {
    fn as_ref(&self) -> &TestVectorInt {
        &self.inner
    }
}
