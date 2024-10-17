use rand::Rng;

use super::{BattleGameMode, GeneralMove, Moves};
use crate::error::{Error, Result};
use crate::gameplay::player::{Player, PlayerState};
use crate::gameplay::{GameHandler, GameResult, Players};
use crate::utils::rng;

/// Represents Gem Grab.
///
/// Gem Grab is a 1v1 gamemode (3v3 in-game) where you need to collect 10 gems
/// before the enemy to win. At the center of the map, a gem mine spits out a gem
/// every few seconds.
///
/// If a player dies, he drops all his gems where he died. He respawns at the back of the map
/// after one game round.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct GemGrab {
    /// Number of dropped gems available.
    dropped: u8,
}

impl GemGrab {
    /// Creates a new [`GemGrab`] object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add gems to dropped amount.
    fn drop_gems(&mut self, gems: u8) {
        self.dropped += gems;
    }

    fn initialize_player(&self, player: &mut PlayerState) {
        player.extra.insert("gems", 0);
        player.extra.insert("dropped", 0);
    }

    /// Runs Gem Grab.
    pub async fn run(
        mut self,
        players: &mut Players,
        handler: &dyn GameHandler,
    ) -> Result<GameResult> {
        self.initialize_player(&mut players.0.state);
        self.initialize_player(&mut players.1.state);

        let mut result = None;
        let mut round_num = 0;

        while round_num < 150 {
            let (first, second) = if round_num % 2 == 0 {
                (&mut players.0, &mut players.1)
            } else {
                (&mut players.1, &mut players.0)
            };

            if first.state.status.is_respawning() {
                BattleGameMode::dispatch_respawning_message(&first.id, handler).await?;
            } else {
                first.regenerate_ammo(round_num);
                BattleGameMode::heal(first, round_num);

                if first.state.is_stunned {
                    BattleGameMode::handle_stun(first, &second.id, handler).await?;
                    round_num += 1;
                    continue;
                }

                let possible_moves = self.possible_moves(first, second);
                let move_idx =
                    handler.get_move_idx(Moves::GemGrab(&possible_moves), first, second).await?;
                let user_move = if let Some(m) = possible_moves.get(move_idx) {
                    m
                } else {
                    return Err(Error::ResponseError(Some(String::from(
                        "Invalid move: index out of bounds.",
                    ))));
                };

                self.handle_move(user_move, first, second).await;

                if second.state.health == 0 {
                    second.respawn();

                    let gems = second.state.extra.entry("gems").or_insert(0);

                    // This works because remainder when division by 2 is always 0 or 1.
                    let dropped = gems.div_euclid(2) + gems.rem_euclid(2);
                    *gems -= dropped;

                    self.drop_gems(dropped);

                    handler.info(&first.id, "Opponent defeated! Respawning next round.").await?;
                    handler.info(&second.id, "You are defeated! Respawning next round.").await?;

                    round_num += 1;
                    continue;
                }
            }

            if let Some(res) = self.check_result(first, second) {
                result = Some(res);
                break;
            }

            round_num += 1;
        }

        let final_result = if let Some(result) = result {
            result
        } else {
            BattleGameMode::time_out(&players.0.id, &players.1.id, handler).await?;
            GameResult::Draw
        };

        Ok(final_result)
    }

    fn possible_moves(&self, first: &Player, second: &Player) -> Vec<GemGrabMove> {
        let mut moves = vec![GemGrabMove::General(GeneralMove::Dodge), GemGrabMove::CollectGem];

        let can_attack = first.can_attack();
        let can_super = first.can_super();

        if !second.state.status.is_respawning() {
            if can_attack {
                moves.push(GemGrabMove::General(GeneralMove::Attack));
            }

            if can_super {
                moves.push(GemGrabMove::General(GeneralMove::Ult));
            }
        } else {
            moves.push(GemGrabMove::CollectDroppedGems);
        }

        if second.state.spawn.is_some() {
            if can_attack {
                moves.push(GemGrabMove::General(GeneralMove::AttackSpawn));
            }

            if can_super {
                moves.push(GemGrabMove::General(GeneralMove::UltSpawn));
            }
        }

        moves
    }

    fn check_result(&self, first: &Player, second: &Player) -> Option<GameResult> {
        let first_gems = *first.state.extra.get("gems").unwrap_or(&0);
        let second_gems = *second.state.extra.get("gems").unwrap_or(&0);

        if first_gems >= 10 && second_gems < 10 {
            Some(GameResult::Decisive { winner: first.id, loser: second.id })
        } else if second_gems >= 10 && first_gems < 10 {
            Some(GameResult::Decisive { winner: second.id, loser: first.id })
        } else if first_gems >= 10 && second_gems >= 10 {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    async fn handle_move(
        &mut self,
        user_move: &GemGrabMove,
        first: &mut Player,
        second: &mut Player,
    ) {
        match user_move {
            GemGrabMove::General(gm) => gm.handle_move(first, second).await,
            GemGrabMove::CollectGem => {
                // 75% chance of collecting a gem.
                let new = rng::select_one(&[0, 1], &[1, 3]).unwrap_or(&0);

                let gems = first.state.extra.entry("gems").or_insert(0);
                *gems += new;
            },
            GemGrabMove::CollectDroppedGems => {
                let new = rand::thread_rng().gen_range(0..self.dropped);
                self.dropped -= new;

                let gems = first.state.extra.entry("gems").or_insert(0);
                *gems += new;

                self.dropped = 0;
            },
        }

        second.state.is_invincibile = false;
    }
}

/// Represents a user move in Gem Grab.
#[derive(Copy, Clone, Debug)]
pub enum GemGrabMove {
    /// Represents a general move.
    General(GeneralMove),
    /// Represents the move to collect gem.
    CollectGem,
    /// Represents the move to collect dropped gems.
    CollectDroppedGems,
}
