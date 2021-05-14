use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an existing chat corresponding to a given user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatePrivateChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// User identifier
    user_id: i32,
    /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect
    force: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreatePrivateChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreatePrivateChat {}

impl CreatePrivateChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreatePrivateChatBuilder {
        let mut inner = CreatePrivateChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createPrivateChat".to_string();

        RTDCreatePrivateChatBuilder { inner }
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn force(&self) -> bool {
        self.force
    }
}

#[doc(hidden)]
pub struct RTDCreatePrivateChatBuilder {
    inner: CreatePrivateChat,
}

impl RTDCreatePrivateChatBuilder {
    pub fn build(&self) -> CreatePrivateChat {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i32) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn force(&mut self, force: bool) -> &mut Self {
        self.inner.force = force;
        self
    }
}

impl AsRef<CreatePrivateChat> for CreatePrivateChat {
    fn as_ref(&self) -> &CreatePrivateChat {
        self
    }
}

impl AsRef<CreatePrivateChat> for RTDCreatePrivateChatBuilder {
    fn as_ref(&self) -> &CreatePrivateChat {
        &self.inner
    }
}
