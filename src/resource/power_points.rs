use std::ops::{Add, AddAssign};

/// Represents power points.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PowerPoints(pub u32);

impl PowerPoints {
    /// Power Points required to *unlock* a Brawler.
    pub const LEVEL_ONE: PowerPoints = PowerPoints(0);
    /// Power Points required to upgrade a Brawler to level 2 from level 1.
    pub const LEVEL_TWO: PowerPoints = PowerPoints(20);
    /// Power Points required to upgrade a Brawler to level 3 from level 2.
    pub const LEVEL_THREE: PowerPoints = PowerPoints(30);
    /// Power Points required to upgrade a Brawler to level 4 from level 3.
    pub const LEVEL_FOUR: PowerPoints = PowerPoints(50);
    /// Power Points required to upgrade a Brawler to level 5 from level 4.
    pub const LEVEL_FIVE: PowerPoints = PowerPoints(80);
    /// Power Points required to upgrade a Brawler to level 6 from level 5.
    pub const LEVEL_SIX: PowerPoints = PowerPoints(130);
    /// Power Points required to upgrade a Brawler to level 7 from level 6.
    pub const LEVEL_SEVEN: PowerPoints = PowerPoints(210);
    /// Power Points required to upgrade a Brawler to level 8 from level 7.
    pub const LEVEL_EIGHT: PowerPoints = PowerPoints(340);
    /// Power Points required to upgrade a Brawler to level 9 from level 8.
    pub const LEVEL_NINE: PowerPoints = PowerPoints(550);

    /// The maximum number of [`PowerPoints`] required to max out a Brawler.
    pub const fn max_power_points() -> Self {
        Self::LEVEL_TWO
            .const_add(Self::LEVEL_THREE)
            .const_add(Self::LEVEL_FOUR)
            .const_add(Self::LEVEL_FIVE)
            .const_add(Self::LEVEL_SIX)
            .const_add(Self::LEVEL_SEVEN)
            .const_add(Self::LEVEL_EIGHT)
            .const_add(Self::LEVEL_NINE)
    }

    /// Returns [`PowerPoints`] required to upgrade a Brawler to `level` from `level - 1`.
    pub const fn power_points_required(level: u8) -> Self {
        match level {
            2 => Self::LEVEL_TWO,
            3 => Self::LEVEL_THREE,
            4 => Self::LEVEL_FOUR,
            5 => Self::LEVEL_FIVE,
            6 => Self::LEVEL_SIX,
            7 => Self::LEVEL_SEVEN,
            8 => Self::LEVEL_EIGHT,
            9 => Self::LEVEL_NINE,
            _ => Self::LEVEL_ONE,
        }
    }

    /// Returns the maximum number of total [`PowerPoints`] a Brawler can have at a level.
    pub const fn max_at_level(level: u8) -> Self {
        if level == 0 {
            Self(0)
        } else {
            Self::max_at_level(level - 1).const_add(Self::power_points_required(level + 1))
        }
    }

    /// Creates total [`PowerPoints`] from level-specific power points.
    pub const fn total_from(power_points: Self, level: u8) -> Self {
        Self::max_at_level(level).const_add(power_points)
    }

    /// Creates level-specific [`PowerPoints`] from total power points.
    pub const fn level_specific_from(total: Self, level: u8) -> Self {
        total.const_sub(Self::max_at_level(level))
    }

    /// Converts level-specific [`PowerPoints`] to total power points.
    pub const fn to_total(self, level: u8) -> Self {
        Self::total_from(self, level)
    }

    /// Converts total [`PowerPoints`] to level-specific power points.
    pub const fn to_level_specific(self, level: u8) -> Self {
        Self::level_specific_from(self, level)
    }

    /// Returns [`PowerPoints`] required to reach the next level.
    ///
    /// This should be used in cases when `self` represents the **total** number of power points
    /// a Brawler has.
    pub fn to_next_level(self) -> Self {
        // let mut difference = 0;
        let mut i = 1;
        while i < 10 {
            if let Some(difference) = Self::max_at_level(i).0.checked_sub(self.0) {
                return Self(difference);
            }

            i += 1;
        }

        Self(0)
    }

    /// Returns true if a Brawler with total [`PowerPoints`] can be upgraded to specified level.
    ///
    /// Brawlers with level 9 or above cannot be upgraded.
    pub fn can_upgrade(&self, level: u8) -> bool {
        level < 9 && PowerPoints::max_at_level(level) >= *self
    }

    /// Constant addition of two [`PowerPoints`].
    const fn const_add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }

    /// Constant subtraction of rhs from self.
    const fn const_sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Add for PowerPoints {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for PowerPoints {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl From<u32> for PowerPoints {
    fn from(power_points: u32) -> Self {
        Self(power_points)
    }
}

#[cfg(test)]
mod test_power_points {
    use super::*;

    #[test]
    fn test_power_points_required() {
        assert_eq!(PowerPoints::LEVEL_TWO, PowerPoints::power_points_required(2));
        assert_eq!(PowerPoints::LEVEL_NINE, PowerPoints::power_points_required(9));
        assert_eq!(PowerPoints(0), PowerPoints::power_points_required(0));
        assert_eq!(PowerPoints(0), PowerPoints::power_points_required(10));
        assert_eq!(PowerPoints(0), PowerPoints::power_points_required(11));
    }

    #[test]
    fn test_max_power_points() {
        assert_eq!(PowerPoints(1410), PowerPoints::max_power_points());
    }

    #[test]
    fn test_max_at_level() {
        assert_eq!(PowerPoints(0), PowerPoints::max_at_level(0));
        assert_eq!(PowerPoints(20), PowerPoints::max_at_level(1));
        assert_eq!(PowerPoints(100), PowerPoints::max_at_level(3));
        assert_eq!(PowerPoints(1410), PowerPoints::max_at_level(8));
        assert_eq!(PowerPoints(1410), PowerPoints::max_at_level(9));
    }

    #[test]
    fn test_to_next_level() {
        assert_eq!(PowerPoints(20), PowerPoints(0).to_next_level());
        assert_eq!(PowerPoints(10), PowerPoints(10).to_next_level());
        assert_eq!(PowerPoints(40), PowerPoints(60).to_next_level());
        assert_eq!(
            PowerPoints(1),
            PowerPoints(PowerPoints::max_power_points().0 - 1).to_next_level()
        );
        assert_eq!(PowerPoints(0), PowerPoints::max_power_points().to_next_level());
    }
}
