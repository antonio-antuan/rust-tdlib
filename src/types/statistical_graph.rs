use crate::errors::*;
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
    #[serde(rename(serialize = "getStatisticalGraph", deserialize = "getStatisticalGraph"))]
    GetStatisticalGraph(GetStatisticalGraph),
    /// The graph data to be asynchronously loaded through getStatisticalGraph
    #[serde(rename(
        serialize = "statisticalGraphAsync",
        deserialize = "statisticalGraphAsync"
    ))]
    Async(StatisticalGraphAsync),
    /// A graph data
    #[serde(rename(
        serialize = "statisticalGraphData",
        deserialize = "statisticalGraphData"
    ))]
    Data(StatisticalGraphData),
    /// An error message to be shown to the user instead of the graph
    #[serde(rename(
        serialize = "statisticalGraphError",
        deserialize = "statisticalGraphError"
    ))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStatisticalGraphAsyncBuilder {
        let mut inner = StatisticalGraphAsync::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStatisticalGraphAsyncBuilder { inner }
    }

    pub fn token(&self) -> &String {
        &self.token
    }
}

#[doc(hidden)]
pub struct RTDStatisticalGraphAsyncBuilder {
    inner: StatisticalGraphAsync,
}

impl RTDStatisticalGraphAsyncBuilder {
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

impl AsRef<StatisticalGraphAsync> for RTDStatisticalGraphAsyncBuilder {
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
    json_data: String,
    /// If non-empty, a token which can be used to receive a zoomed in graph
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStatisticalGraphDataBuilder {
        let mut inner = StatisticalGraphData::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStatisticalGraphDataBuilder { inner }
    }

    pub fn json_data(&self) -> &String {
        &self.json_data
    }

    pub fn zoom_token(&self) -> &String {
        &self.zoom_token
    }
}

#[doc(hidden)]
pub struct RTDStatisticalGraphDataBuilder {
    inner: StatisticalGraphData,
}

impl RTDStatisticalGraphDataBuilder {
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

impl AsRef<StatisticalGraphData> for RTDStatisticalGraphDataBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStatisticalGraphErrorBuilder {
        let mut inner = StatisticalGraphError::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStatisticalGraphErrorBuilder { inner }
    }

    pub fn error_message(&self) -> &String {
        &self.error_message
    }
}

#[doc(hidden)]
pub struct RTDStatisticalGraphErrorBuilder {
    inner: StatisticalGraphError,
}

impl RTDStatisticalGraphErrorBuilder {
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

impl AsRef<StatisticalGraphError> for RTDStatisticalGraphErrorBuilder {
    fn as_ref(&self) -> &StatisticalGraphError {
        &self.inner
    }
}
