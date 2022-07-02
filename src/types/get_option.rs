use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns the value of an option by its name. (Check the list of available options on https://core.telegram.org/tdlib/options.) Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The name of the option

    #[serde(default)]
    name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDOptionValue for GetOption {}

impl RFunction for GetOption {}

impl GetOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetOptionBuilder {
        let mut inner = GetOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getOption".to_string();

        GetOptionBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct GetOptionBuilder {
    inner: GetOption,
}

#[deprecated]
pub type RTDGetOptionBuilder = GetOptionBuilder;

impl GetOptionBuilder {
    pub fn build(&self) -> GetOption {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<GetOption> for GetOption {
    fn as_ref(&self) -> &GetOption {
        self
    }
}

impl AsRef<GetOption> for GetOptionBuilder {
    fn as_ref(&self) -> &GetOption {
        &self.inner
    }
}
