use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Theme colors for a payment form
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PaymentFormTheme {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A color of the payment form background in the RGB24 format

    #[serde(default)]
    background_color: i32,
    /// A color of text in the RGB24 format

    #[serde(default)]
    text_color: i32,
    /// A color of hints in the RGB24 format

    #[serde(default)]
    hint_color: i32,
    /// A color of links in the RGB24 format

    #[serde(default)]
    link_color: i32,
    /// A color of the buttons in the RGB24 format

    #[serde(default)]
    button_color: i32,
    /// A color of text on the buttons in the RGB24 format

    #[serde(default)]
    button_text_color: i32,
}

impl RObject for PaymentFormTheme {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl PaymentFormTheme {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PaymentFormThemeBuilder {
        let mut inner = PaymentFormTheme::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PaymentFormThemeBuilder { inner }
    }

    pub fn background_color(&self) -> i32 {
        self.background_color
    }

    pub fn text_color(&self) -> i32 {
        self.text_color
    }

    pub fn hint_color(&self) -> i32 {
        self.hint_color
    }

    pub fn link_color(&self) -> i32 {
        self.link_color
    }

    pub fn button_color(&self) -> i32 {
        self.button_color
    }

    pub fn button_text_color(&self) -> i32 {
        self.button_text_color
    }
}

#[doc(hidden)]
pub struct PaymentFormThemeBuilder {
    inner: PaymentFormTheme,
}

#[deprecated]
pub type RTDPaymentFormThemeBuilder = PaymentFormThemeBuilder;

impl PaymentFormThemeBuilder {
    pub fn build(&self) -> PaymentFormTheme {
        self.inner.clone()
    }

    pub fn background_color(&mut self, background_color: i32) -> &mut Self {
        self.inner.background_color = background_color;
        self
    }

    pub fn text_color(&mut self, text_color: i32) -> &mut Self {
        self.inner.text_color = text_color;
        self
    }

    pub fn hint_color(&mut self, hint_color: i32) -> &mut Self {
        self.inner.hint_color = hint_color;
        self
    }

    pub fn link_color(&mut self, link_color: i32) -> &mut Self {
        self.inner.link_color = link_color;
        self
    }

    pub fn button_color(&mut self, button_color: i32) -> &mut Self {
        self.inner.button_color = button_color;
        self
    }

    pub fn button_text_color(&mut self, button_text_color: i32) -> &mut Self {
        self.inner.button_text_color = button_text_color;
        self
    }
}

impl AsRef<PaymentFormTheme> for PaymentFormTheme {
    fn as_ref(&self) -> &PaymentFormTheme {
        self
    }
}

impl AsRef<PaymentFormTheme> for PaymentFormThemeBuilder {
    fn as_ref(&self) -> &PaymentFormTheme {
        &self.inner
    }
}
