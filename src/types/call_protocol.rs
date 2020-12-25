
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Specifies the supported call protocols
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallProtocol {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if UDP peer-to-peer connections are supported
  udp_p2p: bool,
  /// True, if connection through UDP reflectors is supported
  udp_reflector: bool,
  /// The minimum supported API layer; use 65
  min_layer: i64,
  /// The maximum supported API layer; use 65
  max_layer: i64,
  
}

impl RObject for CallProtocol {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callProtocol" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl CallProtocol {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCallProtocolBuilder {
    let mut inner = CallProtocol::default();
    inner.td_name = "callProtocol".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCallProtocolBuilder { inner }
  }

  pub fn udp_p2p(&self) -> bool { self.udp_p2p }

  pub fn udp_reflector(&self) -> bool { self.udp_reflector }

  pub fn min_layer(&self) -> i64 { self.min_layer }

  pub fn max_layer(&self) -> i64 { self.max_layer }

}

#[doc(hidden)]
pub struct RTDCallProtocolBuilder {
  inner: CallProtocol
}

impl RTDCallProtocolBuilder {
  pub fn build(&self) -> CallProtocol { self.inner.clone() }

   
  pub fn udp_p2p(&mut self, udp_p2p: bool) -> &mut Self {
    self.inner.udp_p2p = udp_p2p;
    self
  }

   
  pub fn udp_reflector(&mut self, udp_reflector: bool) -> &mut Self {
    self.inner.udp_reflector = udp_reflector;
    self
  }

   
  pub fn min_layer(&mut self, min_layer: i64) -> &mut Self {
    self.inner.min_layer = min_layer;
    self
  }

   
  pub fn max_layer(&mut self, max_layer: i64) -> &mut Self {
    self.inner.max_layer = max_layer;
    self
  }

}

impl AsRef<CallProtocol> for CallProtocol {
  fn as_ref(&self) -> &CallProtocol { self }
}

impl AsRef<CallProtocol> for RTDCallProtocolBuilder {
  fn as_ref(&self) -> &CallProtocol { &self.inner }
}



