use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Informs the user that some of the elements in their Telegram Passport contain errors; for bots only. The user will not be able to resend the elements, until the errors are fixed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetPassportElementErrors {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier

    #[serde(default)]
    user_id: i64,
    /// The errors

    #[serde(default)]
    errors: Vec<InputPassportElementError>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetPassportElementErrors {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetPassportElementErrors {}

impl SetPassportElementErrors {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetPassportElementErrorsBuilder {
        let mut inner = SetPassportElementErrors::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setPassportElementErrors".to_string();

        SetPassportElementErrorsBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn errors(&self) -> &Vec<InputPassportElementError> {
        &self.errors
    }
}

#[doc(hidden)]
pub struct SetPassportElementErrorsBuilder {
    inner: SetPassportElementErrors,
}

#[deprecated]
pub type RTDSetPassportElementErrorsBuilder = SetPassportElementErrorsBuilder;

impl SetPassportElementErrorsBuilder {
    pub fn build(&self) -> SetPassportElementErrors {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn errors(&mut self, errors: Vec<InputPassportElementError>) -> &mut Self {
        self.inner.errors = errors;
        self
    }
}

impl AsRef<SetPassportElementErrors> for SetPassportElementErrors {
    fn as_ref(&self) -> &SetPassportElementErrors {
        self
    }
}

impl AsRef<SetPassportElementErrors> for SetPassportElementErrorsBuilder {
    fn as_ref(&self) -> &SetPassportElementErrors {
        &self.inner
    }
}
