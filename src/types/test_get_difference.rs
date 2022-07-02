use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Forces an updates.getDifference call to the Telegram servers; for testing only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestGetDifference {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestGetDifference {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestGetDifference {}

impl TestGetDifference {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TestGetDifferenceBuilder {
        let mut inner = TestGetDifference::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testGetDifference".to_string();

        TestGetDifferenceBuilder { inner }
    }
}

#[doc(hidden)]
pub struct TestGetDifferenceBuilder {
    inner: TestGetDifference,
}

#[deprecated]
pub type RTDTestGetDifferenceBuilder = TestGetDifferenceBuilder;

impl TestGetDifferenceBuilder {
    pub fn build(&self) -> TestGetDifference {
        self.inner.clone()
    }
}

impl AsRef<TestGetDifference> for TestGetDifference {
    fn as_ref(&self) -> &TestGetDifference {
        self
    }
}

impl AsRef<TestGetDifference> for TestGetDifferenceBuilder {
    fn as_ref(&self) -> &TestGetDifference {
        &self.inner
    }
}
