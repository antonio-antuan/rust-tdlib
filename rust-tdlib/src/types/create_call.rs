use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Creates a new call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user to be called
    user_id: i32,
    /// Description of the call protocols supported by the application
    protocol: CallProtocol,
    /// True, if a video call needs to be created
    is_video: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateCall {}

impl CreateCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateCallBuilder {
        let mut inner = CreateCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createCall".to_string();

        RTDCreateCallBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn protocol(&self) -> &CallProtocol {
        &self.protocol
    }

    pub fn is_video(&self) -> bool {
        self.is_video
    }
}

#[doc(hidden)]
pub struct RTDCreateCallBuilder {
    inner: CreateCall,
}

impl RTDCreateCallBuilder {
    pub fn build(&self) -> CreateCall {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn protocol<T: AsRef<CallProtocol>>(&mut self, protocol: T) -> &mut Self {
        self.inner.protocol = protocol.as_ref().clone();
        self
    }

    pub fn is_video(&mut self, is_video: bool) -> &mut Self {
        self.inner.is_video = is_video;
        self
    }
}

impl AsRef<CreateCall> for CreateCall {
    fn as_ref(&self) -> &CreateCall {
        self
    }
}

impl AsRef<CreateCall> for RTDCreateCallBuilder {
    fn as_ref(&self) -> &CreateCall {
        &self.inner
    }
}
