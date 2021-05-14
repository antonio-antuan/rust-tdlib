use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns an existing chat corresponding to a known basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBasicGroupChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Basic group identifier
    basic_group_id: i32,
    /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect
    force: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CreateBasicGroupChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for CreateBasicGroupChat {}

impl CreateBasicGroupChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDCreateBasicGroupChatBuilder {
        let mut inner = CreateBasicGroupChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "createBasicGroupChat".to_string();

        RTDCreateBasicGroupChatBuilder { inner }
    }

    pub fn basic_group_id(&self) -> i32 {
        self.basic_group_id
    }

    pub fn force(&self) -> bool {
        self.force
    }
}

#[doc(hidden)]
pub struct RTDCreateBasicGroupChatBuilder {
    inner: CreateBasicGroupChat,
}

impl RTDCreateBasicGroupChatBuilder {
    pub fn build(&self) -> CreateBasicGroupChat {
        self.inner.clone()
    }

    pub fn basic_group_id(&mut self, basic_group_id: i32) -> &mut Self {
        self.inner.basic_group_id = basic_group_id;
        self
    }

    pub fn force(&mut self, force: bool) -> &mut Self {
        self.inner.force = force;
        self
    }
}

impl AsRef<CreateBasicGroupChat> for CreateBasicGroupChat {
    fn as_ref(&self) -> &CreateBasicGroupChat {
        self
    }
}

impl AsRef<CreateBasicGroupChat> for RTDCreateBasicGroupChatBuilder {
    fn as_ref(&self) -> &CreateBasicGroupChat {
        &self.inner
    }
}
