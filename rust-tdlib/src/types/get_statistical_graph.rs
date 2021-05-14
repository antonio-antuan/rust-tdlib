use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Loads an asynchronous or a zoomed in statistical graph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStatisticalGraph {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,
    /// The token for graph loading
    token: String,
    /// X-value for zoomed in graph or 0 otherwise
    x: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetStatisticalGraph {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStatisticalGraph for GetStatisticalGraph {}

impl RFunction for GetStatisticalGraph {}

impl GetStatisticalGraph {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetStatisticalGraphBuilder {
        let mut inner = GetStatisticalGraph::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getStatisticalGraph".to_string();

        RTDGetStatisticalGraphBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub fn x(&self) -> i64 {
        self.x
    }
}

#[doc(hidden)]
pub struct RTDGetStatisticalGraphBuilder {
    inner: GetStatisticalGraph,
}

impl RTDGetStatisticalGraphBuilder {
    pub fn build(&self) -> GetStatisticalGraph {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }

    pub fn x(&mut self, x: i64) -> &mut Self {
        self.inner.x = x;
        self
    }
}

impl AsRef<GetStatisticalGraph> for GetStatisticalGraph {
    fn as_ref(&self) -> &GetStatisticalGraph {
        self
    }
}

impl AsRef<GetStatisticalGraph> for RTDGetStatisticalGraphBuilder {
    fn as_ref(&self) -> &GetStatisticalGraph {
        &self.inner
    }
}
