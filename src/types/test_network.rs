use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends a simple network request to the Telegram servers; for testing only. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestNetwork {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestNetwork {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestNetwork {}

impl TestNetwork {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestNetworkBuilder {
        let mut inner = TestNetwork::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testNetwork".to_string();

        RTDTestNetworkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDTestNetworkBuilder {
    inner: TestNetwork,
}

impl RTDTestNetworkBuilder {
    pub fn build(&self) -> TestNetwork {
        self.inner.clone()
    }
}

impl AsRef<TestNetwork> for TestNetwork {
    fn as_ref(&self) -> &TestNetwork {
        self
    }
}

impl AsRef<TestNetwork> for RTDTestNetworkBuilder {
    fn as_ref(&self) -> &TestNetwork {
        &self.inner
    }
}
