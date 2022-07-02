use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains Telegram terms of service
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TermsOfService {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text of the terms of service
    text: FormattedText,
    /// The minimum age of a user to be able to accept the terms; 0 if any

    #[serde(default)]
    min_user_age: i32,
    /// True, if a blocking popup with terms of service must be shown to the user

    #[serde(default)]
    show_popup: bool,
}

impl RObject for TermsOfService {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TermsOfService {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TermsOfServiceBuilder {
        let mut inner = TermsOfService::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TermsOfServiceBuilder { inner }
    }

    pub fn text(&self) -> &FormattedText {
        &self.text
    }

    pub fn min_user_age(&self) -> i32 {
        self.min_user_age
    }

    pub fn show_popup(&self) -> bool {
        self.show_popup
    }
}

#[doc(hidden)]
pub struct TermsOfServiceBuilder {
    inner: TermsOfService,
}

#[deprecated]
pub type RTDTermsOfServiceBuilder = TermsOfServiceBuilder;

impl TermsOfServiceBuilder {
    pub fn build(&self) -> TermsOfService {
        self.inner.clone()
    }

    pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().clone();
        self
    }

    pub fn min_user_age(&mut self, min_user_age: i32) -> &mut Self {
        self.inner.min_user_age = min_user_age;
        self
    }

    pub fn show_popup(&mut self, show_popup: bool) -> &mut Self {
        self.inner.show_popup = show_popup;
        self
    }
}

impl AsRef<TermsOfService> for TermsOfService {
    fn as_ref(&self) -> &TermsOfService {
        self
    }
}

impl AsRef<TermsOfService> for TermsOfServiceBuilder {
    fn as_ref(&self) -> &TermsOfService {
        &self.inner
    }
}
