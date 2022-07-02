use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A simple object containing a vector of objects that hold a string; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestVectorStringObject {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of objects

    #[serde(default)]
    value: Vec<TestString>,
}

impl RObject for TestVectorStringObject {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TestVectorStringObject {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestVectorStringObjectBuilder {
        let mut inner = TestVectorStringObject::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TestVectorStringObjectBuilder { inner }
    }

    pub fn value(&self) -> &Vec<TestString> {
        &self.value
    }
}

#[doc(hidden)]
pub struct TestVectorStringObjectBuilder {
    inner: TestVectorStringObject,
}

#[deprecated]
pub type RTDTestVectorStringObjectBuilder = TestVectorStringObjectBuilder;

impl TestVectorStringObjectBuilder {
    pub fn build(&self) -> TestVectorStringObject {
        self.inner.clone()
    }

    pub fn value(&mut self, value: Vec<TestString>) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<TestVectorStringObject> for TestVectorStringObject {
    fn as_ref(&self) -> &TestVectorStringObject {
        self
    }
}

impl AsRef<TestVectorStringObject> for TestVectorStringObjectBuilder {
    fn as_ref(&self) -> &TestVectorStringObject {
        &self.inner
    }
}
