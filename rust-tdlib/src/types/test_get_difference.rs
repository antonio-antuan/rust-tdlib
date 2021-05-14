use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestGetDifferenceBuilder {
        let mut inner = TestGetDifference::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testGetDifference".to_string();

        RTDTestGetDifferenceBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTestGetDifferenceBuilder {
    inner: TestGetDifference,
}

impl RTDTestGetDifferenceBuilder {
    pub fn build(&self) -> TestGetDifference {
        self.inner.clone()
    }
}

impl AsRef<TestGetDifference> for TestGetDifference {
    fn as_ref(&self) -> &TestGetDifference {
        self
    }
}

impl AsRef<TestGetDifference> for RTDTestGetDifferenceBuilder {
    fn as_ref(&self) -> &TestGetDifference {
        &self.inner
    }
}
