use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Does nothing and ensures that the Update object is used; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestUseUpdate {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestUseUpdate {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDUpdate for TestUseUpdate {}

impl RFunction for TestUseUpdate {}

impl TestUseUpdate {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestUseUpdateBuilder {
        let mut inner = TestUseUpdate::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testUseUpdate".to_string();

        TestUseUpdateBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TestUseUpdateBuilder {
    inner: TestUseUpdate,
}

#[deprecated]
pub type RTDTestUseUpdateBuilder = TestUseUpdateBuilder;

impl TestUseUpdateBuilder {
    pub fn build(&self) -> TestUseUpdate {
        self.inner.clone()
    }
}

impl AsRef<TestUseUpdate> for TestUseUpdate {
    fn as_ref(&self) -> &TestUseUpdate {
        self
    }
}

impl AsRef<TestUseUpdate> for TestUseUpdateBuilder {
    fn as_ref(&self) -> &TestUseUpdate {
        &self.inner
    }
}
