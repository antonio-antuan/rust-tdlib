use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns an existing chat corresponding to a known supergroup or channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSupergroupChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Supergroup or channel identifier

    #[serde(default)]
    supergroup_id: i64,
    /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect

    #[serde(default)]
    force: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateSupergroupChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateSupergroupChat {}

impl CreateSupergroupChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CreateSupergroupChatBuilder {
        let mut inner = CreateSupergroupChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createSupergroupChat".to_string();

        CreateSupergroupChatBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn force(&self) -> bool {
        self.force
    }
}

#[doc(hidden)]
pub struct CreateSupergroupChatBuilder {
    inner: CreateSupergroupChat,
}

#[deprecated]
pub type RTDCreateSupergroupChatBuilder = CreateSupergroupChatBuilder;

impl CreateSupergroupChatBuilder {
    pub fn build(&self) -> CreateSupergroupChat {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn force(&mut self, force: bool) -> &mut Self {
        self.inner.force = force;
        self
    }
}

impl AsRef<CreateSupergroupChat> for CreateSupergroupChat {
    fn as_ref(&self) -> &CreateSupergroupChat {
        self
    }
}

impl AsRef<CreateSupergroupChat> for CreateSupergroupChatBuilder {
    fn as_ref(&self) -> &CreateSupergroupChat {
        &self.inner
    }
}
