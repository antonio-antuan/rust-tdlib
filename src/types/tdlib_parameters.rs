
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains parameters for TDLib initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// If set to true, the Telegram test environment will be used instead of the production environment
  use_test_dc: bool,
  /// The path to the directory for the persistent database; if empty, the current working directory will be used
  database_directory: String,
  /// The path to the directory for storing files; if empty, database_directory will be used
  files_directory: String,
  /// If set to true, information about downloaded and uploaded files will be saved between application restarts
  use_file_database: bool,
  /// If set to true, the library will maintain a cache of users, basic groups, supergroups, channels and secret chats. Implies use_file_database
  use_chat_info_database: bool,
  /// If set to true, the library will maintain a cache of chats and messages. Implies use_chat_info_database
  use_message_database: bool,
  /// If set to true, support for secret chats will be enabled
  use_secret_chats: bool,
  /// Application identifier for Telegram API access, which can be obtained at https://my.telegram.org
  api_id: i64,
  /// Application identifier hash for Telegram API access, which can be obtained at https://my.telegram.org
  api_hash: String,
  /// IETF language tag of the user's operating system language; must be non-empty
  system_language_code: String,
  /// Model of the device the application is being run on; must be non-empty
  device_model: String,
  /// Version of the operating system the application is being run on; must be non-empty
  system_version: String,
  /// Application version; must be non-empty
  application_version: String,
  /// If set to true, old files will automatically be deleted
  enable_storage_optimizer: bool,
  /// If set to true, original file names will be ignored. Otherwise, downloaded files will be saved under names as close as possible to the original name
  ignore_file_names: bool,
  
}

impl RObject for TdlibParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tdlibParameters" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl TdlibParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTdlibParametersBuilder {
    let mut inner = TdlibParameters::default();
    inner.td_name = "tdlibParameters".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTdlibParametersBuilder { inner }
  }

  pub fn use_test_dc(&self) -> bool { self.use_test_dc }

  pub fn database_directory(&self) -> &String { &self.database_directory }

  pub fn files_directory(&self) -> &String { &self.files_directory }

  pub fn use_file_database(&self) -> bool { self.use_file_database }

  pub fn use_chat_info_database(&self) -> bool { self.use_chat_info_database }

  pub fn use_message_database(&self) -> bool { self.use_message_database }

  pub fn use_secret_chats(&self) -> bool { self.use_secret_chats }

  pub fn api_id(&self) -> i64 { self.api_id }

  pub fn api_hash(&self) -> &String { &self.api_hash }

  pub fn system_language_code(&self) -> &String { &self.system_language_code }

  pub fn device_model(&self) -> &String { &self.device_model }

  pub fn system_version(&self) -> &String { &self.system_version }

  pub fn application_version(&self) -> &String { &self.application_version }

  pub fn enable_storage_optimizer(&self) -> bool { self.enable_storage_optimizer }

  pub fn ignore_file_names(&self) -> bool { self.ignore_file_names }

}

#[doc(hidden)]
pub struct RTDTdlibParametersBuilder {
  inner: TdlibParameters
}

impl RTDTdlibParametersBuilder {
  pub fn build(&self) -> TdlibParameters { self.inner.clone() }

   
  pub fn use_test_dc(&mut self, use_test_dc: bool) -> &mut Self {
    self.inner.use_test_dc = use_test_dc;
    self
  }

   
  pub fn database_directory<T: AsRef<str>>(&mut self, database_directory: T) -> &mut Self {
    self.inner.database_directory = database_directory.as_ref().to_string();
    self
  }

   
  pub fn files_directory<T: AsRef<str>>(&mut self, files_directory: T) -> &mut Self {
    self.inner.files_directory = files_directory.as_ref().to_string();
    self
  }

   
  pub fn use_file_database(&mut self, use_file_database: bool) -> &mut Self {
    self.inner.use_file_database = use_file_database;
    self
  }

   
  pub fn use_chat_info_database(&mut self, use_chat_info_database: bool) -> &mut Self {
    self.inner.use_chat_info_database = use_chat_info_database;
    self
  }

   
  pub fn use_message_database(&mut self, use_message_database: bool) -> &mut Self {
    self.inner.use_message_database = use_message_database;
    self
  }

   
  pub fn use_secret_chats(&mut self, use_secret_chats: bool) -> &mut Self {
    self.inner.use_secret_chats = use_secret_chats;
    self
  }

   
  pub fn api_id(&mut self, api_id: i64) -> &mut Self {
    self.inner.api_id = api_id;
    self
  }

   
  pub fn api_hash<T: AsRef<str>>(&mut self, api_hash: T) -> &mut Self {
    self.inner.api_hash = api_hash.as_ref().to_string();
    self
  }

   
  pub fn system_language_code<T: AsRef<str>>(&mut self, system_language_code: T) -> &mut Self {
    self.inner.system_language_code = system_language_code.as_ref().to_string();
    self
  }

   
  pub fn device_model<T: AsRef<str>>(&mut self, device_model: T) -> &mut Self {
    self.inner.device_model = device_model.as_ref().to_string();
    self
  }

   
  pub fn system_version<T: AsRef<str>>(&mut self, system_version: T) -> &mut Self {
    self.inner.system_version = system_version.as_ref().to_string();
    self
  }

   
  pub fn application_version<T: AsRef<str>>(&mut self, application_version: T) -> &mut Self {
    self.inner.application_version = application_version.as_ref().to_string();
    self
  }

   
  pub fn enable_storage_optimizer(&mut self, enable_storage_optimizer: bool) -> &mut Self {
    self.inner.enable_storage_optimizer = enable_storage_optimizer;
    self
  }

   
  pub fn ignore_file_names(&mut self, ignore_file_names: bool) -> &mut Self {
    self.inner.ignore_file_names = ignore_file_names;
    self
  }

}

impl AsRef<TdlibParameters> for TdlibParameters {
  fn as_ref(&self) -> &TdlibParameters { self }
}

impl AsRef<TdlibParameters> for RTDTdlibParametersBuilder {
  fn as_ref(&self) -> &TdlibParameters { &self.inner }
}



