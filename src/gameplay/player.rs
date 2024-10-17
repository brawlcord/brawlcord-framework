use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::battle_brawler::{BrawlerExt, Spawn};

/// A unique identifier for a [`Player`] during a brawl.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PlayerId(pub u64);

/// Represents a player during a brawl.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Player {
    /// The unique ID of the player.
    pub id: PlayerId,
    /// The state of the player.
    pub state: PlayerState,
    /// Whether the player is the first one in lobby or not.
    pub is_first: bool,
    /// The state of the player's brawler.
    pub brawler_state: PlayerBrawlerState,
}

impl Player {
    /// Creates a new [`Player`](Player) with most values set to default.
    pub fn new(id: PlayerId, brawler_state: PlayerBrawlerState, is_first: bool) -> Self {
        let info = brawler_state.brawler.info();
        let ammo = info.attack.max_ammo;
        let health = info.health;

        Self { id, is_first, brawler_state, state: PlayerState::new(ammo, health) }
    }

    /// Tries to regenerate the player's ammo.
    ///
    /// Ammo regeneration depends on the number of round when the
    /// player last used the ammo and its Brawler's reload speed.
    ///
    /// Ammo is not regenerated if the player's current ammo is not less
    /// than the max ammo it can have.
    ///
    /// `true` is returned if ammo is regenerated, `false` if not.
    pub fn regenerate_ammo(&mut self, round_num: u8) -> bool {
        self.state.regenerate_ammo(self.brawler_state.brawler.as_ref(), round_num)
    }

    /// Heals the player by given amount up till the max health.
    pub fn heal(&mut self, amount: u32) {
        self.state.heal(amount);
    }

    /// Sets the player's status as [`Respawning`] and health as max health.
    ///
    /// [`Respawning`]: CharacterStatus::Respawning
    pub fn respawn(&mut self) {
        self.state.status = CharacterStatus::Respawning;
        self.state.health = self.state.max_health;
    }

    /// Checks if the player can attack.
    pub const fn can_attack(&self) -> bool {
        self.state.ammo > 0
    }

    /// Checks if the player can use the SUPER ability.
    pub fn can_super(&self) -> bool {
        self.state.attacks > self.brawler_state.brawler.super_hits_required()
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// A wrapper for the player's spawn.
#[derive(Clone, Debug)]
pub struct PlayerSpawn {
    /// Stats and info about the spawn.
    pub info: Spawn,
    /// The current health of the spawn.
    pub health: u32,
    /// The status of the spawn.
    pub status: CharacterStatus,
}

/// A point representing the player's position.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Position(u32, u32);

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self(x, y)
    }
}

/// The current state of a player.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PlayerState {
    /// Amount of ammo the player has.
    pub ammo: u8,
    /// Round number when player last used ammo.
    pub last_used_ammo: u8,
    /// Number of times player successfully attacked.
    ///
    /// It is reset after a player uses his super.
    pub attacks: u32,
    /// Whether the player is invincible right now or not.
    pub is_invincibile: bool,
    /// The player's current status (alive, dead or respawning)
    pub status: CharacterStatus,
    /// `Spawn` of the player's `Brawler`.
    pub spawn: Option<PlayerSpawn>,
    /// Maximum health points player can have.
    pub max_health: u32,
    /// The player's current health points.
    pub health: u32,
    /// Round number when last attacked opponent or got attacked by the opponent
    pub last_attack_round: u8,
    /// Whether the player is stunned right now or not.
    pub is_stunned: bool,
    /// The player's position on the map.
    pub position: Position,
    /// Extra gamemode-specific data.
    pub extra: HashMap<&'static str, u8>,
}

impl PlayerState {
    /// Creates a new [`PlayerState`] with `ammo` and `health`, using default for other fields.
    pub fn new(ammo: u8, health: u32) -> Self {
        Self {
            ammo,
            last_used_ammo: 0,
            attacks: 0,
            is_invincibile: false,
            status: CharacterStatus::Alive,
            spawn: None,
            max_health: health,
            health,
            last_attack_round: 0,
            is_stunned: false,
            position: Position::new(0, 0),
            extra: HashMap::new(),
        }
    }

    /// Returns distance from another player.
    ///
    /// Distance is calculated using the distance formula:
    /// `sqrt((x1 - x2)^2 + (y1 - y2)^2)`
    pub fn distance_from_player(&self, player_state: &Self) -> f32 {
        ((self.position.0 as f32 - player_state.position.0 as f32).powi(2)
            + (self.position.1 as f32 - player_state.position.1 as f32).powi(2))
        .sqrt()
    }

    /// Tries to regenerate the player's ammo.
    ///
    /// Ammo regeneration depends on the number of round when the
    /// player last used the ammo and its Brawler's reload speed.
    ///
    /// Ammo is not regenerated if the player's current ammo is not less
    /// than the max ammo it can have.
    ///
    /// `true` is returned if ammo is regenerated, `false` if not.
    pub fn regenerate_ammo(&mut self, brawler: &dyn BrawlerExt, round_num: u8) -> bool {
        let attack = &brawler.info().attack;
        let reload = attack.reload.ceil() as u8;
        let max_ammo = attack.max_ammo;

        if self.last_used_ammo <= round_num.saturating_sub(reload) && self.ammo < max_ammo {
            self.ammo += 1;

            true
        } else {
            false
        }
    }

    /// Heals the player by given amount up till the max health.
    fn heal(&mut self, amount: u32) {
        self.health = self.max_health.min(self.health + amount);
    }

    /// Checks if the player is alive.
    ///
    /// It also considers the health of the player in addition to the status.
    pub fn is_alive(&self) -> bool {
        self.status.is_alive() || self.health > 0
    }

    /// Checks if the player is respawning.
    pub fn is_respawning(&self) -> bool {
        self.status.is_respawning()
    }

    /// Checks if the player is respawning.
    ///
    /// It also considers the health of the player in addition to the status.
    pub fn is_dead(&self) -> bool {
        self.status.is_dead() || self.health == 0
    }

    /// Applies `amount` damage to the player, updating the status if the player dies.
    pub fn damage(&mut self, amount: u32) {
        if self.health <= amount {
            self.health = 0;
            self.status = CharacterStatus::Dead;
        } else {
            self.health -= amount;
        }
    }
}

/// Represents the state of a player's brawler.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PlayerBrawlerState {
    /// The player's selected `Brawler`.
    pub brawler: Arc<dyn BrawlerExt>,
    /// The player's selected `Brawler`'s level.
    pub level: u32,
}

impl PlayerBrawlerState {
    /// Creates a new [`PlayerBrawlerState`] with provided brawler and level.
    pub fn new<B: 'static + BrawlerExt>(brawler: B, level: u32) -> Self {
        Self { brawler: Arc::new(brawler), level }
    }
}

/// A character's current status.
#[derive(Clone, Debug)]
pub enum CharacterStatus {
    /// Character is alive.
    Alive,
    /// Character is dead, but respawning.
    Respawning,
    /// Character is dead and cannot respawn.
    Dead,
}

impl CharacterStatus {
    /// Checks if the character is alive.
    pub fn is_alive(&self) -> bool {
        matches!(self, CharacterStatus::Alive)
    }

    /// Checks if the character is respawning.
    pub fn is_respawning(&self) -> bool {
        matches!(self, CharacterStatus::Respawning)
    }

    /// Checks if the character is respawning.
    pub fn is_dead(&self) -> bool {
        matches!(self, CharacterStatus::Dead)
    }
}
