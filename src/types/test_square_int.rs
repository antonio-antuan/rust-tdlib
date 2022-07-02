use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the squared received number; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestSquareInt {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number to square

    #[serde(default)]
    x: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestSquareInt {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestSquareInt {}

impl TestSquareInt {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestSquareIntBuilder {
        let mut inner = TestSquareInt::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testSquareInt".to_string();

        TestSquareIntBuilder { inner }
    }

    pub fn x(&self) -> i32 {
        self.x
    }
}

#[doc(hidden)]
pub struct TestSquareIntBuilder {
    inner: TestSquareInt,
}

#[deprecated]
pub type RTDTestSquareIntBuilder = TestSquareIntBuilder;

impl TestSquareIntBuilder {
    pub fn build(&self) -> TestSquareInt {
        self.inner.clone()
    }

    pub fn x(&mut self, x: i32) -> &mut Self {
        self.inner.x = x;
        self
    }
}

impl AsRef<TestSquareInt> for TestSquareInt {
    fn as_ref(&self) -> &TestSquareInt {
        self
    }
}

impl AsRef<TestSquareInt> for TestSquareIntBuilder {
    fn as_ref(&self) -> &TestSquareInt {
        &self.inner
    }
}
