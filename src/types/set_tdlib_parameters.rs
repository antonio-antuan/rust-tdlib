use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the parameters for TDLib initialization. Works only when the current authorization state is authorizationStateWaitTdlibParameters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetTdlibParameters {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Parameters for TDLib initialization
    parameters: TdlibParameters,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetTdlibParameters {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetTdlibParameters {}

impl SetTdlibParameters {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetTdlibParametersBuilder {
        let mut inner = SetTdlibParameters::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setTdlibParameters".to_string();

        SetTdlibParametersBuilder { inner }
    }

    pub fn parameters(&self) -> &TdlibParameters {
        &self.parameters
    }
}

#[doc(hidden)]
pub struct SetTdlibParametersBuilder {
    inner: SetTdlibParameters,
}

#[deprecated]
pub type RTDSetTdlibParametersBuilder = SetTdlibParametersBuilder;

impl SetTdlibParametersBuilder {
    pub fn build(&self) -> SetTdlibParameters {
        self.inner.clone()
    }

    pub fn parameters<T: AsRef<TdlibParameters>>(&mut self, parameters: T) -> &mut Self {
        self.inner.parameters = parameters.as_ref().clone();
        self
    }
}

impl AsRef<SetTdlibParameters> for SetTdlibParameters {
    fn as_ref(&self) -> &SetTdlibParameters {
        self
    }
}

impl AsRef<SetTdlibParameters> for SetTdlibParametersBuilder {
    fn as_ref(&self) -> &SetTdlibParameters {
        &self.inner
    }
}
