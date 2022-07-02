use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Checks whether the current session can be used to transfer a chat ownership to another user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CanTransferOwnership {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for CanTransferOwnership {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDCanTransferOwnershipResult for CanTransferOwnership {}

impl RFunction for CanTransferOwnership {}

impl CanTransferOwnership {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> CanTransferOwnershipBuilder {
        let mut inner = CanTransferOwnership::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "canTransferOwnership".to_string();

        CanTransferOwnershipBuilder { inner }
    }
}

#[doc(hidden)]
pub struct CanTransferOwnershipBuilder {
    inner: CanTransferOwnership,
}

#[deprecated]
pub type RTDCanTransferOwnershipBuilder = CanTransferOwnershipBuilder;

impl CanTransferOwnershipBuilder {
    pub fn build(&self) -> CanTransferOwnership {
        self.inner.clone()
    }
}

impl AsRef<CanTransferOwnership> for CanTransferOwnership {
    fn as_ref(&self) -> &CanTransferOwnership {
        self
    }
}

impl AsRef<CanTransferOwnership> for CanTransferOwnershipBuilder {
    fn as_ref(&self) -> &CanTransferOwnership {
        &self.inner
    }
}
