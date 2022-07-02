use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a vector of objects that hold a number; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorIntObject {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of objects

    #[serde(default)]
    value: Vec<TestInt>,
}

impl RObject for TestVectorIntObject {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TestVectorIntObject {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestVectorIntObjectBuilder {
        let mut inner = TestVectorIntObject::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TestVectorIntObjectBuilder { inner }
    }

    pub fn value(&self) -> &Vec<TestInt> {
        &self.value
    }
}

#[doc(hidden)]
pub struct TestVectorIntObjectBuilder {
    inner: TestVectorIntObject,
}

#[deprecated]
pub type RTDTestVectorIntObjectBuilder = TestVectorIntObjectBuilder;

impl TestVectorIntObjectBuilder {
    pub fn build(&self) -> TestVectorIntObject {
        self.inner.clone()
    }

    pub fn value(&mut self, value: Vec<TestInt>) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<TestVectorIntObject> for TestVectorIntObject {
    fn as_ref(&self) -> &TestVectorIntObject {
        self
    }
}

impl AsRef<TestVectorIntObject> for TestVectorIntObjectBuilder {
    fn as_ref(&self) -> &TestVectorIntObject {
        &self.inner
    }
}
