use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Does nothing; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallEmpty {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestCallEmpty {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestCallEmpty {}

impl TestCallEmpty {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestCallEmptyBuilder {
        let mut inner = TestCallEmpty::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testCallEmpty".to_string();

        TestCallEmptyBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TestCallEmptyBuilder {
    inner: TestCallEmpty,
}

#[deprecated]
pub type RTDTestCallEmptyBuilder = TestCallEmptyBuilder;

impl TestCallEmptyBuilder {
    pub fn build(&self) -> TestCallEmpty {
        self.inner.clone()
    }
}

impl AsRef<TestCallEmpty> for TestCallEmpty {
    fn as_ref(&self) -> &TestCallEmpty {
        self
    }
}

impl AsRef<TestCallEmpty> for TestCallEmptyBuilder {
    fn as_ref(&self) -> &TestCallEmpty {
        &self.inner
    }
}
