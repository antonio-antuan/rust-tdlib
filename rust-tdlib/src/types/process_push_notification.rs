use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Handles a push notification. Returns error with code 406 if the push notification is not supported and connection to the server is required to fetch new data. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessPushNotification {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// JSON-encoded push notification payload with all fields sent by the server, and "google.sent_time" and "google.notification.sound" fields added
    payload: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ProcessPushNotification {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ProcessPushNotification {}

impl ProcessPushNotification {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDProcessPushNotificationBuilder {
        let mut inner = ProcessPushNotification::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "processPushNotification".to_string();

        RTDProcessPushNotificationBuilder { inner }
    }

    pub fn payload(&self) -> &String {
        &self.payload
    }
}

#[doc(hidden)]
pub struct RTDProcessPushNotificationBuilder {
    inner: ProcessPushNotification,
}

impl RTDProcessPushNotificationBuilder {
    pub fn build(&self) -> ProcessPushNotification {
        self.inner.clone()
    }

    pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
        self.inner.payload = payload.as_ref().to_string();
        self
    }
}

impl AsRef<ProcessPushNotification> for ProcessPushNotification {
    fn as_ref(&self) -> &ProcessPushNotification {
        self
    }
}

impl AsRef<ProcessPushNotification> for RTDProcessPushNotificationBuilder {
    fn as_ref(&self) -> &ProcessPushNotification {
        &self.inner
    }
}
