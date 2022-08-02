use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Describes a statistical graph
pub trait TDStatisticalGraph: Debug + RObject {}

/// Describes a statistical graph
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum StatisticalGraph {
    #[doc(hidden)]
    _Default,
    /// Loads an asynchronous or a zoomed in statistical graph
    #[serde(rename = "getStatisticalGraph")]
    GetStatisticalGraph(GetStatisticalGraph),
    /// The graph data to be asynchronously loaded through getStatisticalGraph
    #[serde(rename = "statisticalGraphAsync")]
    Async(StatisticalGraphAsync),
    /// A graph data
    #[serde(rename = "statisticalGraphData")]
    Data(StatisticalGraphData),
    /// An error message to be shown to the user instead of the graph
    #[serde(rename = "statisticalGraphError")]
    Error(StatisticalGraphError),
}

impl Default for StatisticalGraph {
    fn default() -> Self {
        StatisticalGraph::_Default
    }
}

impl RObject for StatisticalGraph {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            StatisticalGraph::GetStatisticalGraph(t) => t.extra(),
            StatisticalGraph::Async(t) => t.extra(),
            StatisticalGraph::Data(t) => t.extra(),
            StatisticalGraph::Error(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            StatisticalGraph::GetStatisticalGraph(t) => t.client_id(),
            StatisticalGraph::Async(t) => t.client_id(),
            StatisticalGraph::Data(t) => t.client_id(),
            StatisticalGraph::Error(t) => t.client_id(),

            _ => None,
        }
    }
}

impl StatisticalGraph {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, StatisticalGraph::_Default)
    }
}

impl AsRef<StatisticalGraph> for StatisticalGraph {
    fn as_ref(&self) -> &StatisticalGraph {
        self
    }
}

/// The graph data to be asynchronously loaded through getStatisticalGraph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticalGraphAsync {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The token to use for data loading

    #[serde(default)]
    token: String,
}

impl RObject for StatisticalGraphAsync {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStatisticalGraph for StatisticalGraphAsync {}

impl StatisticalGraphAsync {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StatisticalGraphAsyncBuilder {
        let mut inner = StatisticalGraphAsync::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StatisticalGraphAsyncBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct StatisticalGraphAsyncBuilder {
    inner: StatisticalGraphAsync,
}

#[deprecated]
pub type RTDStatisticalGraphAsyncBuilder = StatisticalGraphAsyncBuilder;

impl StatisticalGraphAsyncBuilder {
    pub fn build(&self) -> StatisticalGraphAsync {
        self.inner.clone()
    }

    pub fn token<T: AsRef<str>>(&mut self, token: T) -> &mut Self {
        self.inner.token = token.as_ref().to_string();
        self
    }
}

impl AsRef<StatisticalGraphAsync> for StatisticalGraphAsync {
    fn as_ref(&self) -> &StatisticalGraphAsync {
        self
    }
}

impl AsRef<StatisticalGraphAsync> for StatisticalGraphAsyncBuilder {
    fn as_ref(&self) -> &StatisticalGraphAsync {
        &self.inner
    }
}

/// A graph data
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticalGraphData {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Graph data in JSON format

    #[serde(default)]
    json_data: String,
    /// If non-empty, a token which can be used to receive a zoomed in graph

    #[serde(default)]
    zoom_token: String,
}

impl RObject for StatisticalGraphData {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStatisticalGraph for StatisticalGraphData {}

impl StatisticalGraphData {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StatisticalGraphDataBuilder {
        let mut inner = StatisticalGraphData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StatisticalGraphDataBuilder { inner }
    }

    pub fn json_data(&self) -> &String {
        &self.json_data
    }

    pub fn zoom_token(&self) -> &String {
        &self.zoom_token
    }
}

#[doc(hidden)]
pub struct StatisticalGraphDataBuilder {
    inner: StatisticalGraphData,
}

#[deprecated]
pub type RTDStatisticalGraphDataBuilder = StatisticalGraphDataBuilder;

impl StatisticalGraphDataBuilder {
    pub fn build(&self) -> StatisticalGraphData {
        self.inner.clone()
    }

    pub fn json_data<T: AsRef<str>>(&mut self, json_data: T) -> &mut Self {
        self.inner.json_data = json_data.as_ref().to_string();
        self
    }

    pub fn zoom_token<T: AsRef<str>>(&mut self, zoom_token: T) -> &mut Self {
        self.inner.zoom_token = zoom_token.as_ref().to_string();
        self
    }
}

impl AsRef<StatisticalGraphData> for StatisticalGraphData {
    fn as_ref(&self) -> &StatisticalGraphData {
        self
    }
}

impl AsRef<StatisticalGraphData> for StatisticalGraphDataBuilder {
    fn as_ref(&self) -> &StatisticalGraphData {
        &self.inner
    }
}

/// An error message to be shown to the user instead of the graph
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticalGraphError {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The error message

    #[serde(default)]
    error_message: String,
}

impl RObject for StatisticalGraphError {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDStatisticalGraph for StatisticalGraphError {}

impl StatisticalGraphError {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StatisticalGraphErrorBuilder {
        let mut inner = StatisticalGraphError::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StatisticalGraphErrorBuilder { inner }
    }

    pub fn error_message(&self) -> &String {
        &self.error_message
    }
}

#[doc(hidden)]
pub struct StatisticalGraphErrorBuilder {
    inner: StatisticalGraphError,
}

#[deprecated]
pub type RTDStatisticalGraphErrorBuilder = StatisticalGraphErrorBuilder;

impl StatisticalGraphErrorBuilder {
    pub fn build(&self) -> StatisticalGraphError {
        self.inner.clone()
    }

    pub fn error_message<T: AsRef<str>>(&mut self, error_message: T) -> &mut Self {
        self.inner.error_message = error_message.as_ref().to_string();
        self
    }
}

impl AsRef<StatisticalGraphError> for StatisticalGraphError {
    fn as_ref(&self) -> &StatisticalGraphError {
        self
    }
}

impl AsRef<StatisticalGraphError> for StatisticalGraphErrorBuilder {
    fn as_ref(&self) -> &StatisticalGraphError {
        &self.inner
    }
}
