use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A value with information about its recent changes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticalValue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The current value
    value: f32,
    /// The value for the previous day
    previous_value: f32,
    /// The growth rate of the value, as a percentage
    growth_rate_percentage: f32,
}

impl RObject for StatisticalValue {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "statisticalValue"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl StatisticalValue {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDStatisticalValueBuilder {
        let mut inner = StatisticalValue::default();
        inner.td_name = "statisticalValue".to_string();
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
