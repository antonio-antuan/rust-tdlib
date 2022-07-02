use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Succeeds after a specified amount of time has passed. Can be called before initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetAlarm {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Number of seconds before the function returns

    #[serde(default)]
    seconds: f32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetAlarm {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetAlarm {}

impl SetAlarm {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetAlarmBuilder {
        let mut inner = SetAlarm::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setAlarm".to_string();

        SetAlarmBuilder { inner }
    }

    pub fn seconds(&self) -> f32 {
        self.seconds
    }
}

#[doc(hidden)]
pub struct SetAlarmBuilder {
    inner: SetAlarm,
}

#[deprecated]
pub type RTDSetAlarmBuilder = SetAlarmBuilder;

impl SetAlarmBuilder {
    pub fn build(&self) -> SetAlarm {
        self.inner.clone()
    }

    pub fn seconds(&mut self, seconds: f32) -> &mut Self {
        self.inner.seconds = seconds;
        self
    }
}

impl AsRef<SetAlarm> for SetAlarm {
    fn as_ref(&self) -> &SetAlarm {
        self
    }
}

impl AsRef<SetAlarm> for SetAlarmBuilder {
    fn as_ref(&self) -> &SetAlarm {
        &self.inner
    }
}
