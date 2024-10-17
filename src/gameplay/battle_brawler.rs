//! Provides default implementation for in-game Brawlers along with
//! helpers to implement custom Brawlers.

pub mod defaults;

use std::collections::HashMap;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use super::player::PlayerState;

/// Extension trait for Brawlers that adds all the functionality to them.
///
/// The structure of this trait allows for easy customisation of a Brawler.
/// Only the `info()` method is required to implement this trait for a Brawler.
pub trait BrawlerExt: Send + Sync + std::fmt::Debug {
    /// Returns a reference to the Brawler's info.
    fn info(&self) -> &BrawlerInfo;

    /// Returns a HashMap containing stats that depend on the Brawler level.
    ///
    /// The returned HashMap is guaranteed to contain the following key-value pairs:
    ///
    /// - "health": u32
    /// - "attack": u32
    /// - "ult_damage": u32
    ///
    /// It may contain more pairs depending on the Brawler.
    fn stats(&self) -> HashMap<&str, u32> {
        let info = self.info();

        let mut stats = HashMap::new();
        stats.insert("health", info.health);
        stats.insert("attack", info.attack.damage);
        stats.insert("ult_damage", info.ult.damage.unwrap_or(0));

        stats
    }

    /// Returns Brawler's health at the specified level.
    fn health(&self, level: u32) -> u32 {
        self.buff_stat(self.info().health, level)
    }

    /// Performs Brawler's attack.
    ///
    /// `first` is the attacker, `second` is getting attacked.
    fn attack(&self, first: &mut PlayerState, second: &mut PlayerState, first_brawler_level: u32) {
        let attack = &self.info().attack;

        let attack_damage = self.buff_stat(attack.damage, first_brawler_level);

        let distance = first.distance_from_player(second);

        if attack.range < distance {
            // Can't attack the enemy.
            return;
        }

        // The ceil is important here because if the difference between
        // the range and distance is very small (~0), number of projectiles
        // hit will be very high.
        let diff = (attack.range - distance).ceil();

        let projectiles = (attack.projectiles as f32 / diff).ceil() as u32;

        // Reduce the enemy's health.
        second.health -= attack_damage * projectiles;

        // Decrease ammo.
        first.ammo -= 1;
    }

    /// Performs Brawler's super.
    ///
    /// `first` is the attacker, `second` is getting attacked.
    fn ult(&self, first: &mut PlayerState, second: &mut PlayerState, first_brawler_level: u32) {
        let ult = &self.info().ult;

        let ult_damage = self.buff_stat(ult.damage.unwrap_or(0), first_brawler_level);

        let distance = first.distance_from_player(second);

        if ult.range.unwrap_or(0.0) < distance {
            // Can't attack the enemy.
            return;
        }

        // The ceil is important here because if the difference between
        // the range and distance is very small (~0), number of projectiles
        // will be very high.
        let diff = (ult.range.unwrap_or(0.0) - distance).ceil();

        let projectiles = (ult.projectiles as f32 / diff).ceil() as u32;

        // Reduce the enemy's health.
        second.health -= ult_damage * projectiles;

        // Reset attacks count.
        first.attacks = 0;
    }

    fn chance_calculation(&self, raw: u32) -> u32 {
        let chance: u32 = thread_rng().gen_range(0..11);

        if chance >= 9 {
            raw
        } else if chance >= 6 {
            (raw as f32 * 0.7) as u32
        } else if chance >= 4 {
            (raw as f32 * 0.5) as u32
        } else if chance >= 2 {
            (raw as f32 * 0.3) as u32
        } else {
            0
        }
    }

    /// Returns stat after buffing to the specified level.
    fn buff_stat(&self, base: u32, level: u32) -> u32 {
        let level = if level == 10 { 9 } else { level };

        base + (base as f32 / 20.0 * (level - 1) as f32) as u32
    }

    /// Buffs all `Brawler::stats()` to the specified level.
    fn buff_stats(&self, level: u32) -> HashMap<&str, u32> {
        let mut stats = HashMap::new();
        for (stat, value) in self.stats() {
            stats.insert(stat, self.buff_stat(value, level));
        }

        stats
    }

    /// Returns the number of hits required to use super.
    fn super_hits_required(&self) -> u32 {
        self.info().ult.hits_required
    }

    /// Whether the Brawler has a spawn or not.
    ///
    /// It is set to false by default.
    fn has_spawn(&self) -> bool {
        false
    }

    /// Brawler's spawn attack.
    fn spawn(&self, _level: u32) {
        if !self.has_spawn() {
            return;
        }

        todo!()
    }
}

/// Default amount of ammo a Brawler has.
const fn default_ammo() -> u8 {
    3
}

/// Represents a battle Brawler's info.
///
/// See [`BrawlerExt`] for all methods available for battle Brawlers.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BrawlerInfo {
    /// Name of the Brawler.
    pub name: String,
    /// Health points of the Brawler at level 1.
    pub health: u32,
    /// Speed of the Brawler.
    pub speed: u32,
    /// Brawler's [`BattleAttack`] at level 1.
    pub attack: BattleAttack,
    /// Brawler's SUPER ([`BattleUlt`]) at level 1.
    pub ult: BattleUlt,
}

/// Represents the attack of a Brawler.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BattleAttack {
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
}

/// Represents the SUPER of a Brawler.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct BattleUlt {
    /// Damage output of the SUPER.
    ///
    /// It is `None` for Brawlers with special SUPER.
    pub damage: Option<u32>,
    /// Description of the SUPER.
    pub description: String,
    /// Range of the SUPER.
    ///
    /// It is `None` for spawners.
    pub range: Option<f32>,
    /// Number of projectiles in the SUPER.
    pub projectiles: u32,
    /// Number of hits required to charge SUPER.
    pub hits_required: u32,
    /// Spawn of the SUPER.
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
