use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Specifies the supported call protocols
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProtocol {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if UDP peer-to-peer connections are supported
    udp_p2p: bool,
    /// True, if connection through UDP reflectors is supported
    udp_reflector: bool,
    /// The minimum supported API layer; use 65
    min_layer: i32,
    /// The maximum supported API layer; use 65
    max_layer: i32,
    /// List of supported libtgvoip versions
    library_versions: Vec<String>,
}

impl RObject for CallProtocol {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl CallProtocol {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCallProtocolBuilder {
        let mut inner = CallProtocol::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDCallProtocolBuilder { inner }
    }

    pub fn udp_p2p(&self) -> bool {
        self.udp_p2p
    }

    pub fn udp_reflector(&self) -> bool {
        self.udp_reflector
    }

    pub fn min_layer(&self) -> i32 {
        self.min_layer
    }

    pub fn max_layer(&self) -> i32 {
        self.max_layer
    }

    pub fn library_versions(&self) -> &Vec<String> {
        &self.library_versions
    }
}

#[doc(hidden)]
pub struct RTDCallProtocolBuilder {
    inner: CallProtocol,
}

impl RTDCallProtocolBuilder {
    pub fn build(&self) -> CallProtocol {
        self.inner.clone()
    }

    pub fn udp_p2p(&mut self, udp_p2p: bool) -> &mut Self {
        self.inner.udp_p2p = udp_p2p;
        self
    }

    pub fn udp_reflector(&mut self, udp_reflector: bool) -> &mut Self {
        self.inner.udp_reflector = udp_reflector;
        self
    }

    pub fn min_layer(&mut self, min_layer: i32) -> &mut Self {
        self.inner.min_layer = min_layer;
        self
    }

    pub fn max_layer(&mut self, max_layer: i32) -> &mut Self {
        self.inner.max_layer = max_layer;
        self
    }

    pub fn library_versions(&mut self, library_versions: Vec<String>) -> &mut Self {
        self.inner.library_versions = library_versions;
        self
    }
}

impl AsRef<CallProtocol> for CallProtocol {
    fn as_ref(&self) -> &CallProtocol {
        self
    }
}

impl AsRef<CallProtocol> for RTDCallProtocolBuilder {
    fn as_ref(&self) -> &CallProtocol {
        &self.inner
    }
}
