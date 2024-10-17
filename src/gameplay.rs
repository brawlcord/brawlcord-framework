//! The core of the library.
//!
//! This module defines the most important data types and their interactions
//! in the form of a "brawl".

pub mod battle_brawler;
pub mod battle_game_mode;
pub mod battle_log;
pub mod player;

use std::sync::Arc;

use async_trait::async_trait;
#[doc(inline)]
pub use battle_brawler::{BrawlerExt, BrawlerInfo, Spawn};
#[doc(inline)]
pub use battle_game_mode::{BattleGameMode, Moves};
#[doc(inline)]
pub use player::{Player, PlayerId};

use crate::error::Result;

/// Represents a brawl/game.
#[derive(Clone)]
pub struct Game {
    pub game_mode: BattleGameMode,
    pub players: Players,
    pub result: Option<GameResult>,
    pub handler: Arc<dyn GameHandler>,
}

impl Game {
    /// Creates a new [`Game`](Game).
    pub fn new<H: 'static + GameHandler>(
        gamemode: BattleGameMode,
        players: Players,
        handler: H,
    ) -> Self {
        Self { game_mode: gamemode, players, result: None, handler: Arc::new(handler) }
    }

    /// Runs the game.
    pub async fn run(mut self) -> Result<GameResult> {
        self.game_mode.run(&mut self.players, self.handler.as_ref()).await
    }
}

/// Represents the players of a game.
#[derive(Clone, Debug)]
pub struct Players(pub Player, pub Player);

impl Players {
    /// Creates a new [`Players`](Players) instance.
    pub fn new(first: Player, second: Player) -> Self {
        Self(first, second)
    }
}

/// Represents the result of a game.
#[derive(Clone, Debug)]
pub enum GameResult {
    /// Game ended with one player winning and one losing.
    Decisive { winner: PlayerId, loser: PlayerId },
    /// Game ended in a draw.
    Draw,
}

impl GameResult {
    /// Checks if the result is decisive.
    pub fn is_decisive(&self) -> bool {
        matches!(self, Self::Decisive { winner: _, loser: _ })
    }

    /// Checks if the result is a draw.
    pub fn is_draw(&self) -> bool {
        matches!(self, Self::Draw)
    }
}

/// A trait to enable communication between the game and the players during a brawl.
///
/// This trait allows developers to handle events emitted during games and notify
/// or prompt the players appropriately. Some events are dispatched solely for the purpose
/// of providing information whereas some events expect a user response.
///
/// The game is terminated if any errors originate while informing or prompting players.
#[async_trait]
pub trait GameHandler {
    /// This is used to share information about the game to the players.
    async fn info(&self, player_id: &PlayerId, msg: &str) -> Result<()>;

    async fn get_move_idx<'a>(
        &self,
        moves: Moves<'a>,
        first: &Player,
        second: &Player,
    ) -> Result<usize>;
}
