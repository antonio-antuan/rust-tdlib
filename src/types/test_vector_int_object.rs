use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestVectorIntObjectBuilder {
        let mut inner = TestVectorIntObject::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTestVectorIntObjectBuilder { inner }
    }

    pub fn value(&self) -> &Vec<TestInt> {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDTestVectorIntObjectBuilder {
    inner: TestVectorIntObject,
}

impl RTDTestVectorIntObjectBuilder {
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

impl AsRef<TestVectorIntObject> for RTDTestVectorIntObjectBuilder {
    fn as_ref(&self) -> &TestVectorIntObject {
        &self.inner
    }
}
