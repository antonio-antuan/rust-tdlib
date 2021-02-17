use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogTags {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLogTags {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetLogTags {}

impl GetLogTags {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetLogTagsBuilder {
        let mut inner = GetLogTags::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLogTags".to_string();

        RTDGetLogTagsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetLogTagsBuilder {
    inner: GetLogTags,
}

impl RTDGetLogTagsBuilder {
    pub fn build(&self) -> GetLogTags {
        self.inner.clone()
    }
}

impl AsRef<GetLogTags> for GetLogTags {
    fn as_ref(&self) -> &GetLogTags {
        self
    }
}

impl AsRef<GetLogTags> for RTDGetLogTagsBuilder {
    fn as_ref(&self) -> &GetLogTags {
        &self.inner
    }
}
