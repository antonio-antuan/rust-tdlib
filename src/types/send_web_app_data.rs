use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sends data received from a keyboardButtonTypeWebApp Web App to a bot
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendWebAppData {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// Text of the keyboardButtonTypeWebApp button, which opened the Web App

    #[serde(default)]
    button_text: String,
    /// Received data

    #[serde(default)]
    data: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendWebAppData {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendWebAppData {}

impl SendWebAppData {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSendWebAppDataBuilder {
        let mut inner = SendWebAppData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendWebAppData".to_string();

        RTDSendWebAppDataBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn button_text(&self) -> &String {
        &self.button_text
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}

#[doc(hidden)]
pub struct RTDSendWebAppDataBuilder {
    inner: SendWebAppData,
}

impl RTDSendWebAppDataBuilder {
    pub fn build(&self) -> SendWebAppData {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn button_text<T: AsRef<str>>(&mut self, button_text: T) -> &mut Self {
        self.inner.button_text = button_text.as_ref().to_string();
        self
    }

    pub fn data<T: AsRef<str>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().to_string();
        self
    }
}

impl AsRef<SendWebAppData> for SendWebAppData {
    fn as_ref(&self) -> &SendWebAppData {
        self
    }
}

impl AsRef<SendWebAppData> for RTDSendWebAppDataBuilder {
    fn as_ref(&self) -> &SendWebAppData {
        &self.inner
    }
}
