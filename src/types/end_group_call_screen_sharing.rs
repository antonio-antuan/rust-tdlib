use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Ends screen sharing in a joined group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EndGroupCallScreenSharing {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for EndGroupCallScreenSharing {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for EndGroupCallScreenSharing {}

impl EndGroupCallScreenSharing {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EndGroupCallScreenSharingBuilder {
        let mut inner = EndGroupCallScreenSharing::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "endGroupCallScreenSharing".to_string();

        EndGroupCallScreenSharingBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct EndGroupCallScreenSharingBuilder {
    inner: EndGroupCallScreenSharing,
}

#[deprecated]
pub type RTDEndGroupCallScreenSharingBuilder = EndGroupCallScreenSharingBuilder;

impl EndGroupCallScreenSharingBuilder {
    pub fn build(&self) -> EndGroupCallScreenSharing {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<EndGroupCallScreenSharing> for EndGroupCallScreenSharing {
    fn as_ref(&self) -> &EndGroupCallScreenSharing {
        self
    }
}

impl AsRef<EndGroupCallScreenSharing> for EndGroupCallScreenSharingBuilder {
    fn as_ref(&self) -> &EndGroupCallScreenSharing {
        &self.inner
    }
}
