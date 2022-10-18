use levenshtein_diff as lv;
use std::collections::HashSet;
use tinyrand::{Probability, Rand, RandRange, StdRand};
use vosk::Recognizer;

const WORST_FITNESS: u16 = crate::consts::MAXIMUM_DISTANCE * 3 + crate::consts::MAX_WAVES;

pub fn normalize_fitness(ft: u16) -> f32 {
    ft as f32 / WORST_FITNESS as f32
}

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
            let start: u16 = rng.next_range(0..crate::consts::WAVE_LENGTH_SAMPLES);
            let length: u16 = rng.next_range(start..crate::consts::WAVE_LENGTH_SAMPLES);
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
        normalize_fitness((ft * 3 + self.waves.len() as u16) / 4)
    }

    pub fn combine(&self, other: &Individual, rng: &mut StdRand) -> (Individual, Individual) {
        let point = rng.next_lim_u16(crate::consts::WAVE_LENGTH * crate::consts::WAVE_FREQ);
        let mut ind0 = Individual {
            waves: HashSet::new(),
        };
        let mut ind1 = Individual {
            waves: HashSet::new(),
        };
        for wave in self.waves.iter() {
            if wave.start <= point {
                ind0.waves.insert(wave.clone());
            } else {
                ind1.waves.insert(wave.clone());
            }
        }
        for wave in other.waves.iter() {
            if wave.start <= point {
                ind1.waves.insert(wave.clone());
            } else {
                ind0.waves.insert(wave.clone());
            }
        }
        (ind0, ind1)
    }

    pub fn mutate(&mut self, chance: f32, rng: &mut StdRand) {
        let mut to_be_mutated: Vec<(crate::sinewave::SineWave, crate::sinewave::SineWave)> = vec![];
        for sine in self.waves.iter() {
            if rng.next_bool(Probability::new(chance as f64)) {
                let new_sine = crate::sinewave::SineWave {
                    start: sine.start,
                    length: sine.length,
                    frequency: rng.next_range(crate::consts::MIN_FREQ..crate::consts::MAX_FREQ),
                };
                to_be_mutated.push((new_sine, *sine));
            }
        }
        for (new, old) in to_be_mutated.iter() {
            self.waves.remove(old);
            self.waves.insert(*new);
        }
    }

    pub fn to_wave(&self) -> crate::waveform::Waveform {
        let mut waveforms: Vec<crate::waveform::Waveform> = vec![];
        for sine in self.waves.iter() {
            waveforms.push(sine.to_wave());
        }
        crate::waveform::Waveform::combine(waveforms)
    }
}
