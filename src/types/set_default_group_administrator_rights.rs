use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets default administrator rights for adding the bot to basic group and supergroup chats; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetDefaultGroupAdministratorRights {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Default administrator rights for adding the bot to basic group and supergroup chats; may be null
    default_group_administrator_rights: Option<ChatAdministratorRights>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetDefaultGroupAdministratorRights {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetDefaultGroupAdministratorRights {}

impl SetDefaultGroupAdministratorRights {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetDefaultGroupAdministratorRightsBuilder {
        let mut inner = SetDefaultGroupAdministratorRights::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setDefaultGroupAdministratorRights".to_string();

        RTDSetDefaultGroupAdministratorRightsBuilder { inner }
    }

    pub fn default_group_administrator_rights(&self) -> &Option<ChatAdministratorRights> {
        &self.default_group_administrator_rights
    }
}

#[doc(hidden)]
pub struct RTDSetDefaultGroupAdministratorRightsBuilder {
    inner: SetDefaultGroupAdministratorRights,
}

impl RTDSetDefaultGroupAdministratorRightsBuilder {
    pub fn build(&self) -> SetDefaultGroupAdministratorRights {
        self.inner.clone()
    }

    pub fn default_group_administrator_rights<T: AsRef<ChatAdministratorRights>>(
        &mut self,
        default_group_administrator_rights: T,
    ) -> &mut Self {
        self.inner.default_group_administrator_rights =
            Some(default_group_administrator_rights.as_ref().clone());
        self
    }
}

impl AsRef<SetDefaultGroupAdministratorRights> for SetDefaultGroupAdministratorRights {
    fn as_ref(&self) -> &SetDefaultGroupAdministratorRights {
        self
    }
}

impl AsRef<SetDefaultGroupAdministratorRights> for RTDSetDefaultGroupAdministratorRightsBuilder {
    fn as_ref(&self) -> &SetDefaultGroupAdministratorRights {
        &self.inner
    }
}
