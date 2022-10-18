const SAMPLES: usize = (crate::consts::WAVE_LENGTH * crate::consts::WAVE_FREQ) as usize;

pub struct Waveform {
    pub samples: [i16; SAMPLES]
}

impl Waveform {
    pub fn zero() -> Self {
        Self {
            samples: [0; SAMPLES] 
        }
    }

    pub fn combine(waveforms: Vec<Waveform>) -> Waveform {
        let mut samples: [i32; SAMPLES] = [0; SAMPLES];
        for wave in waveforms.iter() {
            for (sample0, sample1) in wave.samples.iter().zip(samples.iter_mut()) {
                *sample1 += *sample0 as i32;
            }
        }
        for sample in samples.iter_mut() {
            *sample /= waveforms.len() as i32;
        }
        let mut result = Self::zero();
        let mut i: usize = 0;
        for sample in result.samples.iter_mut() {
            *sample = samples[i] as i16;
            i += 1;
        }
        result
    }
}

