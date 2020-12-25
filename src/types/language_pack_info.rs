
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a language pack
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique language pack identifier
  id: String,
  /// Identifier of a base language pack; may be empty. If a string is missed in the language pack, then it should be fetched from base language pack. Unsupported in custom language packs
  base_language_pack_id: String,
  /// Language name
  name: String,
  /// Name of the language in that language
  native_name: String,
  /// A language code to be used to apply plural forms. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info
  plural_code: String,
  /// True, if the language pack is official
  is_official: bool,
  /// True, if the language pack strings are RTL
  is_rtl: bool,
  /// True, if the language pack is a beta language pack
  is_beta: bool,
  /// True, if the language pack is installed by the current user
  is_installed: bool,
  /// Total number of non-deleted strings from the language pack
  total_string_count: i64,
  /// Total number of translated strings from the language pack
  translated_string_count: i64,
  /// Total number of non-deleted strings from the language pack available locally
  local_string_count: i64,
  /// Link to language translation interface; empty for custom local language packs
  translation_url: String,
  
}

impl RObject for LanguagePackInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl LanguagePackInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLanguagePackInfoBuilder {
    let mut inner = LanguagePackInfo::default();
    inner.td_name = "languagePackInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLanguagePackInfoBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn base_language_pack_id(&self) -> &String { &self.base_language_pack_id }

  pub fn name(&self) -> &String { &self.name }

  pub fn native_name(&self) -> &String { &self.native_name }

  pub fn plural_code(&self) -> &String { &self.plural_code }

  pub fn is_official(&self) -> bool { self.is_official }

  pub fn is_rtl(&self) -> bool { self.is_rtl }

  pub fn is_beta(&self) -> bool { self.is_beta }

  pub fn is_installed(&self) -> bool { self.is_installed }

  pub fn total_string_count(&self) -> i64 { self.total_string_count }

  pub fn translated_string_count(&self) -> i64 { self.translated_string_count }

  pub fn local_string_count(&self) -> i64 { self.local_string_count }

  pub fn translation_url(&self) -> &String { &self.translation_url }

}

#[doc(hidden)]
pub struct RTDLanguagePackInfoBuilder {
  inner: LanguagePackInfo
}

impl RTDLanguagePackInfoBuilder {
  pub fn build(&self) -> LanguagePackInfo { self.inner.clone() }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn base_language_pack_id<T: AsRef<str>>(&mut self, base_language_pack_id: T) -> &mut Self {
    self.inner.base_language_pack_id = base_language_pack_id.as_ref().to_string();
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn native_name<T: AsRef<str>>(&mut self, native_name: T) -> &mut Self {
    self.inner.native_name = native_name.as_ref().to_string();
    self
  }

   
  pub fn plural_code<T: AsRef<str>>(&mut self, plural_code: T) -> &mut Self {
    self.inner.plural_code = plural_code.as_ref().to_string();
    self
  }

   
  pub fn is_official(&mut self, is_official: bool) -> &mut Self {
    self.inner.is_official = is_official;
    self
  }

   
  pub fn is_rtl(&mut self, is_rtl: bool) -> &mut Self {
    self.inner.is_rtl = is_rtl;
    self
  }

   
  pub fn is_beta(&mut self, is_beta: bool) -> &mut Self {
    self.inner.is_beta = is_beta;
    self
  }

   
  pub fn is_installed(&mut self, is_installed: bool) -> &mut Self {
    self.inner.is_installed = is_installed;
    self
  }

   
  pub fn total_string_count(&mut self, total_string_count: i64) -> &mut Self {
    self.inner.total_string_count = total_string_count;
    self
  }

   
  pub fn translated_string_count(&mut self, translated_string_count: i64) -> &mut Self {
    self.inner.translated_string_count = translated_string_count;
    self
  }

   
  pub fn local_string_count(&mut self, local_string_count: i64) -> &mut Self {
    self.inner.local_string_count = local_string_count;
    self
  }

   
  pub fn translation_url<T: AsRef<str>>(&mut self, translation_url: T) -> &mut Self {
    self.inner.translation_url = translation_url.as_ref().to_string();
    self
  }

}

impl AsRef<LanguagePackInfo> for LanguagePackInfo {
  fn as_ref(&self) -> &LanguagePackInfo { self }
}

impl AsRef<LanguagePackInfo> for RTDLanguagePackInfoBuilder {
  fn as_ref(&self) -> &LanguagePackInfo { &self.inner }
}



