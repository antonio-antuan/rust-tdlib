use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires owner privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteSupergroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup or channel
    supergroup_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteSupergroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteSupergroup {}

impl DeleteSupergroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteSupergroupBuilder {
        let mut inner = DeleteSupergroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteSupergroup".to_string();

        RTDDeleteSupergroupBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i32 {
        self.supergroup_id
    }
}

#[doc(hidden)]
pub struct RTDDeleteSupergroupBuilder {
    inner: DeleteSupergroup,
}

impl RTDDeleteSupergroupBuilder {
    pub fn build(&self) -> DeleteSupergroup {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i32) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }
}

impl AsRef<DeleteSupergroup> for DeleteSupergroup {
    fn as_ref(&self) -> &DeleteSupergroup {
        self
    }
}

impl AsRef<DeleteSupergroup> for RTDDeleteSupergroupBuilder {
    fn as_ref(&self) -> &DeleteSupergroup {
        &self.inner
    }
}
