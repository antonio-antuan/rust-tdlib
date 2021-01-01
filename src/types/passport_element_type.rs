use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains the type of a Telegram Passport element
pub trait TDPassportElementType: Debug + RObject {}

/// Contains the type of a Telegram Passport element
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PassportElementType {
    #[doc(hidden)]
    _Default(()),
    /// A Telegram Passport element containing the user's address
    Address(PassportElementTypeAddress),
    /// A Telegram Passport element containing the user's bank statement
    BankStatement(PassportElementTypeBankStatement),
    /// A Telegram Passport element containing the user's driver license
    DriverLicense(PassportElementTypeDriverLicense),
    /// A Telegram Passport element containing the user's email address
    EmailAddress(PassportElementTypeEmailAddress),
    /// A Telegram Passport element containing the user's identity card
    IdentityCard(PassportElementTypeIdentityCard),
    /// A Telegram Passport element containing the user's internal passport
    InternalPassport(PassportElementTypeInternalPassport),
    /// A Telegram Passport element containing the user's passport
    Passport(PassportElementTypePassport),
    /// A Telegram Passport element containing the registration page of the user's passport
    PassportRegistration(PassportElementTypePassportRegistration),
    /// A Telegram Passport element containing the user's personal details
    PersonalDetails(PassportElementTypePersonalDetails),
    /// A Telegram Passport element containing the user's phone number
    PhoneNumber(PassportElementTypePhoneNumber),
    /// A Telegram Passport element containing the user's rental agreement
    RentalAgreement(PassportElementTypeRentalAgreement),
    /// A Telegram Passport element containing the user's temporary registration
    TemporaryRegistration(PassportElementTypeTemporaryRegistration),
    /// A Telegram Passport element containing the user's utility bill
    UtilityBill(PassportElementTypeUtilityBill),
}

impl Default for PassportElementType {
    fn default() -> Self {
        PassportElementType::_Default(())
    }
}

impl<'de> Deserialize<'de> for PassportElementType {
    fn deserialize<D>(deserializer: D) -> Result<PassportElementType, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          PassportElementType,
          (passportElementTypeAddress, Address);
          (passportElementTypeBankStatement, BankStatement);
          (passportElementTypeDriverLicense, DriverLicense);
          (passportElementTypeEmailAddress, EmailAddress);
          (passportElementTypeIdentityCard, IdentityCard);
          (passportElementTypeInternalPassport, InternalPassport);
          (passportElementTypePassport, Passport);
          (passportElementTypePassportRegistration, PassportRegistration);
          (passportElementTypePersonalDetails, PersonalDetails);
          (passportElementTypePhoneNumber, PhoneNumber);
          (passportElementTypeRentalAgreement, RentalAgreement);
          (passportElementTypeTemporaryRegistration, TemporaryRegistration);
          (passportElementTypeUtilityBill, UtilityBill);

        )(deserializer)
    }
}

impl RObject for PassportElementType {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            PassportElementType::Address(t) => t.td_name(),
            PassportElementType::BankStatement(t) => t.td_name(),
            PassportElementType::DriverLicense(t) => t.td_name(),
            PassportElementType::EmailAddress(t) => t.td_name(),
            PassportElementType::IdentityCard(t) => t.td_name(),
            PassportElementType::InternalPassport(t) => t.td_name(),
            PassportElementType::Passport(t) => t.td_name(),
            PassportElementType::PassportRegistration(t) => t.td_name(),
            PassportElementType::PersonalDetails(t) => t.td_name(),
            PassportElementType::PhoneNumber(t) => t.td_name(),
            PassportElementType::RentalAgreement(t) => t.td_name(),
            PassportElementType::TemporaryRegistration(t) => t.td_name(),
            PassportElementType::UtilityBill(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            PassportElementType::Address(t) => t.extra(),
            PassportElementType::BankStatement(t) => t.extra(),
            PassportElementType::DriverLicense(t) => t.extra(),
            PassportElementType::EmailAddress(t) => t.extra(),
            PassportElementType::IdentityCard(t) => t.extra(),
            PassportElementType::InternalPassport(t) => t.extra(),
            PassportElementType::Passport(t) => t.extra(),
            PassportElementType::PassportRegistration(t) => t.extra(),
            PassportElementType::PersonalDetails(t) => t.extra(),
            PassportElementType::PhoneNumber(t) => t.extra(),
            PassportElementType::RentalAgreement(t) => t.extra(),
            PassportElementType::TemporaryRegistration(t) => t.extra(),
            PassportElementType::UtilityBill(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl PassportElementType {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PassportElementType::_Default(_))
    }
}

impl AsRef<PassportElementType> for PassportElementType {
    fn as_ref(&self) -> &PassportElementType {
        self
    }
}

/// A Telegram Passport element containing the user's address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeAddress {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeAddress"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeAddress {}

impl PassportElementTypeAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeAddressBuilder {
        let mut inner = PassportElementTypeAddress::default();
        inner.td_name = "passportElementTypeAddress".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeAddressBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeAddressBuilder {
    inner: PassportElementTypeAddress,
}

impl RTDPassportElementTypeAddressBuilder {
    pub fn build(&self) -> PassportElementTypeAddress {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeAddress> for PassportElementTypeAddress {
    fn as_ref(&self) -> &PassportElementTypeAddress {
        self
    }
}

impl AsRef<PassportElementTypeAddress> for RTDPassportElementTypeAddressBuilder {
    fn as_ref(&self) -> &PassportElementTypeAddress {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's bank statement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeBankStatement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeBankStatement {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeBankStatement"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeBankStatement {}

impl PassportElementTypeBankStatement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeBankStatementBuilder {
        let mut inner = PassportElementTypeBankStatement::default();
        inner.td_name = "passportElementTypeBankStatement".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeBankStatementBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeBankStatementBuilder {
    inner: PassportElementTypeBankStatement,
}

impl RTDPassportElementTypeBankStatementBuilder {
    pub fn build(&self) -> PassportElementTypeBankStatement {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeBankStatement> for PassportElementTypeBankStatement {
    fn as_ref(&self) -> &PassportElementTypeBankStatement {
        self
    }
}

impl AsRef<PassportElementTypeBankStatement> for RTDPassportElementTypeBankStatementBuilder {
    fn as_ref(&self) -> &PassportElementTypeBankStatement {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's driver license
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeDriverLicense {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeDriverLicense {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeDriverLicense"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeDriverLicense {}

impl PassportElementTypeDriverLicense {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeDriverLicenseBuilder {
        let mut inner = PassportElementTypeDriverLicense::default();
        inner.td_name = "passportElementTypeDriverLicense".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeDriverLicenseBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeDriverLicenseBuilder {
    inner: PassportElementTypeDriverLicense,
}

impl RTDPassportElementTypeDriverLicenseBuilder {
    pub fn build(&self) -> PassportElementTypeDriverLicense {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeDriverLicense> for PassportElementTypeDriverLicense {
    fn as_ref(&self) -> &PassportElementTypeDriverLicense {
        self
    }
}

impl AsRef<PassportElementTypeDriverLicense> for RTDPassportElementTypeDriverLicenseBuilder {
    fn as_ref(&self) -> &PassportElementTypeDriverLicense {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeEmailAddress {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeEmailAddress"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeEmailAddress {}

impl PassportElementTypeEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeEmailAddressBuilder {
        let mut inner = PassportElementTypeEmailAddress::default();
        inner.td_name = "passportElementTypeEmailAddress".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeEmailAddressBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeEmailAddressBuilder {
    inner: PassportElementTypeEmailAddress,
}

impl RTDPassportElementTypeEmailAddressBuilder {
    pub fn build(&self) -> PassportElementTypeEmailAddress {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeEmailAddress> for PassportElementTypeEmailAddress {
    fn as_ref(&self) -> &PassportElementTypeEmailAddress {
        self
    }
}

impl AsRef<PassportElementTypeEmailAddress> for RTDPassportElementTypeEmailAddressBuilder {
    fn as_ref(&self) -> &PassportElementTypeEmailAddress {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's identity card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeIdentityCard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeIdentityCard {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeIdentityCard"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeIdentityCard {}

impl PassportElementTypeIdentityCard {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeIdentityCardBuilder {
        let mut inner = PassportElementTypeIdentityCard::default();
        inner.td_name = "passportElementTypeIdentityCard".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeIdentityCardBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeIdentityCardBuilder {
    inner: PassportElementTypeIdentityCard,
}

impl RTDPassportElementTypeIdentityCardBuilder {
    pub fn build(&self) -> PassportElementTypeIdentityCard {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeIdentityCard> for PassportElementTypeIdentityCard {
    fn as_ref(&self) -> &PassportElementTypeIdentityCard {
        self
    }
}

impl AsRef<PassportElementTypeIdentityCard> for RTDPassportElementTypeIdentityCardBuilder {
    fn as_ref(&self) -> &PassportElementTypeIdentityCard {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's internal passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeInternalPassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeInternalPassport {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeInternalPassport"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeInternalPassport {}

impl PassportElementTypeInternalPassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeInternalPassportBuilder {
        let mut inner = PassportElementTypeInternalPassport::default();
        inner.td_name = "passportElementTypeInternalPassport".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeInternalPassportBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeInternalPassportBuilder {
    inner: PassportElementTypeInternalPassport,
}

impl RTDPassportElementTypeInternalPassportBuilder {
    pub fn build(&self) -> PassportElementTypeInternalPassport {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeInternalPassport> for PassportElementTypeInternalPassport {
    fn as_ref(&self) -> &PassportElementTypeInternalPassport {
        self
    }
}

impl AsRef<PassportElementTypeInternalPassport> for RTDPassportElementTypeInternalPassportBuilder {
    fn as_ref(&self) -> &PassportElementTypeInternalPassport {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypePassport {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypePassport"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypePassport {}

impl PassportElementTypePassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePassportBuilder {
        let mut inner = PassportElementTypePassport::default();
        inner.td_name = "passportElementTypePassport".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypePassportBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePassportBuilder {
    inner: PassportElementTypePassport,
}

impl RTDPassportElementTypePassportBuilder {
    pub fn build(&self) -> PassportElementTypePassport {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePassport> for PassportElementTypePassport {
    fn as_ref(&self) -> &PassportElementTypePassport {
        self
    }
}

impl AsRef<PassportElementTypePassport> for RTDPassportElementTypePassportBuilder {
    fn as_ref(&self) -> &PassportElementTypePassport {
        &self.inner
    }
}

/// A Telegram Passport element containing the registration page of the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePassportRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypePassportRegistration {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypePassportRegistration"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypePassportRegistration {}

impl PassportElementTypePassportRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePassportRegistrationBuilder {
        let mut inner = PassportElementTypePassportRegistration::default();
        inner.td_name = "passportElementTypePassportRegistration".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypePassportRegistrationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePassportRegistrationBuilder {
    inner: PassportElementTypePassportRegistration,
}

impl RTDPassportElementTypePassportRegistrationBuilder {
    pub fn build(&self) -> PassportElementTypePassportRegistration {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePassportRegistration> for PassportElementTypePassportRegistration {
    fn as_ref(&self) -> &PassportElementTypePassportRegistration {
        self
    }
}

impl AsRef<PassportElementTypePassportRegistration>
    for RTDPassportElementTypePassportRegistrationBuilder
{
    fn as_ref(&self) -> &PassportElementTypePassportRegistration {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePersonalDetails {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypePersonalDetails {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypePersonalDetails"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypePersonalDetails {}

impl PassportElementTypePersonalDetails {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePersonalDetailsBuilder {
        let mut inner = PassportElementTypePersonalDetails::default();
        inner.td_name = "passportElementTypePersonalDetails".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypePersonalDetailsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePersonalDetailsBuilder {
    inner: PassportElementTypePersonalDetails,
}

impl RTDPassportElementTypePersonalDetailsBuilder {
    pub fn build(&self) -> PassportElementTypePersonalDetails {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePersonalDetails> for PassportElementTypePersonalDetails {
    fn as_ref(&self) -> &PassportElementTypePersonalDetails {
        self
    }
}

impl AsRef<PassportElementTypePersonalDetails> for RTDPassportElementTypePersonalDetailsBuilder {
    fn as_ref(&self) -> &PassportElementTypePersonalDetails {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypePhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypePhoneNumber {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypePhoneNumber"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypePhoneNumber {}

impl PassportElementTypePhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypePhoneNumberBuilder {
        let mut inner = PassportElementTypePhoneNumber::default();
        inner.td_name = "passportElementTypePhoneNumber".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypePhoneNumberBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypePhoneNumberBuilder {
    inner: PassportElementTypePhoneNumber,
}

impl RTDPassportElementTypePhoneNumberBuilder {
    pub fn build(&self) -> PassportElementTypePhoneNumber {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypePhoneNumber> for PassportElementTypePhoneNumber {
    fn as_ref(&self) -> &PassportElementTypePhoneNumber {
        self
    }
}

impl AsRef<PassportElementTypePhoneNumber> for RTDPassportElementTypePhoneNumberBuilder {
    fn as_ref(&self) -> &PassportElementTypePhoneNumber {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's rental agreement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeRentalAgreement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeRentalAgreement {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeRentalAgreement"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeRentalAgreement {}

impl PassportElementTypeRentalAgreement {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeRentalAgreementBuilder {
        let mut inner = PassportElementTypeRentalAgreement::default();
        inner.td_name = "passportElementTypeRentalAgreement".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeRentalAgreementBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeRentalAgreementBuilder {
    inner: PassportElementTypeRentalAgreement,
}

impl RTDPassportElementTypeRentalAgreementBuilder {
    pub fn build(&self) -> PassportElementTypeRentalAgreement {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeRentalAgreement> for PassportElementTypeRentalAgreement {
    fn as_ref(&self) -> &PassportElementTypeRentalAgreement {
        self
    }
}

impl AsRef<PassportElementTypeRentalAgreement> for RTDPassportElementTypeRentalAgreementBuilder {
    fn as_ref(&self) -> &PassportElementTypeRentalAgreement {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's temporary registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeTemporaryRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeTemporaryRegistration {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeTemporaryRegistration"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeTemporaryRegistration {}

impl PassportElementTypeTemporaryRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeTemporaryRegistrationBuilder {
        let mut inner = PassportElementTypeTemporaryRegistration::default();
        inner.td_name = "passportElementTypeTemporaryRegistration".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeTemporaryRegistrationBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeTemporaryRegistrationBuilder {
    inner: PassportElementTypeTemporaryRegistration,
}

impl RTDPassportElementTypeTemporaryRegistrationBuilder {
    pub fn build(&self) -> PassportElementTypeTemporaryRegistration {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeTemporaryRegistration> for PassportElementTypeTemporaryRegistration {
    fn as_ref(&self) -> &PassportElementTypeTemporaryRegistration {
        self
    }
}

impl AsRef<PassportElementTypeTemporaryRegistration>
    for RTDPassportElementTypeTemporaryRegistrationBuilder
{
    fn as_ref(&self) -> &PassportElementTypeTemporaryRegistration {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's utility bill
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTypeUtilityBill {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
}

impl RObject for PassportElementTypeUtilityBill {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "passportElementTypeUtilityBill"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDPassportElementType for PassportElementTypeUtilityBill {}

impl PassportElementTypeUtilityBill {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTypeUtilityBillBuilder {
        let mut inner = PassportElementTypeUtilityBill::default();
        inner.td_name = "passportElementTypeUtilityBill".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDPassportElementTypeUtilityBillBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTypeUtilityBillBuilder {
    inner: PassportElementTypeUtilityBill,
}

impl RTDPassportElementTypeUtilityBillBuilder {
    pub fn build(&self) -> PassportElementTypeUtilityBill {
        self.inner.clone()
    }
}

impl AsRef<PassportElementTypeUtilityBill> for PassportElementTypeUtilityBill {
    fn as_ref(&self) -> &PassportElementTypeUtilityBill {
        self
    }
}

impl AsRef<PassportElementTypeUtilityBill> for RTDPassportElementTypeUtilityBillBuilder {
    fn as_ref(&self) -> &PassportElementTypeUtilityBill {
        &self.inner
    }
}
