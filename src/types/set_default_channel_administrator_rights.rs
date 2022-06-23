use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets default administrator rights for adding the bot to channel chats; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetDefaultChannelAdministratorRights {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Default administrator rights for adding the bot to channels; may be null
    default_channel_administrator_rights: Option<ChatAdministratorRights>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetDefaultChannelAdministratorRights {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetDefaultChannelAdministratorRights {}

impl SetDefaultChannelAdministratorRights {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetDefaultChannelAdministratorRightsBuilder {
        let mut inner = SetDefaultChannelAdministratorRights::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setDefaultChannelAdministratorRights".to_string();

        RTDSetDefaultChannelAdministratorRightsBuilder { inner }
    }

    pub fn default_channel_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
        &self.default_channel_administrator_rights
    }
}

#[doc(hidden)]
pub struct RTDSetDefaultChannelAdministratorRightsBuilder {
    inner: SetDefaultChannelAdministratorRights,
}

impl RTDSetDefaultChannelAdministratorRightsBuilder {
    pub fn build(&self) -> SetDefaultChannelAdministratorRights {
        self.inner.clone()
    }

    pub fn default_channel_administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        default_channel_administrator_rights: T,
    ) -> &mut Self {
        self.inner.default_channel_administrator_rights =
            Some(default_channel_administrator_rights.as_ref().clone());
        self
    }
}

impl AsRef<SetDefaultChannelAdministratorRights> for SetDefaultChannelAdministratorRights {
    fn as_ref(&self) -> &SetDefaultChannelAdministratorRights {
        self
    }
}

impl AsRef<SetDefaultChannelAdministratorRights>
    for RTDSetDefaultChannelAdministratorRightsBuilder
{
    fn as_ref(&self) -> &SetDefaultChannelAdministratorRights {
        &self.inner
    }
}
