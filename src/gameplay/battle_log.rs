use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::player::PlayerId;

/// A struct representing a battle log entry.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BattleLogEntry {
    /// The players that participated in the battle.
    pub players: Vec<PlayerLogEntry>,
    /// The name of the Game Mode in which the battle was played.
    pub game_mode: String,
    /// The timestamp when the battle ended.
    pub timestamp: DateTime<Utc>,
}

impl BattleLogEntry {
    /// Creates a new [`BattleLogEntry`].
    pub fn new(players: Vec<PlayerLogEntry>, game_mode: String) -> Self {
        Self { players, game_mode, timestamp: Utc::now() }
    }
}

/// Represents a battle log entry for a player.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PlayerLogEntry {
    /// The ID of the player.
    pub id: PlayerId,
    /// The log entry for the player's Brawler.
    pub brawler_entry: PlayerBrawlerLogEntry,
    /// The reward trophies the player earned from the battle.
    pub reward_trophies: i32,
    /// Whether the player won the battle.
    pub won: bool,
}

impl PlayerLogEntry {
    /// Creates a new [`PlayerLogEntry`].
    pub fn new(
        id: PlayerId,
        brawler_entry: PlayerBrawlerLogEntry,
        reward_trophies: i32,
        won: bool,
    ) -> Self {
        Self { id, brawler_entry, reward_trophies, won }
    }
}

/// Represents a battle log entry for a player's Brawler.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PlayerBrawlerLogEntry {
    /// The name of the Brawler.
    pub name: String,
    /// The level of the Brawler.
    pub level: u32,
    /// The trophies of the Brawler.
    pub trophies: u32,
}

impl PlayerBrawlerLogEntry {
    /// Creates a new [`PlayerBrawlerLogEntry`].
    pub fn new(name: String, level: u32, trophies: u32) -> Self {
        Self { name, level, trophies }
    }
}
