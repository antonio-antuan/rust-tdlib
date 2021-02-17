use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Confirms QR code authentication on another device. Returns created session on success
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConfirmQrCodeAuthentication {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A link from a QR code. The link must be scanned by the in-app camera
    link: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ConfirmQrCodeAuthentication {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ConfirmQrCodeAuthentication {}

impl ConfirmQrCodeAuthentication {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDConfirmQrCodeAuthenticationBuilder {
        let mut inner = ConfirmQrCodeAuthentication::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "confirmQrCodeAuthentication".to_string();

        RTDConfirmQrCodeAuthenticationBuilder { inner }
    }

    pub fn link(&self) -> &String {
        &self.link
    }
}

#[doc(hidden)]
pub struct RTDConfirmQrCodeAuthenticationBuilder {
    inner: ConfirmQrCodeAuthentication,
}

impl RTDConfirmQrCodeAuthenticationBuilder {
    pub fn build(&self) -> ConfirmQrCodeAuthentication {
        self.inner.clone()
    }

    pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
        self.inner.link = link.as_ref().to_string();
        self
    }
}

impl AsRef<ConfirmQrCodeAuthentication> for ConfirmQrCodeAuthentication {
    fn as_ref(&self) -> &ConfirmQrCodeAuthentication {
        self
    }
}

impl AsRef<ConfirmQrCodeAuthentication> for RTDConfirmQrCodeAuthenticationBuilder {
    fn as_ref(&self) -> &ConfirmQrCodeAuthentication {
        &self.inner
    }
}
