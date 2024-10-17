pub mod gemgrab;
pub mod showdown;

use gemgrab::*;
use showdown::*;

use super::player::{Player, PlayerId};
use super::{GameHandler, GameResult, Players};
use crate::error::Result;
use crate::model::game_mode::Event;

const HEALING_TIME: u8 = 3;
const HEALING_OVER_TIME: u32 = 100;

/// Represents a game mode usable for battles.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct BattleGameMode {
    /// Represents the event of the game mode.
    pub event: Event,
}

impl BattleGameMode {
    /// Runs the game.
    pub async fn run(self, players: &mut Players, handler: &dyn GameHandler) -> Result<GameResult> {
        match self.event {
            Event::GemGrab => GemGrab::new().run(players, handler).await,
            _ => unimplemented!(),
        }
    }

    /// Heals a player.
    ///
    /// Whether a player is healed or not depends on the round when the player
    /// last attacked or took damage. `true` is returned if the player is healed,
    /// `false` if not.
    pub fn heal(player: &mut Player, round_num: u8) -> bool {
        if player.state.last_attack_round + HEALING_TIME < round_num {
            player.heal(HEALING_OVER_TIME);

            true
        } else {
            false
        }
    }

    /// Handles the stun of a player.
    ///
    /// It removes the stun if a player is stunned and informs both the players
    /// about the stun.
    ///
    /// Forwards any error that occurs due to the event dispatch.
    pub async fn handle_stun(
        stunned: &mut Player,
        other_id: &PlayerId,
        handler: &dyn GameHandler,
    ) -> Result<()> {
        if !stunned.state.is_stunned {
            return Ok(());
        }

        handler.info(&stunned.id, "You are stunned!").await?;
        handler.info(other_id, "Opponent is stunned!").await?;

        stunned.state.is_stunned = false;

        Ok(())
    }

    /// Informs both players that the match ended in a draw because of timeout.
    ///
    /// Forwards any error that occurs due to the event dispatch.
    pub async fn time_out(
        first: &PlayerId,
        second: &PlayerId,
        handler: &dyn GameHandler,
    ) -> Result<()> {
        handler.info(first, "Time's up. Match ended in a draw.").await?;
        handler.info(second, "Time's up. Match ended in a draw.").await?;

        Ok(())
    }

    /// Informs player that they are currently respawning.
    ///
    /// Forwards any error that occurs due to the event dispatch.
    pub async fn dispatch_respawning_message(
        player: &PlayerId,
        handler: &dyn GameHandler,
    ) -> Result<()> {
        handler.info(player, "You are respawning!").await
    }

    /// Returns the final result from optional result.
    ///
    /// If the optional result is `None`, the game is considered timed-out
    /// and timeout event is dispatched for both players.
    ///
    /// Forwards any error that occurs due to the event dispatch.
    pub async fn result(
        result: Option<GameResult>,
        players: &Players,
        handler: &dyn GameHandler,
    ) -> Result<GameResult> {
        Ok(if let Some(result) = result {
            result
        } else {
            BattleGameMode::time_out(&players.0.id, &players.1.id, handler).await?;
            GameResult::Draw
        })
    }
}

/// Represents a valid move in a game mode.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum Moves<'a> {
    /// Moves in Gem Grab.
    GemGrab(&'a [GemGrabMove]),
    Showdown(&'a [ShowdownMove]),
}

/// Represents a user move.
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum GeneralMove {
    /// Attack enemy Brawler.
    Attack,
    /// Use super. It can either spawn a character or attack enemy Brawler.
    Ult,
    /// Attack enemy spawn.
    AttackSpawn,
    /// Use super attack on enemy spawn.
    UltSpawn,
    /// Dodge the next move.
    Dodge,
    /* /// Shoot the ball using a normal attack.
     * ///
     * /// It is valid in Brawl Ball only.
     * ShootBall,
     * /// Shoot the ball using the super.
     * ///
     * /// It is valid in Brawl Ball only.
     * UltShootBall, */
}

impl GeneralMove {
    /// Handles a general move.
    pub async fn handle_move(&self, first: &mut Player, second: &mut Player) {
        let brawler = &first.brawler_state.brawler;
        let brawler_level = first.brawler_state.level;
        match self {
            Self::Attack => brawler.attack(&mut first.state, &mut second.state, brawler_level),
            Self::Ult => brawler.ult(&mut first.state, &mut second.state, brawler_level),
            Self::AttackSpawn => unimplemented!(),
            Self::UltSpawn => unimplemented!(),
            Self::Dodge => first.state.is_invincibile = true,
        }
    }
}
