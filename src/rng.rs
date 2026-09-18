use std::sync::Mutex;
use rltk::RandomNumberGenerator;

lazy_static! {
    static ref RNG: Mutex<RandomNumberGenerator> =
        Mutex::new(RandomNumberGenerator::new());
}

/// This function re-initializes the `RandomNumberGenerator` with a new seed value.
/// 
/// #Arguments:
/// `seed`: `u64` value to use as the new seed
pub fn reseed(seed: u64) {
    *RNG.lock().unwrap() = RandomNumberGenerator::seeded(seed);
}

/// This function rolls DnD style dice for a random result.
/// 
/// #Arguments
/// -`n`: the number of times to roll a die
/// -`die_type`: how many sides each die has
/// 
/// #Return
/// `i32`: Additive integer of the random 'rolls'
pub fn roll_dice(n:i32, die_type: i32) -> i32 {
    RNG.lock().unwrap().roll_dice(n, die_type)
}

/// This function returns a random number that is between two other integers
/// 
/// #Arguments
/// -`min`: lower value for the range
/// -`max`: upper value for the range
pub fn range(min: i32, max: i32) -> i32
{
    RNG.lock().unwrap().range(min, max)
}