use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the value of an option. (Check the list of available options on https://core.telegram.org/tdlib/options.) Only writable options can be set. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetOption {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The name of the option

    #[serde(default)]
    name: String,
    /// The new value of the option; pass null to reset option value to a default value

    #[serde(skip_serializing_if = "OptionValue::_is_default")]
    value: OptionValue,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetOption {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetOption {}

impl SetOption {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetOptionBuilder {
        let mut inner = SetOption::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setOption".to_string();

        SetOptionBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn value(&self) -> &OptionValue {
        &self.value
    }
}

#[doc(hidden)]
pub struct SetOptionBuilder {
    inner: SetOption,
}

#[deprecated]
pub type RTDSetOptionBuilder = SetOptionBuilder;

impl SetOptionBuilder {
    pub fn build(&self) -> SetOption {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn value<T: AsRef<OptionValue>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().clone();
        self
    }
}

impl AsRef<SetOption> for SetOption {
    fn as_ref(&self) -> &SetOption {
        self
    }
}

impl AsRef<SetOption> for SetOptionBuilder {
    fn as_ref(&self) -> &SetOption {
        &self.inner
    }
}
