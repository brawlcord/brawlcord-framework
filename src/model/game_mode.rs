//! A collection of models and helpers related to game modes.

use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::error::Error;

/// Represents a Game Mode.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[non_exhaustive]
pub struct GameMode {
    /// The event of the game mode.
    pub event: Event,
    /// The description of the game mode.
    pub description: Option<String>,
}

impl GameMode {
    /// Creates a new [`GameMode`] from event and optional description.
    pub fn new(event: Event, description: Option<String>) -> Self {
        Self { event, description }
    }

    /// Returns the [`EventType`] corresponding to the event of the game mode.
    pub const fn get_event_type(&self) -> EventType {
        self.event.get_event_type()
    }
}

/// Represents the game mode event.
///
/// It includes 7 main game mode events present in Brawl Stars.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum Event {
    /// Represents Gem Grab.
    #[serde(rename = "Gem Grab")]
    GemGrab,
    /// Represents Showdown.
    Showdown,
    /// Represents Brawl Ball.
    #[serde(rename = "Brawl BrawlBall")]
    BrawlBall,
    /// Represents Heist.
    Heist,
    /// Represents Bounty.
    Bounty,
    /// Represents Siege.
    Siege,
    /// Represents Hot Zone.
    #[serde(rename = "Hot Zone")]
    HotZone,
}

impl Event {
    /// Returns the [`EventType`] corresponding to the event.
    pub const fn get_event_type(&self) -> EventType {
        match self {
            Self::Showdown => EventType::Individual,
            _ => EventType::Team,
        }
    }
}

impl FromStr for Event {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &*s.to_ascii_lowercase() {
            "gemgrab" | "gem grab" => Self::GemGrab,
            "showdown" => Self::Showdown,
            "brawlball" | "brawl ball" => Self::BrawlBall,
            "heist" => Self::Heist,
            "bounty" => Self::Bounty,
            "siege" => Self::Siege,
            "hotzone" | "hot zone" => Self::HotZone,
            _ => return Err(Error::MiscError(format!("`{}` is not a valid event", s))),
        })
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::GemGrab => "Gem Grab",
            Self::Showdown => "Showdown",
            Self::BrawlBall => "Brawl Ball",
            Self::Heist => "Heist",
            Self::Bounty => "Bounty",
            Self::Siege => "Siege",
            Self::HotZone => "Hot Zone",
        })
    }
}

/// Represents the type of the game mode.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub enum EventType {
    /// Represents a team game mode.
    Team,
    /// Represents an individual game mode.
    Individual,
}

impl FromStr for EventType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &*s.to_ascii_lowercase() {
            "team" => Self::Team,
            "individual" => Self::Individual,
            _ => return Err(Error::MiscError(format!("`{}` is not a valid event type", s))),
        })
    }
}
