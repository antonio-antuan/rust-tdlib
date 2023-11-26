use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends a custom request from a Web App
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendWebAppCustomRequest {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the bot

    #[serde(default)]
    bot_user_id: i64,
    /// The method name

    #[serde(default)]
    method: String,
    /// JSON-serialized method parameters

    #[serde(default)]
    parameters: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendWebAppCustomRequest {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendWebAppCustomRequest {}

impl SendWebAppCustomRequest {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendWebAppCustomRequestBuilder {
        let mut inner = SendWebAppCustomRequest::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendWebAppCustomRequest".to_string();

        SendWebAppCustomRequestBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn method(&self) -> &String {
        &self.method
    }

    pub fn parameters(&self) -> &String {
        &self.parameters
    }
}

#[doc(hidden)]
pub struct SendWebAppCustomRequestBuilder {
    inner: SendWebAppCustomRequest,
}

#[deprecated]
pub type RTDSendWebAppCustomRequestBuilder = SendWebAppCustomRequestBuilder;

impl SendWebAppCustomRequestBuilder {
    pub fn build(&self) -> SendWebAppCustomRequest {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn method<T: AsRef<str>>(&mut self, method: T) -> &mut Self {
        self.inner.method = method.as_ref().to_string();
        self
    }

    pub fn parameters<T: AsRef<str>>(&mut self, parameters: T) -> &mut Self {
        self.inner.parameters = parameters.as_ref().to_string();
        self
    }
}

impl AsRef<SendWebAppCustomRequest> for SendWebAppCustomRequest {
    fn as_ref(&self) -> &SendWebAppCustomRequest {
        self
    }
}

impl AsRef<SendWebAppCustomRequest> for SendWebAppCustomRequestBuilder {
    fn as_ref(&self) -> &SendWebAppCustomRequest {
        &self.inner
    }
}
