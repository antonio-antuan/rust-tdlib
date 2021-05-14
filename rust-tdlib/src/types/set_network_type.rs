use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetNetworkType {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The new network type. By default, networkTypeOther

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "NetworkType::_is_default")]
    type_: NetworkType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetNetworkType {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetNetworkType {}

impl SetNetworkType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetNetworkTypeBuilder {
        let mut inner = SetNetworkType::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setNetworkType".to_string();

        RTDSetNetworkTypeBuilder { inner }
    }

    pub fn type_(&self) -> &NetworkType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDSetNetworkTypeBuilder {
    inner: SetNetworkType,
}

impl RTDSetNetworkTypeBuilder {
    pub fn build(&self) -> SetNetworkType {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<NetworkType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<SetNetworkType> for SetNetworkType {
    fn as_ref(&self) -> &SetNetworkType {
        self
    }
}

impl AsRef<SetNetworkType> for RTDSetNetworkTypeBuilder {
    fn as_ref(&self) -> &SetNetworkType {
        &self.inner
    }
}
