use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// An object of this type is returned on a successful function call for certain functions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Ok {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for Ok {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Ok {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> OkBuilder {
        let mut inner = Ok::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        OkBuilder { inner }
    }
}

#[doc(hidden)]
pub struct OkBuilder {
    inner: Ok,
}

#[deprecated]
pub type RTDOkBuilder = OkBuilder;

impl OkBuilder {
    pub fn build(&self) -> Ok {
        self.inner.clone()
    }
}

impl AsRef<Ok> for Ok {
    fn as_ref(&self) -> &Ok {
        self
    }
}

impl AsRef<Ok> for OkBuilder {
    fn as_ref(&self) -> &Ok {
        &self.inner
    }
}
