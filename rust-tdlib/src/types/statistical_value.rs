use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A value with information about its recent changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticalValue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The current value
    value: f32,
    /// The value for the previous day
    previous_value: f32,
    /// The growth rate of the value, as a percentage
    growth_rate_percentage: f32,
}

impl RObject for StatisticalValue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StatisticalValue {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStatisticalValueBuilder {
        let mut inner = StatisticalValue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDStatisticalValueBuilder { inner }
    }

    pub fn value(&self) -> f32 {
        self.value
    }

    pub fn previous_value(&self) -> f32 {
        self.previous_value
    }

    pub fn growth_rate_percentage(&self) -> f32 {
        self.growth_rate_percentage
    }
}

#[doc(hidden)]
pub struct RTDStatisticalValueBuilder {
    inner: StatisticalValue,
}

impl RTDStatisticalValueBuilder {
    pub fn build(&self) -> StatisticalValue {
        self.inner.clone()
    }

    pub fn value(&mut self, value: f32) -> &mut Self {
        self.inner.value = value;
        self
    }

    pub fn previous_value(&mut self, previous_value: f32) -> &mut Self {
        self.inner.previous_value = previous_value;
        self
    }

    pub fn growth_rate_percentage(&mut self, growth_rate_percentage: f32) -> &mut Self {
        self.inner.growth_rate_percentage = growth_rate_percentage;
        self
    }
}

impl AsRef<StatisticalValue> for StatisticalValue {
    fn as_ref(&self) -> &StatisticalValue {
        self
    }
}

impl AsRef<StatisticalValue> for RTDStatisticalValueBuilder {
    fn as_ref(&self) -> &StatisticalValue {
        &self.inner
    }
}
