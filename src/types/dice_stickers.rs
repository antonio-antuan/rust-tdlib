use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains animated stickers which should be used for dice animation rendering
pub trait TDDiceStickers: Debug + RObject {}

/// Contains animated stickers which should be used for dice animation rendering
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum DiceStickers {
    #[doc(hidden)]
    _Default,
    /// A regular animated sticker
    #[serde(rename(serialize = "diceStickersRegular", deserialize = "diceStickersRegular"))]
    Regular(DiceStickersRegular),
    /// Animated stickers to be combined into a slot machine
    #[serde(rename(
        serialize = "diceStickersSlotMachine",
        deserialize = "diceStickersSlotMachine"
    ))]
    SlotMachine(DiceStickersSlotMachine),
}

impl Default for DiceStickers {
    fn default() -> Self {
        DiceStickers::_Default
    }
}

impl RObject for DiceStickers {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            DiceStickers::Regular(t) => t.extra(),
            DiceStickers::SlotMachine(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            DiceStickers::Regular(t) => t.client_id(),
            DiceStickers::SlotMachine(t) => t.client_id(),

            _ => None,
        }
    }
}

impl DiceStickers {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, DiceStickers::_Default)
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The animated sticker with the dice animation
    sticker: Sticker,
}

impl RObject for DiceStickersRegular {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDiceStickers for DiceStickersRegular {}

impl DiceStickersRegular {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDiceStickersRegularBuilder {
        let mut inner = DiceStickersRegular::default();
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
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
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
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDDiceStickers for DiceStickersSlotMachine {}

impl DiceStickersSlotMachine {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDiceStickersSlotMachineBuilder {
        let mut inner = DiceStickersSlotMachine::default();
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
