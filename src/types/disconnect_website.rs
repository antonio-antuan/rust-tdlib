use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Disconnects website from the current user's Telegram account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisconnectWebsite {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Website identifier

    #[serde(
        deserialize_with = "super::_common::number_from_string",
        serialize_with = "super::_common::string_to_number"
    )]
    #[serde(default)]
    website_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DisconnectWebsite {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DisconnectWebsite {}

impl DisconnectWebsite {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DisconnectWebsiteBuilder {
        let mut inner = DisconnectWebsite::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "disconnectWebsite".to_string();

        DisconnectWebsiteBuilder { inner }
    }

    pub fn website_id(&self) -> i64 {
        self.website_id
    }
}

#[doc(hidden)]
pub struct DisconnectWebsiteBuilder {
    inner: DisconnectWebsite,
}

#[deprecated]
pub type RTDDisconnectWebsiteBuilder = DisconnectWebsiteBuilder;

impl DisconnectWebsiteBuilder {
    pub fn build(&self) -> DisconnectWebsite {
        self.inner.clone()
    }

    pub fn website_id(&mut self, website_id: i64) -> &mut Self {
        self.inner.website_id = website_id;
        self
    }
}

impl AsRef<DisconnectWebsite> for DisconnectWebsite {
    fn as_ref(&self) -> &DisconnectWebsite {
        self
    }
}

impl AsRef<DisconnectWebsite> for DisconnectWebsiteBuilder {
    fn as_ref(&self) -> &DisconnectWebsite {
        &self.inner
    }
}
