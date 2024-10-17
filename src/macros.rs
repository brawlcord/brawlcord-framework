#[allow(unused)]
const END_DOC: &str = "Returns the unit at which the next tier begins.";
#[allow(unused)]
const CAN_ADVANCE_DOC: &str = "Checks if the given units can advance to the next tier.";

/// Implements helper functions for structs representing a start-progress tier system.
///
/// The first argument is the struct itself, and the second argument is the type
/// of the tier unit. These two arguments are required.
///
/// The struct passed must have fields `start` and `progress` of type `unit`.
///
/// The following two arguments are used as doc comments for the `end` and
/// `can_advance` functions respectively. These two arguments are optional.
/// Default doc comments are added for the functions if they are not specified.
macro_rules! impl_tier {
    ($tier:ident, $unit:ty) => {
        impl_tier!($tier, $unit, END_DOC, CAN_ADVANCE_DOC)
    };
    ($tier:ident, $unit:ty, $end_doc:literal) => {
        impl_tier!($tier, $unit, $end_doc, CAN_ADVANCE_DOC)
    };
    ($tier:ident, $unit:ty, $end_doc:literal, $can_advance_doc:literal) => {
        impl $tier {
            #[doc = $end_doc]
            pub fn end(&self) -> $unit {
                self.start + self.progress
            }

            #[doc = $can_advance_doc]
            pub fn can_advance(&self, units: $unit) -> bool {
                units >= self.end()
            }
        }
    };
}

// #[allow(unused)]
macro_rules! impl_tier_manager {
    ($tier_manager:ident, $tier:ident) => {
        impl $tier_manager {
            /// Creates a new tier manager from unsorted tiers.
            ///
            /// The data is sorted by the `start` of each tier. You must ensure
            /// that the data is valid, that is, the end of a tier coincides with the
            /// start of the next tier after the tiers are sorted. An invalid tier
            /// manager maybe be created if the data is invalid. You can check for
            /// the validity of the generated tier manager using the [`is_valid`]
            /// method. You may also try to create a tier manager using the
            /// [`try_from_unsorted`] method, which will automatically check for
            /// the validity.
            ///
            /// Use the [`from_sorted`] method if the data is already sorted.
            ///
            /// [`from_sorted`]: Self::from_sorted
            /// [`try_from_unsorted`]: Self::try_from_unsorted
            /// [`is_valid`]: Self::is_valid
            pub fn from_unsorted(mut tiers: Vec<$tier>) -> Self {
                tiers.sort_unstable_by_key(|t| t.start);

                Self(tiers)
            }

            /// Creates a new tier manager from sorted tiers.
            ///
            /// The data must be sorted by the `start` of each tier. You must ensure
            /// that the data is valid, that is, the end of a tier coincides with the
            /// start of the next tier. An invalid tier manager maybe be created if
            /// the data is invalid. You can check for the validity of the generated tier
            /// manager using the [`is_valid`] method. You may also try to create a tier
            /// manager using the [`try_from_sorted`] method, which will automatically
            /// check for the validity.
            ///
            /// Use the [`from_unsorted`] method if the data is not already sorted.
            ///
            /// [`from_unsorted`]: Self::from_unsorted
            /// [`try_from_sorted`]: Self::try_from_sorted
            /// [`is_valid`]: Self::is_valid
            pub fn from_sorted(tiers: Vec<$tier>) -> Self {
                Self(tiers)
            }

            /// Tries to create a new tier manager from unsorted tiers.
            ///
            /// This is equivalent to creating a tier manager using the [`from_unsorted`]
            /// method and then verifying its validity using the [`is_valid`] method.
            ///
            /// Returns `None` if the tier manager is not valid.
            ///
            /// [`from_unsorted`]: Self::from_unsorted
            /// [`is_valid`]: Self::is_valid
            pub fn try_from_unsorted(tiers: Vec<$tier>) -> Option<Self> {
                let tier_manager = Self::from_sorted(tiers);

                tier_manager.is_valid().then(|| tier_manager)
            }

            /// Tries to create a new tier manager from sorted tiers.
            ///
            /// This is equivalent to creating a tier manager using the [`from_sorted`]
            /// method and then verifying its validity using the [`is_valid`] method.
            ///
            /// Returns `None` if the tier manager is not valid.
            ///
            /// [`from_sorted`]: Self::from_sorted
            /// [`is_valid`]: Self::is_valid
            pub fn try_from_sorted(tiers: Vec<$tier>) -> Option<Self> {
                let tier_manager = Self::from_sorted(tiers);

                tier_manager.is_valid().then(|| tier_manager)
            }

            /// Checks if the tier manager is valid.
            ///
            /// A valid tier manager must have tiers in an order such that
            /// the end of a tier coincides with the start of the next tier.
            pub fn is_valid(&self) -> bool {
                for tiers in self.0.windows(2) {
                    if tiers[0].end() != tiers[1].start {
                        return false;
                    }
                }

                true
            }

            /// Returns a reference to the tier at given index.
            pub fn get(&self, index: usize) -> Option<&$tier> {
                self.0.get(index)
            }

            /// Returns a mutable reference to the tier at given index.
            pub fn get_mut(&mut self, index: usize) -> Option<&mut $tier> {
                self.0.get_mut(index)
            }

            /// Returns the tier if a Brawler with given units can advance any tier.
            ///
            /// Returns None if the units are not sufficient for advancing any tier.
            ///
            /// ## Note
            ///
            /// This function may exhibit incorrect behavior if the tiers are not in
            /// correct order. If you've mutated the tiers, then you can use the [`is_valid`]
            /// method to verify if the order is still correct.
            ///
            /// [`is_valid`]: Self::is_valid
            pub fn advance_rank(&self, trophies: u32) -> Option<&$tier> {
                let mut difference = u32::MAX;
                let mut previous = None;

                for tier in &self.0 {
                    let end = tier.end();
                    if trophies >= end {
                        let current_difference = trophies - end;
                        if difference < current_difference {
                            return previous;
                        }

                        difference = current_difference;
                        previous = Some(tier);
                    }
                }

                previous
            }

            /// Returns a slice of all the tiers present in the manager.
            pub fn tiers(&self) -> &[$tier] {
                self.0.as_slice()
            }

            /// Returns a mutable slice of all the tiers present in the manager.
            pub fn tiers_mut(&mut self) -> &mut [$tier] {
                self.0.as_mut_slice()
            }

            /// Returns the tier corresponding to the provided units.
            ///
            /// Returns `None` if the units are insufficient for all tier or if the
            /// tier manager has zero tiers.
            ///
            /// ## Note
            ///
            /// This function may exhibit incorrect behavior if the tier are not in
            /// correct order. If you've mutated the tiers, then you can use the [`is_valid`]
            /// method to verify if the order is still correct.
            ///
            /// [`is_valid`]: Self::is_valid
            pub fn tier_from_units(&self, units: u32) -> Option<&$tier> {
                for tier in &self.0 {
                    if units >= tier.start && units < tier.end() {
                        return Some(tier);
                    }
                }

                None
            }
        }
    };
}
