use levenshtein_diff as lv;
use std::collections::HashSet;
use tinyrand::{Probability, Rand, RandRange, StdRand};
use vosk::Recognizer;

const WORST_FITNESS: u16 = crate::consts::MAXIMUM_DISTANCE * 3 + crate::consts::MAX_WAVES;


pub struct Individual {
    pub waves: HashSet<crate::sinewave::SineWave>,
}

impl Individual {
    pub fn new_rand(rng: &mut StdRand) -> Self {
        let mut ind = Self {
            waves: HashSet::new(),
        };
        for _ in 0..rng.next_range(crate::consts::MIN_WAVES..crate::consts::MAX_WAVES) {
            let freq: u16 = rng.next_range(crate::consts::MIN_FREQ..crate::consts::MAX_FREQ);
            let start: u16 = rng.next_lim_u16(crate::consts::WAVE_LENGTH_SAMPLES);
            let length: u16 = rng.next_range(crate::consts::WAVE_LENGTH_SAMPLES/50..crate::consts::WAVE_LENGTH_SAMPLES/5);
            ind.waves
                .insert(crate::sinewave::SineWave::new(start, length, freq));
        }
        ind
    }

    pub fn fitness(&self, recognizer: &mut Recognizer) -> f32 {
        let waveform: crate::waveform::Waveform = self.to_wave();
        recognizer.accept_waveform(&waveform.samples);
        let ft: u16 = match recognizer.final_result().single() {
            Some(result) => {
                lv::distance(
                    crate::consts::TARGET_WORD.as_bytes(),
                    result.text.as_bytes(),
                )
                .0 as u16
            }
            None => crate::consts::MAXIMUM_DISTANCE,
        };
        recognizer.reset();
        ft as f32 / crate::consts::MAXIMUM_DISTANCE as f32
    }

    pub fn combine(&self, other: &Individual, rng: &mut StdRand) -> (Individual, Individual) {
        let point = rng.next_lim_u16(crate::consts::WAVE_LENGTH_SAMPLES);
        let mut ind0 = Individual {
            waves: HashSet::new(),
        };
        let mut ind1 = Individual {
            waves: HashSet::new(),
        };
        for wave in self.waves.iter() {
            let wave_end = wave.start + wave.length;
            if wave_end <= point {
                ind0.waves.insert(wave.clone());
            } else if wave.start <= point {
                let mut new_wave_before_point = wave.clone();
                new_wave_before_point.length = point - wave.start;
                let mut new_wave_after_point = wave.clone();
                new_wave_after_point.start = point + 1;
                new_wave_after_point.length -= wave.length - point; 
                ind0.waves.insert(new_wave_before_point);
                ind1.waves.insert(new_wave_after_point);
            } else {
                ind1.waves.insert(wave.clone());
            }
        }
        for wave in other.waves.iter() {
            let wave_end = wave.start + wave.length;
            if wave_end <= point {
                ind1.waves.insert(wave.clone());
            } else if wave.start <= point {
                let mut new_wave_before_point = wave.clone();
                new_wave_before_point.length = point - wave.start;
                let mut new_wave_after_point = wave.clone();
                new_wave_after_point.start = point + 1;
                new_wave_after_point.length -= wave.length - point;
                ind1.waves.insert(new_wave_before_point);
                ind0.waves.insert(new_wave_after_point);
            } else {
                ind0.waves.insert(wave.clone());
            }

        }
        (ind0, ind1)
    }

    pub fn mutate(&mut self, chance: f32, rng: &mut StdRand) -> bool {
        if rng.next_bool(Probability::new(chance as f64)) {
            let freq: u16 = rng.next_range(crate::consts::MIN_FREQ..crate::consts::MAX_FREQ);
            let start: u16 = rng.next_range(0..crate::consts::WAVE_LENGTH_SAMPLES);
            let length: u16 = rng.next_range(start..crate::consts::WAVE_LENGTH_SAMPLES);
            self.waves
                .insert(crate::sinewave::SineWave::new(start, length, freq));
            return true;
        }
        false
    }

    pub fn to_wave(&self) -> crate::waveform::Waveform {
        let waveforms: Vec<crate::waveform::Waveform> = self.waves.iter().map(|sine| sine.to_wave()).collect();
        crate::waveform::Waveform::combine(waveforms)
    }
}
