use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a server for relaying call data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CallServer {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Server identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    id: i64,
    /// Server IPv4 address

    #[serde(default)]
    ip_address: String,
    /// Server IPv6 address

    #[serde(default)]
    ipv6_address: String,
    /// Server port number

    #[serde(default)]
    port: i32,
    /// Server type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "CallServerType::_is_default")]
    type_: CallServerType,
}

impl RObject for CallServer {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl CallServer {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CallServerBuilder {
        let mut inner = CallServer::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        CallServerBuilder { inner }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn ip_address(&self) -> &String {
        &self.ip_address
    }

    pub fn ipv6_address(&self) -> &String {
        &self.ipv6_address
    }

    pub fn port(&self) -> i32 {
        self.port
    }

    pub fn type_(&self) -> &CallServerType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct CallServerBuilder {
    inner: CallServer,
}

#[deprecated]
pub type RTDCallServerBuilder = CallServerBuilder;

impl CallServerBuilder {
    pub fn build(&self) -> CallServer {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i64) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn ip_address<T: AsRef<str>>(&mut self, ip_address: T) -> &mut Self {
        self.inner.ip_address = ip_address.as_ref().to_string();
        self
    }

    pub fn ipv6_address<T: AsRef<str>>(&mut self, ipv6_address: T) -> &mut Self {
        self.inner.ipv6_address = ipv6_address.as_ref().to_string();
        self
    }

    pub fn port(&mut self, port: i32) -> &mut Self {
        self.inner.port = port;
        self
    }

    pub fn type_<T: AsRef<CallServerType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<CallServer> for CallServer {
    fn as_ref(&self) -> &CallServer {
        self
    }
}

impl AsRef<CallServer> for CallServerBuilder {
    fn as_ref(&self) -> &CallServer {
        &self.inner
    }
}
