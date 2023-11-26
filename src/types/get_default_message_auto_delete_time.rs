use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns default message auto-delete time setting for new chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDefaultMessageAutoDeleteTime {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetDefaultMessageAutoDeleteTime {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetDefaultMessageAutoDeleteTime {}

impl GetDefaultMessageAutoDeleteTime {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetDefaultMessageAutoDeleteTimeBuilder {
        let mut inner = GetDefaultMessageAutoDeleteTime::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getDefaultMessageAutoDeleteTime".to_string();

        GetDefaultMessageAutoDeleteTimeBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetDefaultMessageAutoDeleteTimeBuilder {
    inner: GetDefaultMessageAutoDeleteTime,
}

#[deprecated]
pub type RTDGetDefaultMessageAutoDeleteTimeBuilder = GetDefaultMessageAutoDeleteTimeBuilder;

impl GetDefaultMessageAutoDeleteTimeBuilder {
    pub fn build(&self) -> GetDefaultMessageAutoDeleteTime {
        self.inner.clone()
    }
}

impl AsRef<GetDefaultMessageAutoDeleteTime> for GetDefaultMessageAutoDeleteTime {
    fn as_ref(&self) -> &GetDefaultMessageAutoDeleteTime {
        self
    }
}

impl AsRef<GetDefaultMessageAutoDeleteTime> for GetDefaultMessageAutoDeleteTimeBuilder {
    fn as_ref(&self) -> &GetDefaultMessageAutoDeleteTime {
        &self.inner
    }
}
