/// Minimum number of waves an Individual is initialized with.
pub const MIN_WAVES: u16 = 10;
/// Maximum number of waves an Individual is initialized with.
pub const MAX_WAVES: u16 = 400;
/// Minimum frequency of a wave.
pub const MIN_FREQ: u16 = 20;
/// Maximum frequency of a wave.
pub const MAX_FREQ: u16 = 20000;

/// Waveform(sound data) length in seconds
pub const WAVE_LENGTH: u16 = 1;
/// Waveform(sound data) frequency. How many samples per second.
pub const WAVE_FREQ: u16 = 16000;
pub const WAVE_LENGTH_SAMPLES: u16 = WAVE_LENGTH * WAVE_FREQ;

/// Target word we are trying to make voice for
pub const TARGET_WORD: &str = "hello";

/// Maximum LV distance
pub const MAXIMUM_DISTANCE: u16 = 1000;
