use crate::errors::Result;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetLogTagsBuilder {
        let mut inner = GetLogTags::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLogTags".to_string();

        GetLogTagsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetLogTagsBuilder {
    inner: GetLogTags,
}

#[deprecated]
pub type RTDGetLogTagsBuilder = GetLogTagsBuilder;

impl GetLogTagsBuilder {
    pub fn build(&self) -> GetLogTags {
        self.inner.clone()
    }
}

impl AsRef<GetLogTags> for GetLogTags {
    fn as_ref(&self) -> &GetLogTags {
        self
    }
}

impl AsRef<GetLogTags> for GetLogTagsBuilder {
    fn as_ref(&self) -> &GetLogTags {
        &self.inner
    }
}
