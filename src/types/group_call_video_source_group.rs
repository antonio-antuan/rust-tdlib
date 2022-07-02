use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a group of video synchronization source identifiers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallVideoSourceGroup {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The semantics of sources, one of "SIM" or "FID"

    #[serde(default)]
    semantics: String,
    /// The list of synchronization source identifiers

    #[serde(default)]
    source_ids: Vec<i32>,
}

impl RObject for GroupCallVideoSourceGroup {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCallVideoSourceGroup {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallVideoSourceGroupBuilder {
        let mut inner = GroupCallVideoSourceGroup::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallVideoSourceGroupBuilder { inner }
    }

    pub fn semantics(&self) -> &String {
        &self.semantics
    }

    pub fn source_ids(&self) -> &Vec<i32> {
        &self.source_ids
    }
}

#[doc(hidden)]
pub struct GroupCallVideoSourceGroupBuilder {
    inner: GroupCallVideoSourceGroup,
}

#[deprecated]
pub type RTDGroupCallVideoSourceGroupBuilder = GroupCallVideoSourceGroupBuilder;

impl GroupCallVideoSourceGroupBuilder {
    pub fn build(&self) -> GroupCallVideoSourceGroup {
        self.inner.clone()
    }

    pub fn semantics<T: AsRef<str>>(&mut self, semantics: T) -> &mut Self {
        self.inner.semantics = semantics.as_ref().to_string();
        self
    }

    pub fn source_ids(&mut self, source_ids: Vec<i32>) -> &mut Self {
        self.inner.source_ids = source_ids;
        self
    }
}

impl AsRef<GroupCallVideoSourceGroup> for GroupCallVideoSourceGroup {
    fn as_ref(&self) -> &GroupCallVideoSourceGroup {
        self
    }
}

impl AsRef<GroupCallVideoSourceGroup> for GroupCallVideoSourceGroupBuilder {
    fn as_ref(&self) -> &GroupCallVideoSourceGroup {
        &self.inner
    }
}
