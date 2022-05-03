use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired, AudioStatus};
use sdl2::Sdl;

/// A wave which amplitude alternates at a steady frequency.
/// Useful for wwitching cirtuits with two-level logic (0/1).
pub struct SquareWave {
    phase: f32,
    phase_inc: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            if self.phase <= 0.5 {
                *x = self.volume;
            } else {
                *x = -self.volume;
            }

            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Audio {
    device: AudioDevice<SquareWave>,
}

impl Audio {
    pub fn new(sdl: &Sdl) -> Self {
        let subsystem = sdl
            .audio()
            .expect("Failed to instantiate `AudioSubsystem`.");
        let spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None,
        };

        let device = subsystem
            .open_playback(None, &spec, |spec| SquareWave {
                phase: 0.0,
                phase_inc: 440.0 / spec.freq as f32,
                volume: 0.2,
            })
            .expect("Failed to create an instance of `AudioDevice`.");

        Self { device }
    }

    pub fn play(&self) {
        let status = self.device.status();

        if status == AudioStatus::Stopped || status == AudioStatus::Paused {
            self.device.resume();
        }
    }

    pub fn stop(&self) {
        self.device.pause();
    }
}
