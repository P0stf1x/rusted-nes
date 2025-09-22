#![allow(dead_code)] // FIXME

#[derive(Debug)]
pub struct Settings {
    pub clock_delta: f64, // clock delta in nanosecs
    pub emulation_speed: f64,
}

impl Settings {
    #[allow(dead_code)]
    pub fn update() {
        todo!(); // TODO: impl observer pattern from centralised settings provider
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings{
            emulation_speed: 1.0,
            clock_delta: 558.73, // ~1.789773 MHz NTSC NES
        }
    }
}
