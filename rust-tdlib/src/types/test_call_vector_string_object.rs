use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the received vector of objects containing a string; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorStringObject {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of objects to return
    x: Vec<TestString>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestCallVectorStringObject {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestCallVectorStringObject {}

impl TestCallVectorStringObject {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestCallVectorStringObjectBuilder {
        let mut inner = TestCallVectorStringObject::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testCallVectorStringObject".to_string();

        RTDTestCallVectorStringObjectBuilder { inner }
    }

    pub fn x(&self) -> &Vec<TestString> {
        &self.x
    }
}

#[doc(hidden)]
pub struct RTDTestCallVectorStringObjectBuilder {
    inner: TestCallVectorStringObject,
}

impl RTDTestCallVectorStringObjectBuilder {
    pub fn build(&self) -> TestCallVectorStringObject {
        self.inner.clone()
    }

    pub fn x(&mut self, x: Vec<TestString>) -> &mut Self {
        self.inner.x = x;
        self
    }
}

impl AsRef<TestCallVectorStringObject> for TestCallVectorStringObject {
    fn as_ref(&self) -> &TestCallVectorStringObject {
        self
    }
}

impl AsRef<TestCallVectorStringObject> for RTDTestCallVectorStringObjectBuilder {
    fn as_ref(&self) -> &TestCallVectorStringObject {
        &self.inner
    }
}
