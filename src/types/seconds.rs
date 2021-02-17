use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a value representing a number of seconds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Seconds {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of seconds
    seconds: f32,
}

impl RObject for Seconds {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Seconds {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSecondsBuilder {
        let mut inner = Seconds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDSecondsBuilder { inner }
    }

    pub fn seconds(&self) -> f32 {
        self.seconds
    }
}

#[doc(hidden)]
pub struct RTDSecondsBuilder {
    inner: Seconds,
}

impl RTDSecondsBuilder {
    pub fn build(&self) -> Seconds {
        self.inner.clone()
    }

    pub fn seconds(&mut self, seconds: f32) -> &mut Self {
        self.inner.seconds = seconds;
        self
    }
}

impl AsRef<Seconds> for Seconds {
    fn as_ref(&self) -> &Seconds {
        self
    }
}

impl AsRef<Seconds> for RTDSecondsBuilder {
    fn as_ref(&self) -> &Seconds {
        &self.inner
    }
}
