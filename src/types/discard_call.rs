use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Discards a call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiscardCall {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier

    #[serde(default)]
    call_id: i32,
    /// True, if the user was disconnected

    #[serde(default)]
    is_disconnected: bool,
    /// The call duration, in seconds

    #[serde(default)]
    duration: i32,
    /// True, if the call was a video call

    #[serde(default)]
    is_video: bool,
    /// Identifier of the connection used during the call

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    connection_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DiscardCall {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DiscardCall {}

impl DiscardCall {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DiscardCallBuilder {
        let mut inner = DiscardCall::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "discardCall".to_string();

        DiscardCallBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn is_disconnected(&self) -> bool {
        self.is_disconnected
    }

    pub fn duration(&self) -> i32 {
        self.duration
    }

    pub fn is_video(&self) -> bool {
        self.is_video
    }

    pub fn connection_id(&self) -> i64 {
        self.connection_id
    }
}

#[doc(hidden)]
pub struct DiscardCallBuilder {
    inner: DiscardCall,
}

#[deprecated]
pub type RTDDiscardCallBuilder = DiscardCallBuilder;

impl DiscardCallBuilder {
    pub fn build(&self) -> DiscardCall {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }

    pub fn is_disconnected(&mut self, is_disconnected: bool) -> &mut Self {
        self.inner.is_disconnected = is_disconnected;
        self
    }

    pub fn duration(&mut self, duration: i32) -> &mut Self {
        self.inner.duration = duration;
        self
    }

    pub fn is_video(&mut self, is_video: bool) -> &mut Self {
        self.inner.is_video = is_video;
        self
    }

    pub fn connection_id(&mut self, connection_id: i64) -> &mut Self {
        self.inner.connection_id = connection_id;
        self
    }
}

impl AsRef<DiscardCall> for DiscardCall {
    fn as_ref(&self) -> &DiscardCall {
        self
    }
}

impl AsRef<DiscardCall> for DiscardCallBuilder {
    fn as_ref(&self) -> &DiscardCall {
        &self.inner
    }
}
