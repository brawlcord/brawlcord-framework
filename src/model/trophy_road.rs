//! A collection of models and helpers related to the Trophy Raod.

use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::resource::bs_box::BoxType;

/// Represents the Trophy Road.
///
/// It holds every reward available on the Trophy Road.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TrophyRoad(pub Vec<TrophyRoadReward>);

impl TrophyRoad {
    /// Creates a new [`TrophyRoad`].
    pub fn new(rewards: Vec<TrophyRoadReward>) -> Self {
        Self(rewards)
    }

    /// Checks if a player with given trophies can unlock the reward at given index.
    ///
    /// Returns false if the index is out of bounds.
    ///
    /// This only takes the trophy requirements into account and does not
    /// consider whether the player has collected the reward before or not.
    pub fn can_collect(&self, index: usize, trophies: u32) -> bool {
        self.0.get(index).map_or(false, |r| r.can_collect(trophies))
    }

    /// Returns an iterator of Trophy Road rewards that can be
    /// collected by a player with given trophies.
    pub fn collectables(&self, trophies: u32) -> impl Iterator<Item = &TrophyRoadReward> {
        self.0.iter().filter(move |r| r.can_collect(trophies))
    }
}

/// Represents a reward on the Trophy Road.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct TrophyRoadReward {
    /// The trophies at which the reward is gained.
    pub trophies: u32,
    /// The kind of the reward.
    pub kind: TrophyRoadRewardKind,
    /// The number of rewards given.
    pub count: u32,
    /// Extra data associated with the reward.
    pub extra_data: String,
}

impl TrophyRoadReward {
    /// Creates a new [`TrophyRoadReward`].
    pub fn new(trophies: u32, kind: TrophyRoadRewardKind, count: u32, extra_data: String) -> Self {
        Self { trophies, kind, count, extra_data }
    }

    /// Checks if a player with given trophies can collect the reward.
    ///
    /// This only takes the trophy requirements into account and does not
    /// consider whether the player has collected the reward before or not.
    pub fn can_collect(&self, trophies: u32) -> bool {
        trophies >= self.trophies
    }
}

/// Represents the kind of Trophy Road reward.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum TrophyRoadRewardKind {
    /// Represents gold as reward.
    Gold,
    /// Represents a Brawler as reward.
    Brawler,
    /// Represents a Brawl Box as reward.
    BsBox(BoxType),
    /// Represents token doublers as reward.
    TokenDoublers,
    /// Represents power points as reward.
    PowerPoints,
    /// Represents a game mode as reward.
    GameMode,
}

impl TrophyRoadRewardKind {
    /// The code number used for gold.
    pub const GOLD: u8 = 1;
    /// The code number used for Brawler.
    pub const BRAWLER: u8 = 3;
    /// The code number used for Brawl Box.
    pub const BRAWL_BOX: u8 = 6;
    /// The code number used for token doublers.
    pub const TOKEN_DOUBLERS: u8 = 9;
    /// The code number used for Mega Box.
    pub const MEGA_BOX: u8 = 10;
    /// The code number used for power points.
    pub const POWER_POINTS: u8 = 12;
    /// The code number used for game mode.
    pub const GAME_MODE: u8 = 13;
    /// The code number used for Big Box.
    pub const BIG_BOX: u8 = 14;

    /// Creates a new [`TrophyRoadRewardKind`] from its code.
    ///
    /// Note: `None` is returned for unrecognised codes.
    /// Valid codes: 1, 3, 6, 9, 10, 12, 13, 14.
    pub fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            Self::GOLD => Self::Gold,
            Self::BRAWLER => Self::Brawler,
            Self::BRAWL_BOX => Self::BsBox(BoxType::Brawl),
            Self::TOKEN_DOUBLERS => Self::TokenDoublers,
            Self::MEGA_BOX => Self::BsBox(BoxType::Mega),
            Self::POWER_POINTS => Self::PowerPoints,
            Self::GAME_MODE => Self::GameMode,
            Self::BIG_BOX => Self::BsBox(BoxType::Big),
            _ => return None,
        })
    }

    /// Converts a [`TrophyRoadRewardKind`] into its code.
    ///
    /// Note: `None` is returned for custom box types.
    pub fn to_code(self) -> Option<u8> {
        Some(match self {
            Self::Gold => Self::GOLD,
            Self::Brawler => Self::BRAWLER,
            Self::TokenDoublers => Self::TOKEN_DOUBLERS,
            Self::PowerPoints => Self::POWER_POINTS,
            Self::GameMode => Self::GAME_MODE,
            Self::BsBox(bt) => match bt {
                BoxType::Brawl => Self::BRAWL_BOX,
                BoxType::Big => Self::BIG_BOX,
                BoxType::Mega => Self::MEGA_BOX,
                BoxType::Custom(_) => return None,
            },
        })
    }
}

impl<'de> Deserialize<'de> for TrophyRoadRewardKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_code(u8::deserialize(deserializer)?).ok_or_else(|| {
            DeError::custom("expected one of `1`, `3`, `6`, `9`, `10`, `12`, `13` or `14`")
        })
    }
}

impl Serialize for TrophyRoadRewardKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(code) = self.to_code() {
            serializer.serialize_u8(code)
        } else {
            Err(SerError::custom("unexpected reward type found"))
        }
    }
}
