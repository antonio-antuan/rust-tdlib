use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends debug information for a call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCallDebugInformation {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier
    call_id: i32,
    /// Debug information in application-specific format
    debug_information: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendCallDebugInformation {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendCallDebugInformation {}

impl SendCallDebugInformation {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendCallDebugInformationBuilder {
        let mut inner = SendCallDebugInformation::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendCallDebugInformation".to_string();

        RTDSendCallDebugInformationBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn debug_information(&self) -> &String {
        &self.debug_information
    }
}

#[doc(hidden)]
pub struct RTDSendCallDebugInformationBuilder {
    inner: SendCallDebugInformation,
}

impl RTDSendCallDebugInformationBuilder {
    pub fn build(&self) -> SendCallDebugInformation {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }

    pub fn debug_information<T: AsRef<str>>(&mut self, debug_information: T) -> &mut Self {
        self.inner.debug_information = debug_information.as_ref().to_string();
        self
    }
}

impl AsRef<SendCallDebugInformation> for SendCallDebugInformation {
    fn as_ref(&self) -> &SendCallDebugInformation {
        self
    }
}

impl AsRef<SendCallDebugInformation> for RTDSendCallDebugInformationBuilder {
    fn as_ref(&self) -> &SendCallDebugInformation {
        &self.inner
    }
}
