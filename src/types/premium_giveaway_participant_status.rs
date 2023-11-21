use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about status of a user in a Telegram Premium giveaway
pub trait TDPremiumGiveawayParticipantStatus: Debug + RObject {}

/// Contains information about status of a user in a Telegram Premium giveaway
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(tag = "@type")]
pub enum PremiumGiveawayParticipantStatus {
    #[doc(hidden)]
    #[default]
    _Default,
    /// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
    #[serde(rename = "premiumGiveawayParticipantStatusAdministrator")]
    Administrator(PremiumGiveawayParticipantStatusAdministrator),
    /// The user can't participate in the giveaway, because they have already been member of the chat
    #[serde(rename = "premiumGiveawayParticipantStatusAlreadyWasMember")]
    AlreadyWasMember(PremiumGiveawayParticipantStatusAlreadyWasMember),
    /// The user can't participate in the giveaway, because they phone number is from a disallowed country
    #[serde(rename = "premiumGiveawayParticipantStatusDisallowedCountry")]
    DisallowedCountry(PremiumGiveawayParticipantStatusDisallowedCountry),
    /// The user is eligible for the giveaway
    #[serde(rename = "premiumGiveawayParticipantStatusEligible")]
    Eligible(PremiumGiveawayParticipantStatusEligible),
    /// The user participates in the giveaway
    #[serde(rename = "premiumGiveawayParticipantStatusParticipating")]
    Participating(PremiumGiveawayParticipantStatusParticipating),
}

impl RObject for PremiumGiveawayParticipantStatus {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PremiumGiveawayParticipantStatus::Administrator(t) => t.extra(),
            PremiumGiveawayParticipantStatus::AlreadyWasMember(t) => t.extra(),
            PremiumGiveawayParticipantStatus::DisallowedCountry(t) => t.extra(),
            PremiumGiveawayParticipantStatus::Eligible(t) => t.extra(),
            PremiumGiveawayParticipantStatus::Participating(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PremiumGiveawayParticipantStatus::Administrator(t) => t.client_id(),
            PremiumGiveawayParticipantStatus::AlreadyWasMember(t) => t.client_id(),
            PremiumGiveawayParticipantStatus::DisallowedCountry(t) => t.client_id(),
            PremiumGiveawayParticipantStatus::Eligible(t) => t.client_id(),
            PremiumGiveawayParticipantStatus::Participating(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PremiumGiveawayParticipantStatus {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PremiumGiveawayParticipantStatus::_Default)
    }
}

impl AsRef<PremiumGiveawayParticipantStatus> for PremiumGiveawayParticipantStatus {
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatus {
        self
    }
}

/// The user can't participate in the giveaway, because they are an administrator in one of the chats that created the giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayParticipantStatusAdministrator {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat administered by the user

    #[serde(default)]
    chat_id: i64,
}

impl RObject for PremiumGiveawayParticipantStatusAdministrator {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayParticipantStatus for PremiumGiveawayParticipantStatusAdministrator {}

impl PremiumGiveawayParticipantStatusAdministrator {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayParticipantStatusAdministratorBuilder {
        let mut inner = PremiumGiveawayParticipantStatusAdministrator::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayParticipantStatusAdministratorBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayParticipantStatusAdministratorBuilder {
    inner: PremiumGiveawayParticipantStatusAdministrator,
}

#[deprecated]
pub type RTDPremiumGiveawayParticipantStatusAdministratorBuilder =
    PremiumGiveawayParticipantStatusAdministratorBuilder;

impl PremiumGiveawayParticipantStatusAdministratorBuilder {
    pub fn build(&self) -> PremiumGiveawayParticipantStatusAdministrator {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusAdministrator>
    for PremiumGiveawayParticipantStatusAdministrator
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusAdministrator {
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusAdministrator>
    for PremiumGiveawayParticipantStatusAdministratorBuilder
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusAdministrator {
        &self.inner
    }
}

/// The user can't participate in the giveaway, because they have already been member of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayParticipantStatusAlreadyWasMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) when the user joined the chat

    #[serde(default)]
    joined_chat_date: i32,
}

impl RObject for PremiumGiveawayParticipantStatusAlreadyWasMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayParticipantStatus for PremiumGiveawayParticipantStatusAlreadyWasMember {}

impl PremiumGiveawayParticipantStatusAlreadyWasMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayParticipantStatusAlreadyWasMemberBuilder {
        let mut inner = PremiumGiveawayParticipantStatusAlreadyWasMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayParticipantStatusAlreadyWasMemberBuilder { inner }
    }

    pub fn joined_chat_date(&self) -> i32 {
        self.joined_chat_date
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayParticipantStatusAlreadyWasMemberBuilder {
    inner: PremiumGiveawayParticipantStatusAlreadyWasMember,
}

#[deprecated]
pub type RTDPremiumGiveawayParticipantStatusAlreadyWasMemberBuilder =
    PremiumGiveawayParticipantStatusAlreadyWasMemberBuilder;

impl PremiumGiveawayParticipantStatusAlreadyWasMemberBuilder {
    pub fn build(&self) -> PremiumGiveawayParticipantStatusAlreadyWasMember {
        self.inner.clone()
    }

    pub fn joined_chat_date(&mut self, joined_chat_date: i32) -> &mut Self {
        self.inner.joined_chat_date = joined_chat_date;
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusAlreadyWasMember>
    for PremiumGiveawayParticipantStatusAlreadyWasMember
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusAlreadyWasMember {
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusAlreadyWasMember>
    for PremiumGiveawayParticipantStatusAlreadyWasMemberBuilder
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusAlreadyWasMember {
        &self.inner
    }
}

/// The user can't participate in the giveaway, because they phone number is from a disallowed country
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayParticipantStatusDisallowedCountry {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A two-letter ISO 3166-1 alpha-2 country code of the user's country

    #[serde(default)]
    user_country_code: String,
}

impl RObject for PremiumGiveawayParticipantStatusDisallowedCountry {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayParticipantStatus for PremiumGiveawayParticipantStatusDisallowedCountry {}

impl PremiumGiveawayParticipantStatusDisallowedCountry {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayParticipantStatusDisallowedCountryBuilder {
        let mut inner = PremiumGiveawayParticipantStatusDisallowedCountry::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayParticipantStatusDisallowedCountryBuilder { inner }
    }

    pub fn user_country_code(&self) -> &String {
        &self.user_country_code
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayParticipantStatusDisallowedCountryBuilder {
    inner: PremiumGiveawayParticipantStatusDisallowedCountry,
}

#[deprecated]
pub type RTDPremiumGiveawayParticipantStatusDisallowedCountryBuilder =
    PremiumGiveawayParticipantStatusDisallowedCountryBuilder;

impl PremiumGiveawayParticipantStatusDisallowedCountryBuilder {
    pub fn build(&self) -> PremiumGiveawayParticipantStatusDisallowedCountry {
        self.inner.clone()
    }

    pub fn user_country_code<T: AsRef<str>>(&mut self, user_country_code: T) -> &mut Self {
        self.inner.user_country_code = user_country_code.as_ref().to_string();
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusDisallowedCountry>
    for PremiumGiveawayParticipantStatusDisallowedCountry
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusDisallowedCountry {
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusDisallowedCountry>
    for PremiumGiveawayParticipantStatusDisallowedCountryBuilder
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusDisallowedCountry {
        &self.inner
    }
}

/// The user is eligible for the giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayParticipantStatusEligible {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumGiveawayParticipantStatusEligible {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayParticipantStatus for PremiumGiveawayParticipantStatusEligible {}

impl PremiumGiveawayParticipantStatusEligible {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayParticipantStatusEligibleBuilder {
        let mut inner = PremiumGiveawayParticipantStatusEligible::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayParticipantStatusEligibleBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayParticipantStatusEligibleBuilder {
    inner: PremiumGiveawayParticipantStatusEligible,
}

#[deprecated]
pub type RTDPremiumGiveawayParticipantStatusEligibleBuilder =
    PremiumGiveawayParticipantStatusEligibleBuilder;

impl PremiumGiveawayParticipantStatusEligibleBuilder {
    pub fn build(&self) -> PremiumGiveawayParticipantStatusEligible {
        self.inner.clone()
    }
}

impl AsRef<PremiumGiveawayParticipantStatusEligible> for PremiumGiveawayParticipantStatusEligible {
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusEligible {
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusEligible>
    for PremiumGiveawayParticipantStatusEligibleBuilder
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusEligible {
        &self.inner
    }
}

/// The user participates in the giveaway
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumGiveawayParticipantStatusParticipating {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for PremiumGiveawayParticipantStatusParticipating {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPremiumGiveawayParticipantStatus for PremiumGiveawayParticipantStatusParticipating {}

impl PremiumGiveawayParticipantStatusParticipating {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PremiumGiveawayParticipantStatusParticipatingBuilder {
        let mut inner = PremiumGiveawayParticipantStatusParticipating::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PremiumGiveawayParticipantStatusParticipatingBuilder { inner }
    }
}

#[doc(hidden)]
pub struct PremiumGiveawayParticipantStatusParticipatingBuilder {
    inner: PremiumGiveawayParticipantStatusParticipating,
}

#[deprecated]
pub type RTDPremiumGiveawayParticipantStatusParticipatingBuilder =
    PremiumGiveawayParticipantStatusParticipatingBuilder;

impl PremiumGiveawayParticipantStatusParticipatingBuilder {
    pub fn build(&self) -> PremiumGiveawayParticipantStatusParticipating {
        self.inner.clone()
    }
}

impl AsRef<PremiumGiveawayParticipantStatusParticipating>
    for PremiumGiveawayParticipantStatusParticipating
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusParticipating {
        self
    }
}

impl AsRef<PremiumGiveawayParticipantStatusParticipating>
    for PremiumGiveawayParticipantStatusParticipatingBuilder
{
    fn as_ref(&self) -> &PremiumGiveawayParticipantStatusParticipating {
        &self.inner
    }
}
