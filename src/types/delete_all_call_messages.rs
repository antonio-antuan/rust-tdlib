use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Deletes all call messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeleteAllCallMessages {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Pass true to delete the messages for all users
    revoke: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for DeleteAllCallMessages {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for DeleteAllCallMessages {}

impl DeleteAllCallMessages {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDeleteAllCallMessagesBuilder {
        let mut inner = DeleteAllCallMessages::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "deleteAllCallMessages".to_string();

        RTDDeleteAllCallMessagesBuilder { inner }
    }

    pub fn revoke(&self) -> bool {
        self.revoke
    }
}

#[doc(hidden)]
pub struct RTDDeleteAllCallMessagesBuilder {
    inner: DeleteAllCallMessages,
}

impl RTDDeleteAllCallMessagesBuilder {
    pub fn build(&self) -> DeleteAllCallMessages {
        self.inner.clone()
    }

    pub fn revoke(&mut self, revoke: bool) -> &mut Self {
        self.inner.revoke = revoke;
        self
    }
}

impl AsRef<DeleteAllCallMessages> for DeleteAllCallMessages {
    fn as_ref(&self) -> &DeleteAllCallMessages {
        self
    }
}

impl AsRef<DeleteAllCallMessages> for RTDDeleteAllCallMessagesBuilder {
    fn as_ref(&self) -> &DeleteAllCallMessages {
        &self.inner
    }
}
