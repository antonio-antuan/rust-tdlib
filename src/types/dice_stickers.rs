use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Contains animated stickers which should be used for dice animation rendering
pub trait TDDiceStickers: Debug + RObject {}

/// Contains animated stickers which should be used for dice animation rendering
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum DiceStickers {
    #[doc(hidden)]
    _Default(()),
    /// A regular animated sticker
    Regular(DiceStickersRegular),
    /// Animated stickers to be combined into a slot machine
    SlotMachine(DiceStickersSlotMachine),
}

impl Default for DiceStickers {
    fn default() -> Self {
        DiceStickers::_Default(())
    }
}

impl<'de> Deserialize<'de> for DiceStickers {
    fn deserialize<D>(deserializer: D) -> Result<DiceStickers, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        rtd_enum_deserialize!(
          DiceStickers,
          (diceStickersRegular, Regular);
          (diceStickersSlotMachine, SlotMachine);

        )(deserializer)
    }
}

impl RObject for DiceStickers {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        match self {
            DiceStickers::Regular(t) => t.td_name(),
            DiceStickers::SlotMachine(t) => t.td_name(),

            _ => "-1",
        }
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        match self {
            DiceStickers::Regular(t) => t.extra(),
            DiceStickers::SlotMachine(t) => t.extra(),

            _ => None,
        }
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl DiceStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, DiceStickers::_Default(_))
    }
}

impl AsRef<DiceStickers> for DiceStickers {
    fn as_ref(&self) -> &DiceStickers {
        self
    }
}

/// A regular animated sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiceStickersRegular {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The animated sticker with the dice animation
    sticker: Sticker,
}

impl RObject for DiceStickersRegular {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "diceStickersRegular"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDiceStickers for DiceStickersRegular {}

impl DiceStickersRegular {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDiceStickersRegularBuilder {
        let mut inner = DiceStickersRegular::default();
        inner.td_name = "diceStickersRegular".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDDiceStickersRegularBuilder { inner }
    }

    pub fn sticker(&self) -> &Sticker {
        &self.sticker
    }
}

#[doc(hidden)]
pub struct RTDDiceStickersRegularBuilder {
    inner: DiceStickersRegular,
}

impl RTDDiceStickersRegularBuilder {
    pub fn build(&self) -> DiceStickersRegular {
        self.inner.clone()
    }

    pub fn sticker<T: AsRef<Sticker>>(&mut self, sticker: T) -> &mut Self {
        self.inner.sticker = sticker.as_ref().clone();
        self
    }
}

impl AsRef<DiceStickersRegular> for DiceStickersRegular {
    fn as_ref(&self) -> &DiceStickersRegular {
        self
    }
}

impl AsRef<DiceStickersRegular> for RTDDiceStickersRegularBuilder {
    fn as_ref(&self) -> &DiceStickersRegular {
        &self.inner
    }
}

/// Animated stickers to be combined into a slot machine
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DiceStickersSlotMachine {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The animated sticker with the slot machine background. The background animation must start playing after all reel animations finish
    background: Sticker,
    /// The animated sticker with the lever animation. The lever animation must play once in the initial dice state
    lever: Sticker,
    /// The animated sticker with the left reel
    left_reel: Sticker,
    /// The animated sticker with the center reel
    center_reel: Sticker,
    /// The animated sticker with the right reel
    right_reel: Sticker,
}

impl RObject for DiceStickersSlotMachine {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "diceStickersSlotMachine"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl TDDiceStickers for DiceStickersSlotMachine {}

impl DiceStickersSlotMachine {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDiceStickersSlotMachineBuilder {
        let mut inner = DiceStickersSlotMachine::default();
        inner.td_name = "diceStickersSlotMachine".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDDiceStickersSlotMachineBuilder { inner }
    }

    pub fn background(&self) -> &Sticker {
        &self.background
    }

    pub fn lever(&self) -> &Sticker {
        &self.lever
    }

    pub fn left_reel(&self) -> &Sticker {
        &self.left_reel
    }

    pub fn center_reel(&self) -> &Sticker {
        &self.center_reel
    }

    pub fn right_reel(&self) -> &Sticker {
        &self.right_reel
    }
}

#[doc(hidden)]
pub struct RTDDiceStickersSlotMachineBuilder {
    inner: DiceStickersSlotMachine,
}

impl RTDDiceStickersSlotMachineBuilder {
    pub fn build(&self) -> DiceStickersSlotMachine {
        self.inner.clone()
    }

    pub fn background<T: AsRef<Sticker>>(&mut self, background: T) -> &mut Self {
        self.inner.background = background.as_ref().clone();
        self
    }

    pub fn lever<T: AsRef<Sticker>>(&mut self, lever: T) -> &mut Self {
        self.inner.lever = lever.as_ref().clone();
        self
    }

    pub fn left_reel<T: AsRef<Sticker>>(&mut self, left_reel: T) -> &mut Self {
        self.inner.left_reel = left_reel.as_ref().clone();
        self
    }

    pub fn center_reel<T: AsRef<Sticker>>(&mut self, center_reel: T) -> &mut Self {
        self.inner.center_reel = center_reel.as_ref().clone();
        self
    }

    pub fn right_reel<T: AsRef<Sticker>>(&mut self, right_reel: T) -> &mut Self {
        self.inner.right_reel = right_reel.as_ref().clone();
        self
    }
}

impl AsRef<DiceStickersSlotMachine> for DiceStickersSlotMachine {
    fn as_ref(&self) -> &DiceStickersSlotMachine {
        self
    }
}

impl AsRef<DiceStickersSlotMachine> for RTDDiceStickersSlotMachineBuilder {
    fn as_ref(&self) -> &DiceStickersSlotMachine {
        &self.inner
    }
}
