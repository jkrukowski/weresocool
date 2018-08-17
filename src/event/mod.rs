use instrument::{oscillator::Oscillator, stereo_waveform::StereoWaveform};
use ratios::{Pan, R};

#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub sounds: Vec<Sound>,
    pub length: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sound {
    pub frequency: f32,
    pub gain: f32,
    pub pan: f32,
}

impl Sound {
    pub fn init() -> Sound {
        Sound {
            frequency: 0.0,
            gain: 0.0,
            pan: 0.0,
        }
    }
}

impl Event {
    pub fn init(frequency: f32, gain: f32, pan: f32, length: f32) -> Event {
        Event {
            sounds: {
                vec![Sound {
                    frequency,
                    gain,
                    pan,
                }]
            },
            length,
        }
    }
}

pub trait Render<T> {
    fn render(&mut self, oscillator: &mut Oscillator) -> StereoWaveform;
}

impl Render<Event> for Event {
    fn render(&mut self, oscillator: &mut Oscillator) -> StereoWaveform {
        oscillator.update(self.sounds.clone());
        println!("{}", self.length);
        let n_samples_to_generate = (self.length * 44_100.0).floor() as usize;
        oscillator.generate(n_samples_to_generate)
    }
}

impl Render<Vec<Event>> for Vec<Event> {
    fn render(&mut self, oscillator: &mut Oscillator) -> StereoWaveform {
        let mut result: StereoWaveform = StereoWaveform::new(0);
        let mut events = self.clone();
        events.push(Event::init(0.0, 0.0, 0.0, 2.0));
        for mut event in events {
            let stereo_waveform = event.render(oscillator);
            result.append(stereo_waveform);
        }

        result
    }
}

#[cfg(test)]
mod test;
