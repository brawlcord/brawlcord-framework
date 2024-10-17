//! Utilities to assist with start-progress tier systems.
//!
//! These include leagues, Brawler ranks and Brawler levels.

use serde::{Deserialize, Serialize};

/// Represents a Brawler level.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Level {
    /// The number of power points at which the level starts.
    pub start: u32,
    /// The number of power points that must be collected to advance to the next level.
    pub progress: u32,
    /// The amount of currency required to advance to the next level.
    pub required_currency: u32,
}

impl Level {
    /// Creates a new [`Level`].
    pub fn new(start: u32, progress: u32, required_currency: u32) -> Self {
        Self { start, progress, required_currency }
    }
}

impl_tier!(
    Level,
    u32,
    "The number of power points at which this level ends and the next level begins.",
    "Checks if a Brawler with given power points can be upgraded to the next level."
);

/// Represents a level manager to assist with level-ups.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LevelManager(Vec<Level>);

impl LevelManager {
    /// Returns the number of power points required to level up from given `level`.
    ///
    /// Returns `None` if tier corresponding to the level is not present in the manager.
    pub fn level_up_cost(&self, level: u8) -> Option<u32> {
        if level == 0 {
            None
        } else {
            self.get(level as usize - 1).map(|l| l.required_currency)
        }
    }
}

impl_tier_manager!(LevelManager, Level);

/// Represents a league of the trophy road.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct League {
    /// The name of the league.
    pub name: String,
    /// The number of trophies at which the league begins.
    pub start: u32,
    /// The number of trophies which must be gained in this
    /// league to qualify for the next league.
    pub progress: u32,
}

impl League {
    /// Creates a new [`League`].
    pub fn new(name: String, start: u32, progress: u32) -> Self {
        Self { name, start, progress }
    }
}

impl_tier!(
    League,
    u32,
    "The number of trophies at which this league ends and the next league begins.",
    "Checks if a player with given trophies can advance to the next league."
);

/// Represents a league manager to assist with league-ups.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct LeagueManager(Vec<League>);

impl_tier_manager!(LeagueManager, League);

/// Represents the trophy rank of a Brawler.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Rank {
    /// The number of trophies at which the rank begins.
    pub start: u32,
    /// The number of trophies which must be gained
    ///  at this rank to qualify for the next rank.
    pub progress: u32,
    /// The count of the primary reward for leveling up from the rank.
    pub primary_reward_count: u32,
    /// The count of the secondary reward for leveling up from the rank.
    pub secondary_reward_count: u32,
}

impl Rank {
    /// Creates a new [`Rank`].
    pub fn new(
        start: u32,
        progress: u32,
        primary_reward_count: u32,
        secondary_reward_count: u32,
    ) -> Self {
        Self { start, progress, primary_reward_count, secondary_reward_count }
    }

    // /// The number of trophies at which this rank ends and the next rank begins.
    // pub fn end(&self) -> u32 {
    //     self.start + self.progress
    // }

    // /// Checks if the a Brawler with given trophies can advance to the next rank.
    // pub fn can_advance(&self, trophies: u32) -> bool {
    //     trophies >= self.end()
    // }
}

impl_tier!(
    Rank,
    u32,
    "The number of trophies at which this rank ends and the next rank begins.",
    "Checks if a Brawler with given trophies can advance to the next rank."
);

/// Represents a rank manager to assist with rank-ups.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct RankManager(Vec<Rank>);

impl_tier_manager!(RankManager, Rank);
