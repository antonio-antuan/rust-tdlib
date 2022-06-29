use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a button to be shown instead of bot commands menu button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotMenuButton {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Text of the button

    #[serde(default)]
    text: String,
    /// URL to be passed to openWebApp

    #[serde(default)]
    url: String,
}

impl RObject for BotMenuButton {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl BotMenuButton {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDBotMenuButtonBuilder {
        let mut inner = BotMenuButton::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDBotMenuButtonBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn url(&self) -> &String {
        &self.url
    }
}

#[doc(hidden)]
pub struct RTDBotMenuButtonBuilder {
    inner: BotMenuButton,
}

impl RTDBotMenuButtonBuilder {
    pub fn build(&self) -> BotMenuButton {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }
}

impl AsRef<BotMenuButton> for BotMenuButton {
    fn as_ref(&self) -> &BotMenuButton {
        self
    }
}

impl AsRef<BotMenuButton> for RTDBotMenuButtonBuilder {
    fn as_ref(&self) -> &BotMenuButton {
        &self.inner
    }
}
