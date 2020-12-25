
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents the results of the inline query. Use sendInlineQueryResultMessage to send the result of the query
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResults {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the inline query
  inline_query_id: isize,
  /// The offset for the next request. If empty, there are no more results
  next_offset: String,
  /// Results of the query
  results: Vec<InlineQueryResult>,
  /// If non-empty, this text should be shown on the button, which opens a private chat with the bot and sends the bot a start message with the switch_pm_parameter
  switch_pm_text: String,
  /// Parameter for the bot start message
  switch_pm_parameter: String,
  
}

impl RObject for InlineQueryResults {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResults" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl InlineQueryResults {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDInlineQueryResultsBuilder {
    let mut inner = InlineQueryResults::default();
    inner.td_name = "inlineQueryResults".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDInlineQueryResultsBuilder { inner }
  }

  pub fn inline_query_id(&self) -> isize { self.inline_query_id }

  pub fn next_offset(&self) -> &String { &self.next_offset }

  pub fn results(&self) -> &Vec<InlineQueryResult> { &self.results }

  pub fn switch_pm_text(&self) -> &String { &self.switch_pm_text }

  pub fn switch_pm_parameter(&self) -> &String { &self.switch_pm_parameter }

}

#[doc(hidden)]
pub struct RTDInlineQueryResultsBuilder {
  inner: InlineQueryResults
}

impl RTDInlineQueryResultsBuilder {
  pub fn build(&self) -> InlineQueryResults { self.inner.clone() }

   
  pub fn inline_query_id(&mut self, inline_query_id: isize) -> &mut Self {
    self.inner.inline_query_id = inline_query_id;
    self
  }

   
  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }

   
  pub fn results(&mut self, results: Vec<InlineQueryResult>) -> &mut Self {
    self.inner.results = results;
    self
  }

   
  pub fn switch_pm_text<T: AsRef<str>>(&mut self, switch_pm_text: T) -> &mut Self {
    self.inner.switch_pm_text = switch_pm_text.as_ref().to_string();
    self
  }

   
  pub fn switch_pm_parameter<T: AsRef<str>>(&mut self, switch_pm_parameter: T) -> &mut Self {
    self.inner.switch_pm_parameter = switch_pm_parameter.as_ref().to_string();
    self
  }

}

impl AsRef<InlineQueryResults> for InlineQueryResults {
  fn as_ref(&self) -> &InlineQueryResults { self }
}

impl AsRef<InlineQueryResults> for RTDInlineQueryResultsBuilder {
  fn as_ref(&self) -> &InlineQueryResults { &self.inner }
}



