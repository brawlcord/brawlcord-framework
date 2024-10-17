//! A collection of models and helpers related to Brawlers.

use std::borrow::Cow;
use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};

/// Default amount of ammo a Brawler has.
const fn default_ammo() -> u8 {
    3
}

/// Default attack/ult descriptor: "Damage".
fn default_descriptor() -> Option<String> {
    Some(String::from("Damage"))
}

/// Represents a Brawler.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Brawler {
    /// Name of the Brawler.
    pub name: String,
    /// Health points of the Brawler at level 1.
    pub health: u32,
    /// Speed of the Brawler.
    pub speed: u32,
    /// Rarity of the Brawler.
    pub rarity: Rarity,
    /// Brawler's `Attack` at level 1.
    pub attack: Attack,
    /// Brawler's super ([`Ult`]) at level 1.
    pub ult: Ult,
    /// Brawler's first `Gadget`.
    pub gadget1: Gadget,
    /// Brawler's second `Gadget`.
    pub gadget2: Gadget,
    /// Brawler's first `StarPower`.
    pub sp1: StarPower,
    /// Brawler's second `StarPower`.
    pub sp2: StarPower,
    /// List of all `Skin`s of the Bralwer.
    pub skins: Vec<Skin>,
}

/// Represents the rarity of a Brawler.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
pub enum Rarity {
    /// Represents the Trophy Road rarity.
    #[serde(rename = "Trophy Road")]
    TrophyRoad(u32),
    /// Represents the Rare rarity.
    Rare,
    /// Represents the Super Rare rarity.
    #[serde(rename = "Super Rare")]
    SuperRare,
    /// Represents the Epic rarity.
    Epic,
    /// Represents the Mythic rarity.
    Mythic,
    /// Represents the Legendary rarity.
    Legendary,
    /// Represents the Chromatic rarity.
    Chromatic(ChromaticSeason),
}

/// Represents the season of a Chromatic Bralwer.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
pub enum ChromaticSeason {
    /// The first season. The rarity is equivalent to that of Legendary.
    First,
    /// The second season. The rarity is equivalent to that of Mythic.
    Second,
    /// The third season. The rarity is equivalent to that of Epic.
    Third,
}

impl Rarity {
    /// Returns the `Rarity` right under the current rarity.
    ///
    /// `Chromatic`, `Rare` and `Trophy Road` return `None`.
    pub fn lower(&self) -> Option<Self> {
        match *self {
            Self::Legendary => Some(Self::Mythic),
            Self::Mythic => Some(Self::Epic),
            Self::Epic => Some(Self::SuperRare),
            Self::SuperRare => Some(Self::Rare),
            _ => None,
        }
    }
}

impl Display for Rarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name: Cow<'_, str> = match *self {
            Rarity::TrophyRoad(n) => format!("Trophy Road: {} Trophies", n).into(),
            Rarity::Rare => "Rare".into(),
            Rarity::SuperRare => "Super Rare".into(),
            Rarity::Epic => "Epic".into(),
            Rarity::Mythic => "Mythic".into(),
            Rarity::Legendary => "Legendary".into(),
            Rarity::Chromatic(s) => format!("Chromatic: {} Season", s).into(),
        };

        write!(f, "{}", name)
    }
}

impl Display for ChromaticSeason {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            ChromaticSeason::First => "First",
            ChromaticSeason::Second => "Second",
            ChromaticSeason::Third => "Third",
        })
    }
}

/// Represents the attack of a Brawler.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Attack {
    /// Name of the attack.
    pub name: String,
    /// Damage output of the attack.
    pub damage: u32,
    /// Description of the attack.
    pub description: String,
    /// Maximum attack ammo the Brawler can have.
    ///
    /// Defaults to 3.
    #[serde(default = "default_ammo")]
    pub max_ammo: u8,
    /// Range of the attack.
    pub range: f32,
    /// Reload speed of the attack.
    pub reload: f32,
    /// Number of projectiles in the attack.
    pub projectiles: u32,
    /// Extra text for the attack.
    ///
    /// For example, "Damage per shell" for `Shelly`. It is "Damage" by default.
    #[serde(default = "default_descriptor")]
    pub descriptor: Option<String>,
}

/// Represents the super of a Brawler.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Ult {
    /// Name of the super.
    pub name: String,
    /// Damage output of the super.
    ///
    /// It is `None` for Brawlers with special supers.
    pub damage: Option<u32>,
    /// Description of the super.
    pub description: String,
    /// Range of the super.
    ///
    /// It is `None` for spawners.
    pub range: Option<f32>,
    /// Number of projectiles in the super.
    pub projectiles: u32,
    /// Number of hits required to charge super.
    pub hits_required: u32,
    /// Extra text for the super.
    ///
    /// For example, "Damage per shell" for `Shelly`. It is "Damage" by default.
    #[serde(default = "default_descriptor")]
    pub descriptor: Option<String>,
    /// Spawn of the super.
    ///
    /// It is `None` for most Brawlers.
    pub spawn: Option<Spawn>,
}

/// Repreents a Brawler's spawn.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Spawn {
    /// Name of the spawn.
    pub name: String,
    /// Health of the spawn.
    pub health: u32,
    /// Damage output of the spawn.
    pub damage: u32,
    /// Range of the spawn.
    pub range: f32,
    /// Speed of the spawn.
    pub speed: f32,
}

/// Represents a Brawler's Gadget.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Gadget {
    /// Name of the Gadget.
    pub name: String,
    /// Description of the Gadget.
    pub description: String,
}

/// Represents a Brawler's Star Power.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct StarPower {
    /// Name of the Star Power.
    pub name: String,
    /// Description of the Star Power.
    pub description: String,
}

/// Represents a Brawler skin.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Skin {
    /// Name of the skin.
    pub name: String,
    /// Cost of the skin.
    pub cost: u32,
    /// Type of the skin.
    pub kind: SkinType,
    /// Whether the skin is special or not.
    pub special: bool,
}

/// Represents the type of the skin.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum SkinType {
    /// Skin available for gems.
    Gem,
    /// Skin available for star tokens.
    StarToken,
    /// Skin available for free.
    Free,
}

impl SkinType {
    /// Whether the skin is free.
    pub fn is_free(&self) -> bool {
        matches!(self, Self::Free)
    }

    /// Whether the skin is available for gems.
    pub fn is_gem(&self) -> bool {
        matches!(self, Self::Gem)
    }

    /// Whether the skin is available for star tokens.
    pub fn is_star_token(&self) -> bool {
        matches!(self, Self::StarToken)
    }
}
