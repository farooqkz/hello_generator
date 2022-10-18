#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct SineWave {
    /// start of the sine wave in seconds
    pub start: u16,
    /// length of the sine wave in seconds
    pub length: u16,
    /// frequency of the sine wave
    pub frequency: u16,
}

impl SineWave {
    pub fn new(start: u16, length: u16, frequency: u16) -> Self {
        Self {
            length,
            start,
            frequency,
        }
    }

    pub fn to_wave(&self) -> crate::waveform::Waveform {
        let mut wave = crate::waveform::Waveform::zero();
        for t in self.start..self.length {
            let param: f32 = f32::from(2.0 * self.frequency as f32 * (t as f32));
            wave.samples[t as usize] = (param.sin() * (f32::exp2(15.0) - 1.0)) as i16;
        }
        wave
    }
}
