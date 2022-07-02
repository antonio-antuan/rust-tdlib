use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Loads more participants of a group call. The loaded participants will be received through updates. Use the field groupCall.loaded_all_participants to check whether all participants have already been loaded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LoadGroupCallParticipants {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier. The group call must be previously received through getGroupCall and must be joined or being joined

    #[serde(default)]
    group_call_id: i32,
    /// The maximum number of participants to load; up to 100

    #[serde(default)]
    limit: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for LoadGroupCallParticipants {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for LoadGroupCallParticipants {}

impl LoadGroupCallParticipants {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> LoadGroupCallParticipantsBuilder {
        let mut inner = LoadGroupCallParticipants::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "loadGroupCallParticipants".to_string();

        LoadGroupCallParticipantsBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[doc(hidden)]
pub struct LoadGroupCallParticipantsBuilder {
    inner: LoadGroupCallParticipants,
}

#[deprecated]
pub type RTDLoadGroupCallParticipantsBuilder = LoadGroupCallParticipantsBuilder;

impl LoadGroupCallParticipantsBuilder {
    pub fn build(&self) -> LoadGroupCallParticipants {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn limit(&mut self, limit: i32) -> &mut Self {
        self.inner.limit = limit;
        self
    }
}

impl AsRef<LoadGroupCallParticipants> for LoadGroupCallParticipants {
    fn as_ref(&self) -> &LoadGroupCallParticipants {
        self
    }
}

impl AsRef<LoadGroupCallParticipants> for LoadGroupCallParticipantsBuilder {
    fn as_ref(&self) -> &LoadGroupCallParticipants {
        &self.inner
    }
}
