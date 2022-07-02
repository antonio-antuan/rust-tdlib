use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the received vector of objects containing a number; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorIntObject {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of objects to return

    #[serde(default)]
    x: Vec<TestInt>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestCallVectorIntObject {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestCallVectorIntObject {}

impl TestCallVectorIntObject {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestCallVectorIntObjectBuilder {
        let mut inner = TestCallVectorIntObject::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testCallVectorIntObject".to_string();

        TestCallVectorIntObjectBuilder { inner }
    }

    pub fn x(&self) -> &Vec<TestInt> {
        &self.x
    }
}

#[doc(hidden)]
pub struct TestCallVectorIntObjectBuilder {
    inner: TestCallVectorIntObject,
}

#[deprecated]
pub type RTDTestCallVectorIntObjectBuilder = TestCallVectorIntObjectBuilder;

impl TestCallVectorIntObjectBuilder {
    pub fn build(&self) -> TestCallVectorIntObject {
        self.inner.clone()
    }

    pub fn x(&mut self, x: Vec<TestInt>) -> &mut Self {
        self.inner.x = x;
        self
    }
}

impl AsRef<TestCallVectorIntObject> for TestCallVectorIntObject {
    fn as_ref(&self) -> &TestCallVectorIntObject {
        self
    }
}

impl AsRef<TestCallVectorIntObject> for TestCallVectorIntObjectBuilder {
    fn as_ref(&self) -> &TestCallVectorIntObject {
        &self.inner
    }
}
