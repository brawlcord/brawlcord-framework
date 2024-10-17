use std::collections::HashMap;

use indexmap::IndexMap;
use rand::prelude::{IteratorRandom, SliceRandom, ThreadRng};

use super::power_points::PowerPoints;
use crate::model::brawler::{Brawler, ChromaticSeason, Rarity};
use crate::utils::rng;

/// Maximum power points a Brawler can have.
const MAXIMUM_POWER_POINTS: PowerPoints = PowerPoints::max_power_points();
/// The default odds to get token doublers as reward.
pub const TOKEN_DOUBLER_ODDS: u32 = 9;
/// The default amount of token doublers given as reward.
pub const TOKEN_DOUBLER_QUANTITY: u32 = 200;

/// Represnts odds to unlock various items in a [`BsBox`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct BoxOdds {
    /// Odds to get power points.
    power_points: f32,
    /// Odds to get a Rare [`Brawler`](crate::model::brawler::Brawler).
    rare: f32,
    /// Odds to get a Super Rare [`Brawler`](crate::model::brawler::Brawler).
    super_rare: f32,
    /// Odds to get a Epic [`Brawler`](crate::model::brawler::Brawler).
    epic: f32,
    /// Odds to get a Mythic [`Brawler`](crate::model::brawler::Brawler).
    mythic: f32,
    /// Odds to get a Legendary [`Brawler`](crate::model::brawler::Brawler).
    legendary: f32,
    /// Odds to get a Gadget.
    gadget: f32,
    /// Odds to get a Star Power.
    star_power: f32,
}

impl BoxOdds {
    /// Returns odds for the given [`Rarity`].
    pub fn get_rarity_odds(&self, rarity: Rarity) -> f32 {
        match rarity {
            Rarity::TrophyRoad(_) => 0.0,
            Rarity::Rare => self.rare,
            Rarity::SuperRare => self.super_rare,
            Rarity::Epic => self.epic,
            Rarity::Mythic => self.mythic,
            Rarity::Legendary => self.legendary,
            Rarity::Chromatic(season) => match season {
                ChromaticSeason::First => self.legendary,
                ChromaticSeason::Second => self.mythic,
                ChromaticSeason::Third => self.epic,
            },
        }
    }
}

impl Default for BoxOdds {
    fn default() -> Self {
        Self {
            power_points: 92.6516,
            rare: 2.2103,
            super_rare: 1.2218,
            epic: 0.5527,
            mythic: 0.2521,
            legendary: 0.1115,
            gadget: 2.0,
            star_power: 1.0,
        }
    }
}

/// Represents a Box in Brawl Stars.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct BsBox {
    pub box_type: BoxType,
}

impl BsBox {
    /// Creates a new [`BsBox`] with given box type.
    pub fn new(box_type: BoxType) -> Self {
        Self { box_type }
    }

    /// Creates a new Brawl Box.
    pub fn brawl_box() -> Self {
        Self::new(BoxType::Brawl)
    }

    /// Creates a new Big Box.
    pub fn big_box() -> Self {
        Self::new(BoxType::Big)
    }

    /// Creates a new Mega Box.
    pub fn mega_box() -> Self {
        Self::new(BoxType::Mega)
    }

    /// Opens the [`BsBox`].
    pub fn open(&self, player_stats: PlayerStats) -> BoxRewards {
        let box_data = self.box_type.box_data();
        let mut gold = rng::weighted_random(box_data.gold[0], box_data.gold[1], box_data.gold[2]);

        let mut rarities = Vec::new();
        let mut gadgets = 0;
        let mut star_powers = 0;
        let mut token_doubler_odds = TOKEN_DOUBLER_ODDS;

        let mut stacks = 0;

        let selected = BoxItem::select_items(&player_stats.odds, box_data.total);

        for item in selected {
            match item {
                BoxItem::PowerPoints => stacks += 1,
                BoxItem::Brawler(rarity) => rarities.push(rarity),
                BoxItem::Gadget => gadgets += 1,
                BoxItem::StarPower => star_powers += 1,
            }
        }

        let mut unlockable_data = player_stats.get_unlockable_data();

        if unlockable_data.power_points.is_empty() {
            gold *= 3;
            stacks = 0;
        } else if unlockable_data.power_points.len() == 1 {
            gold *= 2;
            stacks = 1;
        } else if unlockable_data.power_points.len() < stacks {
            stacks = unlockable_data.power_points.len();
        }

        let mut rewards = BoxRewards { gold, ..Default::default() };

        add_power_points(stacks, &box_data, unlockable_data.power_points, &mut rewards);

        let mut missed = add_brawlers(rarities, &mut unlockable_data.brawlers, &mut rewards);
        missed += add_gadgets(gadgets, &mut unlockable_data.gadgets, &mut rewards);
        missed += add_star_powers(star_powers, &mut unlockable_data.star_powers, &mut rewards);
        token_doubler_odds *= 2u32.pow(missed);

        if token_doubler_odds >= (0..100).choose(&mut rand::thread_rng()).unwrap() {
            rewards.add_token_doublers(TOKEN_DOUBLER_QUANTITY);
        }

        rewards
    }
}

/// Represents the type of a [`BsBox`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub enum BoxType {
    /// Represents a Brawl Box.
    Brawl,
    /// Represents a Big Box.
    Big,
    /// Represents a Mega Box.
    Mega,
    /// Represents a custom Box.
    Custom(BoxData),
}

impl BoxType {
    /// Returns data for the [`BoxType`].
    const fn box_data(&self) -> BoxData {
        match self {
            Self::Brawl => BoxData { total: 2, power_points: [7, 25, 14], gold: [12, 70, 19] },
            Self::Big => BoxData { total: 5, power_points: [27, 75, 46], gold: [36, 210, 63] },
            Self::Mega => BoxData { total: 9, power_points: [81, 225, 132], gold: [6, 210, 63] },
            Self::Custom(data) => *data,
        }
    }
}

/// Represents data for a [`BoxType`].
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct BoxData {
    /// The total number of items that can be present in a box.
    pub total: u8,
    /// Values that represent how much power points a box can give.
    ///
    /// - 0th index -> lowest value
    /// - 1st index -> highest value
    /// - 2nd index -> average value
    pub power_points: [u32; 3],
    /// Values that represent how much gold a box can give.
    ///
    /// - 0th index -> lowest value
    /// - 1st index -> highest value
    /// - 2nd index -> average value
    pub gold: [u32; 3],
}

impl BoxData {
    /// Creates a new [`BoxData`].
    pub fn new(total: u8, power_points: [u32; 3], gold: [u32; 3]) -> Self {
        Self { total, power_points, gold }
    }
}

/// The relevant statistics of a player used to determine rewards of a [`BsBox`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct PlayerStats<'a> {
    /// The odds of unlocking various items.
    pub odds: BoxOdds,
    /// All Brawlers that are available.
    pub all_brawlers: &'a [Brawler],
    /// List of all data of the Brawlers unlocked by the player.
    pub player_brawlers: &'a [BrawlerData],
}

impl<'a> PlayerStats<'a> {
    /// Creates new [`PlayerStats`] based on player's Brawlers data.
    pub fn new(all_brawlers: &'a [Brawler], player_brawlers: &'a [BrawlerData]) -> Self {
        Self { odds: BoxOdds::default(), all_brawlers, player_brawlers }
    }

    /// Returns [`Unlockable`] data for the player.
    fn get_unlockable_data(&self) -> Unlockable {
        let mut brawlers = HashMap::new();
        for brawler in self.all_brawlers {
            let rarity = if let Some(rarity) = UnlockableRarity::from_rarity(brawler.rarity) {
                rarity
            } else {
                continue;
            };

            if !self.player_brawlers.iter().any(|b| b.name == brawler.name) {
                let entry = brawlers.entry(rarity).or_insert_with(Vec::new);
                entry.push(&*brawler.name);
            }
        }

        let mut power_points = IndexMap::new();
        let mut gadgets = HashMap::new();
        let mut star_powers = HashMap::new();

        for brawler in self.player_brawlers {
            let brawler_name = &*brawler.name;
            let total_power_points =
                PowerPoints::total_from(brawler.power_points.into(), brawler.level);
            if total_power_points < MAXIMUM_POWER_POINTS {
                power_points.insert(brawler_name, MAXIMUM_POWER_POINTS.0 - total_power_points.0);
            }

            let level = brawler.level;

            if level >= 7 {
                // Add gadgets.
                gadgets.insert(
                    brawler_name,
                    TwoVariantsInfo::new(!brawler.gadgets.first, !brawler.gadgets.second),
                );

                if level >= 9 {
                    // Add star powers.
                    star_powers.insert(
                        brawler_name,
                        TwoVariantsInfo::new(
                            !brawler.star_powers.first,
                            !brawler.star_powers.second,
                        ),
                    );
                }
            }
        }

        Unlockable { brawlers, power_points, gadgets, star_powers }
    }
}

/// Container for a player's single Brawler's data.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct BrawlerData {
    /// Name of the Brawler.
    name: String,
    /// Power level of the Brawler.
    level: u8,
    /// Number of power points the Brawler has at current level.
    ///
    /// This does not include power points consumed to level up the Brawler.
    power_points: u32,
    /// Represents info about the gadgets of the Brawler.
    gadgets: TwoVariantsInfo,
    /// Represents info about the star powers of the Brawler.
    star_powers: TwoVariantsInfo,
}

impl BrawlerData {
    /// Creates new [`BrawlerData`] from a Brawler's data.
    ///
    /// Note that the `power_points` should only include the power points of the Brawler
    /// at the current level. It should not include power points consumed to level up
    /// the Brawler.
    pub fn new(
        name: String,
        level: u8,
        power_points: u32,
        first_gadget: bool,
        second_gadget: bool,
        first_sp: bool,
        second_sp: bool,
    ) -> Self {
        Self {
            name,
            level,
            power_points,
            gadgets: TwoVariantsInfo::new(first_gadget, second_gadget),
            star_powers: TwoVariantsInfo::new(first_sp, second_sp),
        }
    }
}

/// A simple struct to represent info about items that have two variants.
#[derive(Clone, Copy, Debug, Default)]
struct TwoVariantsInfo {
    /// If the player has the first variant or not.
    first: bool,
    /// If the player has the second variant or not.
    second: bool,
}

impl TwoVariantsInfo {
    /// Creates a new [`TwoVariantsInfo`].
    fn new(first: bool, second: bool) -> Self {
        Self { first, second }
    }

    /// Returns [`TwoVariantsInfo`] after choosing one of the possible two variants.
    ///
    /// If at least one of the variants is false, it simply returns a copy of itself.
    fn choose_one(&self, mut rng: &mut ThreadRng) -> Self {
        if self.first && self.second {
            let choice = *[1, 2].choose(&mut rng).unwrap();
            Self { first: choice == 1, second: choice == 2 }
        } else {
            *self
        }
    }

    /// Combines two instances of [`TwoVariantsInfo`] into one.
    fn combine(&mut self, other: &Self) {
        // Self { first: self.first || other.first, second: self.second || other.second }
        self.first |= other.first;
        self.second |= other.second;
    }

    /// Returns true if at least one of the variants is true.
    fn has_at_least_one(&self) -> bool {
        self.first || self.second
    }
}

/// Represents all unlockable Brawler data for a player.
#[derive(Clone, Debug)]
struct Unlockable<'a> {
    /// A mapping of unlockable rarities corresponding to names of Brawlers that can
    /// be unlocked.
    brawlers: HashMap<UnlockableRarity, Vec<&'a str>>,
    /// A mapping of names of Brawlers and numbers of power points that can be unlocked.
    power_points: IndexMap<&'a str, u32>,
    /// A mapping of names of Brawlers and gadgets that can be unlocked.
    gadgets: HashMap<&'a str, TwoVariantsInfo>,
    /// A mapping of names of Brawlers and star powers that can be unlocked.
    star_powers: HashMap<&'a str, TwoVariantsInfo>,
}

/// Represents unlockable rarity.
///
/// This includes all variants of [`Rarity`] except [`Rarity::TrophyRoad`] and
/// [`Rarity::Chromatic`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
enum UnlockableRarity {
    /// Represents the Rare rarity.
    Rare,
    /// Represents the Super Rare rarity.
    SuperRare,
    /// Represents the Epic rarity.
    Epic,
    /// Represents the Mythic rarity.
    Mythic,
    /// Represents the Legendary rarity.
    Legendary,
}

impl UnlockableRarity {
    /// Creates a new [`UnlockableRarity`] from given [`Rarity`].
    ///
    /// Returns `None` if `rarity` is [`Rarity::TrophyRoad`]. [`Rarity::Chromatic`] is converted
    /// to another rarity based on the season.
    fn from_rarity(rarity: Rarity) -> Option<Self> {
        let unlockable_rarity = match rarity {
            Rarity::TrophyRoad(_) => return None,
            Rarity::Rare => Self::Rare,
            Rarity::SuperRare => Self::SuperRare,
            Rarity::Epic => Self::Epic,
            Rarity::Mythic => Self::Mythic,
            Rarity::Legendary => Self::Legendary,
            Rarity::Chromatic(season) => match season {
                ChromaticSeason::First => Self::Legendary,
                ChromaticSeason::Second => Self::Mythic,
                ChromaticSeason::Third => Self::Epic,
            },
        };

        Some(unlockable_rarity)
    }

    /// Returns optional [`UnlockableRarity`] right under the current rarity.
    ///
    /// `None` is returned if the current rarity is [`UnlockableRarity::Rare`].
    fn lower(&self) -> Option<Self> {
        match *self {
            Self::Legendary => Some(Self::Mythic),
            Self::Mythic => Some(Self::Epic),
            Self::Epic => Some(Self::SuperRare),
            Self::SuperRare => Some(Self::Rare),
            _ => None,
        }
    }
}

/// Enum to represent a single box item.
#[derive(Copy, Clone, Debug)]
enum BoxItem {
    /// Represents power points.
    PowerPoints,
    /// Represents a Brawler with given rarity.
    Brawler(UnlockableRarity),
    /// Represents a Gadget.
    Gadget,
    /// Represents a Star Power.
    StarPower,
}

impl BoxItem {
    /// Selects random `total` items with specified odds.
    pub fn select_items(odds: &BoxOdds, total: u8) -> Vec<Self> {
        let mut items = Vec::new();
        let choices = vec![
            (Self::PowerPoints, odds.power_points),
            (Self::Brawler(UnlockableRarity::Rare), odds.rare),
            (Self::Brawler(UnlockableRarity::SuperRare), odds.super_rare),
            (Self::Brawler(UnlockableRarity::Epic), odds.epic),
            (Self::Brawler(UnlockableRarity::Mythic), odds.mythic),
            (Self::Brawler(UnlockableRarity::Legendary), odds.legendary),
            (Self::Gadget, odds.gadget),
            (Self::StarPower, odds.star_power),
        ];

        let mut rng = rand::thread_rng();

        for _ in 0..total as usize {
            let item = choices
                .choose_weighted(&mut rng, |item| item.1)
                .unwrap_or(&(Self::PowerPoints, 0.0))
                .0;
            items.push(item);
        }

        items
    }
}

/// Represents rewards unlocked by opening a [`BsBox`].
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct BoxRewards {
    /// Names of Brawlers unlocked.
    pub brawlers: Vec<String>,
    /// Mapping of Brawler and [`PowerPoints`] collected.
    pub power_points: HashMap<String, PowerPoints>,
    /// Mapping of Brawler and gadget(s) unlocked.
    pub gadgets: HashMap<String, UnlockedGadgets>,
    /// Mapping of Brawler and star power(s) unlocked.
    pub star_powers: HashMap<String, UnlockedStarPowers>,
    /// Amount of gold collected.
    pub gold: u32,
    /// Amount of token doublers collected.
    pub token_doublers: Option<u32>,
}

impl BoxRewards {
    /// Adds [`PowerPoints`] for a Brawler.
    pub fn add_power_points(&mut self, brawler: String, power_points: impl Into<PowerPoints>) {
        let entry = self.power_points.entry(brawler).or_insert(PowerPoints(0));
        *entry += power_points.into();
    }

    /// Adds a new Brawler.
    pub fn add_brawler(&mut self, brawler: impl ToString) {
        self.brawlers.push(brawler.to_string());
    }

    /// Adds Gadget(s) to a Brawler.
    ///
    /// If any Gadget(s) are already present for the Brawler, the new one(s) are added
    /// in addition to the previous ones.
    pub fn add_gadgets(&mut self, brawler: impl ToString, gadgets: UnlockedGadgets) {
        self.gadgets
            .entry(brawler.to_string())
            .and_modify(|e| e.0.combine(&gadgets.0))
            .or_insert(gadgets);
    }

    /// Adds Star Power(s) to a Brawler.
    ///
    /// If any Star Power(s) are already present for the Brawler, the new one(s) are added
    /// in addition to the previous ones.
    pub fn add_star_powers(&mut self, brawler: impl ToString, star_powers: UnlockedStarPowers) {
        self.star_powers
            .entry(brawler.to_string())
            .and_modify(|e| e.0.combine(&star_powers.0))
            .or_insert(star_powers);
    }

    /// Adds `quantity` token doublers to the existing amount of reward token doublers.
    pub fn add_token_doublers(&mut self, quantity: u32) {
        *self.token_doublers.get_or_insert(0) += quantity;
    }
}

/// Represents unlocked Gadget(s) for a Brawler.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct UnlockedGadgets(TwoVariantsInfo);

impl UnlockedGadgets {
    /// Returns true if the first Gadget is unlocked.
    pub fn first(&self) -> bool {
        self.0.first
    }

    /// Returns true if the second Gadget is unlocked.
    pub fn second(&self) -> bool {
        self.0.second
    }

    /// Sets the `first` Gadget to `value`.
    pub fn set_first(&mut self, value: bool) -> &mut Self {
        self.0.first = value;
        self
    }

    /// Sets the `second` Gadget to `value`.
    pub fn set_second(&mut self, value: bool) -> &mut Self {
        self.0.second = value;
        self
    }
}

/// Represents unlocked Star Power(s) for a Brawler.
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct UnlockedStarPowers(TwoVariantsInfo);

impl UnlockedStarPowers {
    /// Returns true if the first Star Power is unlocked.
    pub fn first(&self) -> bool {
        self.0.first
    }

    /// Returns true if the second Star Power is unlocked.
    pub fn second(&self) -> bool {
        self.0.second
    }

    /// Sets the `first` Star Power to `value`.
    pub fn set_first(&mut self, value: bool) -> &mut Self {
        self.0.first = value;
        self
    }

    /// Sets the `second` Star Power to `value`.
    pub fn set_second(&mut self, value: bool) -> &mut Self {
        self.0.second = value;
        self
    }
}

/// Adds reward [`PowerPoints`] to [`BoxRewards`].
fn add_power_points(
    stacks: usize,
    box_data: &BoxData,
    mut power_points_map: IndexMap<&str, u32>,
    rewards: &mut BoxRewards,
) {
    if stacks > 0 {
        let [lower, upper, avg] = box_data.power_points;
        let power_points = rng::weighted_random(lower, upper, avg);
        let pieces = rng::split_in_integers(power_points, stacks as u32, 1);

        // let mut power_points_map = unlockable_data.power_points;
        rng::shuffle_index_map(&mut power_points_map, &mut rand::thread_rng());

        for piece in pieces {
            for (&brawler, &threshold) in &power_points_map {
                if threshold >= piece {
                    rewards.add_power_points(brawler.to_string(), piece);
                    power_points_map.swap_remove(brawler);
                    break;
                }
            }
        }
    }
}

/// Adds reward Brawlers of provided rarities to [`BoxRewards`].
///
/// Unlocked Brawlers are also removed from `unlockable_brawlers`. It does
/// not preserve the order of the Brawlers.
///
/// Returns the number of rarities missed.
fn add_brawlers(
    rarities: Vec<UnlockableRarity>,
    unlockable_brawlers: &mut HashMap<UnlockableRarity, Vec<&str>>,
    rewards: &mut BoxRewards,
) -> u32 {
    let mut missed = 0;
    for rarity in rarities {
        if let Some(rarity) = get_valid_rarity(rarity, unlockable_brawlers) {
            // Unwrapping here is fine here because `get_valid_rarity` ensures the rarity
            // is present in the map and the rarity has at least one unlockable Brawler.
            let brawlers = unlockable_brawlers.get_mut(&rarity).unwrap();
            let index = (0..brawlers.len()).choose(&mut rand::thread_rng()).unwrap();
            rewards.add_brawler(brawlers[index]);
            brawlers.swap_remove(index);
        } else {
            missed += 1;
        }
    }

    missed
}

/// Adds `total` reward Gadgets for Brawlers that can have at least one Gadget added.
///
/// Unlocked Gadgets are removed from `unlockable_gadgets`. It does
/// not preserve the order of the Brawlers.
///
/// Returns the number of Gadgets that could not be added.
fn add_gadgets(
    total: u32,
    unlockable_gadgets: &mut HashMap<&str, TwoVariantsInfo>,
    rewards: &mut BoxRewards,
) -> u32 {
    let mut missed = 0;
    for _ in 0..total {
        if let Some((brawler, choice)) = handle_two_variants(unlockable_gadgets) {
            rewards.add_gadgets(brawler, UnlockedGadgets(choice));
        } else {
            missed += 1;
        }
    }

    missed
}

/// Adds `total` reward Star Powers for Brawlers that can have at least one Star Power added.
///
/// Unlocked Star Powers are removed from `unlockable_star_powers`. It does
/// not preserve the order of the Brawlers.
///
/// Returns the number of Star Powers that could not be added.
fn add_star_powers(
    total: u32,
    unlockable_star_powers: &mut HashMap<&str, TwoVariantsInfo>,
    rewards: &mut BoxRewards,
) -> u32 {
    let mut missed = 0;
    for _ in 0..total {
        if let Some((brawler, choice)) = handle_two_variants(unlockable_star_powers) {
            rewards.add_star_powers(brawler, UnlockedStarPowers(choice));
        } else {
            missed += 1;
        }
    }

    missed
}

/// Handles a mapping of Brawler items with two variants.
///
/// It chooses one variant and removes the chosen variant from the mapping.
///
/// Returns the selected Brawler and the variant. `None` is returned when the mapping
/// is empty.
fn handle_two_variants<'a>(
    mapping: &mut HashMap<&'a str, TwoVariantsInfo>,
) -> Option<(&'a str, TwoVariantsInfo)> {
    let mut rng = rand::thread_rng();

    if let Some((&brawler, variants_info)) = mapping.iter().choose(&mut rng) {
        let choice = variants_info.choose_one(&mut rng);

        // Remove the unlocked variant from the available variants for the Brawler.
        if let Some(entry) = mapping.get_mut(&brawler) {
            if choice.first {
                entry.first = false;
            } else if choice.second {
                entry.second = false;
            }

            if !entry.has_at_least_one() {
                mapping.remove(&brawler);
            }
        }

        Some((brawler, choice))
    } else {
        None
    }
}

/// Returns a valid [`UnlockableRarity`], starting from the passed rarity.
///
/// A rarity is considered valid if a player can unlock at least
/// one Brawler in that rarity.
///
/// If no Brawler can be unlocked, `None` is returned.
fn get_valid_rarity(
    rarity: UnlockableRarity,
    unlockable_brawlers: &HashMap<UnlockableRarity, Vec<&str>>,
) -> Option<UnlockableRarity> {
    let mut opt_rarity = Some(rarity);
    loop {
        if let Some(rarity) = opt_rarity {
            if unlockable_brawlers.get(&rarity).map_or(false, |v| !v.is_empty()) {
                return opt_rarity;
            } else {
                opt_rarity = rarity.lower();
            }
        } else {
            return None;
        }
    }
}
