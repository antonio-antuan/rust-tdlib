use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the received vector of numbers; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallVectorInt {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Vector of numbers to return
    x: Vec<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestCallVectorInt {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestCallVectorInt {}

impl TestCallVectorInt {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestCallVectorIntBuilder {
        let mut inner = TestCallVectorInt::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testCallVectorInt".to_string();

        RTDTestCallVectorIntBuilder { inner }
    }

    pub fn x(&self) -> &Vec<i32> {
        &self.x
    }
}

#[doc(hidden)]
pub struct RTDTestCallVectorIntBuilder {
    inner: TestCallVectorInt,
}

impl RTDTestCallVectorIntBuilder {
    pub fn build(&self) -> TestCallVectorInt {
        self.inner.clone()
    }

    pub fn x(&mut self, x: Vec<i32>) -> &mut Self {
        self.inner.x = x;
        self
    }
}

impl AsRef<TestCallVectorInt> for TestCallVectorInt {
    fn as_ref(&self) -> &TestCallVectorInt {
        self
    }
}

impl AsRef<TestCallVectorInt> for RTDTestCallVectorIntBuilder {
    fn as_ref(&self) -> &TestCallVectorInt {
        &self.inner
    }
}
