use midir::MidiOutput as MidirLib;
use anyhow::{Context, Result};
use super::MidiOutput;

pub struct MidirOutput {
    conn: midir::MidiOutputConnection,
}

impl MidirOutput {
    pub fn new() -> Result<Self> {
        let midi_out = MidirLib::new("piano-rs")?;
        let ports = midi_out.ports();

        if ports.is_empty() {
            anyhow::bail!(
                "Nenhuma porta MIDI encontrada.\n\
                 Instale um sintetizador MIDI virtual, como:\n\
                 - VirtualMIDISynth (Windows)\n\
                 - Microsoft GS Wavetable Synth (já presente no Windows)"
            );
        }

        println!("Portas MIDI disponíveis:");
        for (i, port) in ports.iter().enumerate() {
            println!("  {}: {}", i, midi_out.port_name(port).unwrap_or_default());
        }

        let port = &ports[0];
        let port_name = midi_out.port_name(port).unwrap_or_default();
        println!("Usando: {}\n", port_name);

        let conn = midi_out
            .connect(port, "piano-rs")
            .context("Falha ao conectar à porta MIDI")?;

        Ok(MidirOutput { conn })
    }
}

impl MidiOutput for MidirOutput {
    fn note_on(&mut self, note: u8, velocity: u8, channel: u8) {
        let _ = self.conn.send(&[0x90 | (channel & 0x0F), note, velocity]);
    }

    fn note_off(&mut self, note: u8, channel: u8) {
        let _ = self.conn.send(&[0x80 | (channel & 0x0F), note, 0]);
    }

    fn set_instrument(&mut self, instrument: u8, channel: u8) {
        let _ = self.conn.send(&[0xC0 | (channel & 0x0F), instrument]);
    }

    fn control_change(&mut self, control: u8, value: u8, channel: u8) {
        let _ = self.conn.send(&[0xB0 | (channel & 0x0F), control, value]);
    }
}
