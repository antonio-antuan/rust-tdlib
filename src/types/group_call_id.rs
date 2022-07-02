use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains the group call identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallId {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    id: i32,
}

impl RObject for GroupCallId {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCallId {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallIdBuilder {
        let mut inner = GroupCallId::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallIdBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}

#[doc(hidden)]
pub struct GroupCallIdBuilder {
    inner: GroupCallId,
}

#[deprecated]
pub type RTDGroupCallIdBuilder = GroupCallIdBuilder;

impl GroupCallIdBuilder {
    pub fn build(&self) -> GroupCallId {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }
}

impl AsRef<GroupCallId> for GroupCallId {
    fn as_ref(&self) -> &GroupCallId {
        self
    }
}

impl AsRef<GroupCallId> for GroupCallIdBuilder {
    fn as_ref(&self) -> &GroupCallId {
        &self.inner
    }
}
