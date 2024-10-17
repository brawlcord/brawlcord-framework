//! Utility functions related to random numbers are defined here.
//!
//! The functions here are exposed to assist developers in overriding behaviour.

use indexmap::IndexMap;
use rand::distributions::{Distribution, Uniform, WeightedIndex};
use rand::seq::IteratorRandom;
use rand::Rng;

/// Generates a weighted random number between `lower` and `upper`.
///
/// The average of the randomly generated numbers is close to `avg`.
///
/// Panics if `lower > avg` or `avg > upper`.
pub fn weighted_random(lower: u32, upper: u32, avg: u32) -> u32 {
    assert!(lower <= avg && avg <= upper);

    let avg_low = (lower + avg) / 2;
    let avg_high = (avg + upper) / 2;

    let p_high = (avg - avg_low) / (avg_high - avg_low);

    let mut rng = rand::thread_rng();
    let (low, high) = if rng.gen::<u32>() < p_high { (avg, upper) } else { (lower, avg) };

    Uniform::new_inclusive(low, high).sample(&mut rng)
}

/// Randomly splits an integer into `total` integers that add up to it.
///
/// All the numbers in the returned vector are more than or equal to `minimum`.
///
/// Returns an empty vector if `total * minimum > number` or `total = 0`.
pub fn split_in_integers(number: u32, total: u32, minimum: u32) -> Vec<u32> {
    if total * minimum > number || total == 0 {
        return Vec::new();
    } else if number == 0 {
        return vec![0; total as usize];
    }

    let max = number - (total * minimum) + total - 1;
    let mut breaks = (0..max).choose_multiple(&mut rand::thread_rng(), total as usize - 1);
    breaks.sort_unstable();
    breaks.push(max);

    // `breaks` always has at least one element
    let mut buckets = vec![breaks[0] + minimum];
    for window in breaks.windows(2) {
        buckets.push(window[1] - window[0] - 1 + minimum);
    }

    buckets
}

/// Selects a random number from `options` by considering corresponding `weights`.
///
/// Returns `None` if the length of `options` is not equal to the length of `weights`,
/// `weights` is empty or the sum of all `weights` is 0.
pub fn select_one<'a, T>(options: &'a [T], weights: &[u32]) -> Option<&'a T> {
    WeightedIndex::new(weights).ok().and_then(|w| options.get(w.sample(&mut rand::thread_rng())))
}

/// Sample a number uniformly between 0 and `ubound`. Uses 32-bit sampling where
/// possible, primarily in order to produce the same output on 32-bit and 64-bit
/// platforms.
///
/// This function is copied from the [`gen_index`] method in the [`rand`] crate.
///
/// [`gen_index`]: https://docs.rs/rand/0.8.3/src/rand/seq/mod.rs.html#659
/// [`rand`]: https://docs.rs/rand/0.8.3/rand/
#[inline]
fn gen_index<R: Rng + ?Sized>(rng: &mut R, ubound: usize) -> usize {
    if ubound <= (core::u32::MAX as usize) {
        rng.gen_range(0..ubound as u32) as usize
    } else {
        rng.gen_range(0..ubound)
    }
}

/// Shuffles an [`IndexMap`] in place.
///
/// This function is ported from the [`shuffle`] method in the [`rand`] crate.
///
/// [`shuffle`]: https://docs.rs/rand/0.8.3/src/rand/seq/mod.rs.html#586
/// [`rand`]: https://docs.rs/rand/0.8.3/rand/
pub(crate) fn shuffle_index_map<R, K, V>(map: &mut IndexMap<K, V>, rng: &mut R)
where
    R: Rng + ?Sized,
{
    for i in (1..map.len()).rev() {
        // invariant: elements with index > i have been locked in place.
        map.swap_indices(i, gen_index(rng, i + 1));
    }
}
