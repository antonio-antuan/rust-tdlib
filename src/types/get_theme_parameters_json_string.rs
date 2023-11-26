use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Converts a themeParameters object to corresponding JSON-serialized string. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetThemeParametersJsonString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Theme parameters to convert to JSON
    theme: ThemeParameters,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetThemeParametersJsonString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetThemeParametersJsonString {}

impl GetThemeParametersJsonString {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetThemeParametersJsonStringBuilder {
        let mut inner = GetThemeParametersJsonString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getThemeParametersJsonString".to_string();

        GetThemeParametersJsonStringBuilder { inner }
    }

    pub fn theme(&self) -> &ThemeParameters {
        &self.theme
    }
}

#[doc(hidden)]
pub struct GetThemeParametersJsonStringBuilder {
    inner: GetThemeParametersJsonString,
}

#[deprecated]
pub type RTDGetThemeParametersJsonStringBuilder = GetThemeParametersJsonStringBuilder;

impl GetThemeParametersJsonStringBuilder {
    pub fn build(&self) -> GetThemeParametersJsonString {
        self.inner.clone()
    }

    pub fn theme<T: AsRef<ThemeParameters>>(&mut self, theme: T) -> &mut Self {
        self.inner.theme = theme.as_ref().clone();
        self
    }
}

impl AsRef<GetThemeParametersJsonString> for GetThemeParametersJsonString {
    fn as_ref(&self) -> &GetThemeParametersJsonString {
        self
    }
}

impl AsRef<GetThemeParametersJsonString> for GetThemeParametersJsonStringBuilder {
    fn as_ref(&self) -> &GetThemeParametersJsonString {
        &self.inner
    }
}
