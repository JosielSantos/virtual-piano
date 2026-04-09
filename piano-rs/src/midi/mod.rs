pub mod driver_midir;
pub mod driver_rustysynth;

pub trait MidiDriver: Send {
    fn note_on(&mut self, note: u8, velocity: u8, channel: u8);
    fn note_off(&mut self, note: u8, channel: u8);
    fn set_instrument(&mut self, instrument: u8, channel: u8);
    fn control_change(&mut self, control: u8, value: u8, channel: u8);
}
