use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the received bytes; for testing only. This is an offline method. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestCallBytes {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Bytes to return
    x: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for TestCallBytes {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for TestCallBytes {}

impl TestCallBytes {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTestCallBytesBuilder {
        let mut inner = TestCallBytes::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "testCallBytes".to_string();

        RTDTestCallBytesBuilder { inner }
    }

    pub fn x(&self) -> &String {
        &self.x
    }
}

#[doc(hidden)]
pub struct RTDTestCallBytesBuilder {
    inner: TestCallBytes,
}

impl RTDTestCallBytesBuilder {
    pub fn build(&self) -> TestCallBytes {
        self.inner.clone()
    }

    pub fn x<T: AsRef<str>>(&mut self, x: T) -> &mut Self {
        self.inner.x = x.as_ref().to_string();
        self
    }
}

impl AsRef<TestCallBytes> for TestCallBytes {
    fn as_ref(&self) -> &TestCallBytes {
        self
    }
}

impl AsRef<TestCallBytes> for RTDTestCallBytesBuilder {
    fn as_ref(&self) -> &TestCallBytes {
        &self.inner
    }
}
