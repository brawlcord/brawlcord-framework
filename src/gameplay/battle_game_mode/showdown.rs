use super::{BattleGameMode, GeneralMove, Moves};
use crate::error::{Error, Result};
use crate::gameplay::player::{Player, PlayerState};
use crate::gameplay::{GameHandler, GameResult, Players};
use crate::utils::rng;

/// The round number at which the poison effect begins.
const POISON_ROUND_NUM: u8 = 40;
/// The amount of damage done due to the poison effect.
const POISON_DAMAGE: u32 = 100;

#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct Showdown {
    /// The round number of the game.
    round_num: u8,
}

impl Showdown {
    /// Creates a new [`Showdown`] object.
    pub fn new() -> Self {
        Self::default()
    }

    fn initialize_player(&self, player: &mut PlayerState) {
        player.extra.insert("powerups", 0);
    }

    /// Runs Showdown.
    pub async fn run(
        mut self,
        players: &mut Players,
        handler: &dyn GameHandler,
    ) -> Result<GameResult> {
        self.initialize_player(&mut players.0.state);
        self.initialize_player(&mut players.1.state);

        let mut result = None;

        while self.round_num < 150 {
            let (first, second) = if self.round_num % 2 == 0 {
                (&mut players.0, &mut players.1)
            } else {
                (&mut players.1, &mut players.0)
            };

            first.regenerate_ammo(self.round_num);
            BattleGameMode::heal(first, self.round_num);

            if first.state.is_stunned {
                BattleGameMode::handle_stun(first, &second.id, handler).await?;
                self.round_num += 1;
                continue;
            }

            let user_move = self.get_user_move(first, second, handler).await?;

            self.handle_move(&user_move, first, second).await;
            self.poison_effect(&mut first.state, &mut second.state);

            if let Some(res) = self.check_result(first, second) {
                result = Some(res);
                break;
            }

            self.round_num += 1;
        }

        BattleGameMode::result(result, players, handler).await
    }

    fn possible_moves(&self, first: &Player, second: &Player) -> Vec<ShowdownMove> {
        let mut moves =
            vec![ShowdownMove::General(GeneralMove::Dodge), ShowdownMove::CollectPowerUp];

        let can_attack = first.can_attack();
        let can_super = first.can_super();
        if can_attack {
            moves.push(ShowdownMove::General(GeneralMove::Attack));
        }

        if can_super {
            moves.push(ShowdownMove::General(GeneralMove::Ult));
        }

        if second.state.spawn.is_some() {
            if can_attack {
                moves.push(ShowdownMove::General(GeneralMove::AttackSpawn));
            }

            if can_super {
                moves.push(ShowdownMove::General(GeneralMove::UltSpawn));
            }
        }

        moves
    }

    fn check_result(&self, first: &Player, second: &Player) -> Option<GameResult> {
        if first.state.is_alive() && second.state.is_dead() {
            Some(GameResult::Decisive { winner: first.id, loser: second.id })
        } else if first.state.is_dead() && second.state.is_alive() {
            Some(GameResult::Decisive { winner: second.id, loser: first.id })
        } else if first.state.is_dead() && second.state.is_dead() {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    async fn handle_move(
        &mut self,
        user_move: &ShowdownMove,
        first: &mut Player,
        second: &mut Player,
    ) {
        match user_move {
            ShowdownMove::General(gm) => gm.handle_move(first, second).await,
            ShowdownMove::CollectPowerUp => {
                // 25% chance of collecting a power-up.
                let new = rng::select_one(&[0, 1], &[3, 1]).unwrap_or(&0);

                let powerups = first.state.extra.entry("powerups").or_insert(0);
                *powerups += new;
            },
        }

        second.state.is_invincibile = false;
    }

    async fn get_user_move(
        &self,
        first: &Player,
        second: &Player,
        handler: &dyn GameHandler,
    ) -> Result<ShowdownMove> {
        let possible_moves = self.possible_moves(first, second);
        let move_idx =
            handler.get_move_idx(Moves::Showdown(&possible_moves), first, second).await?;
        if let Some(m) = possible_moves.get(move_idx) {
            Ok(*m)
        } else {
            Err(Error::ResponseError(Some(String::from("invalid move: index out of bounds."))))
        }
    }

    fn poison_effect(&self, first: &mut PlayerState, second: &mut PlayerState) {
        if self.round_num >= POISON_ROUND_NUM {
            first.damage(POISON_DAMAGE);
            second.damage(POISON_DAMAGE);
        }
    }
}

/// Represents a user move in Showdown.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum ShowdownMove {
    /// Represents a general move.
    General(GeneralMove),
    /// Represents the move to collect power up.
    CollectPowerUp,
}
