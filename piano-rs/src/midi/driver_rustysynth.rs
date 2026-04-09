use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::sync::{Arc, Mutex};

use super::MidiDriver;

pub struct RustySynthDriver {
    synth: Arc<Mutex<Synthesizer>>,
    _stream: cpal::Stream,
}

unsafe impl Send for RustySynthDriver {}

impl RustySynthDriver {
    pub fn new(soundfont_path: &str) -> Result<Self> {
        let mut sf2 = std::fs::File::open(soundfont_path)
            .with_context(|| format!("Nao foi possivel abrir soundfont: {soundfont_path}"))?;
        let sound_font = Arc::new(SoundFont::new(&mut sf2)
            .context("Falha ao carregar soundfont")?);

        let host = cpal::default_host();
        let device = host.default_output_device()
            .context("Nenhum dispositivo de audio encontrado")?;
        let config = device.default_output_config()
            .context("Nenhuma configuracao de audio disponivel")?;

        let sample_rate = config.sample_rate().0 as i32;
        let channels   = config.channels() as usize;

        println!("RustySynth: dispositivo={}", device.name().unwrap_or_default());
        println!("RustySynth: sample_rate={sample_rate}  channels={channels}");
        println!("RustySynth: soundfont={soundfont_path}");

        let settings = SynthesizerSettings::new(sample_rate);
        let synthesizer = Synthesizer::new(&sound_font, &settings)
            .context("Falha ao criar sintetizador")?;
        let synth = Arc::new(Mutex::new(synthesizer));

        let synth_cb = Arc::clone(&synth);
        let stream_config: cpal::StreamConfig = config.into();

        let stream = device.build_output_stream(
            &stream_config,
            move |data: &mut [f32], _| {
                let frames = data.len() / channels;
                let mut left  = vec![0.0f32; frames];
                let mut right = vec![0.0f32; frames];
                if let Ok(mut s) = synth_cb.try_lock() {
                    s.render(&mut left, &mut right);
                }
                for i in 0..frames {
                    if channels >= 2 {
                        data[i * channels]     = left[i];
                        data[i * channels + 1] = right[i];
                    } else {
                        data[i] = (left[i] + right[i]) * 0.5;
                    }
                }
            },
            |err| eprintln!("Erro no stream de audio: {err}"),
            None,
        ).context("Falha ao criar stream de audio")?;

        stream.play().context("Falha ao iniciar stream de audio")?;

        Ok(RustySynthDriver { synth, _stream: stream })
    }
}

impl MidiDriver for RustySynthDriver {
    fn note_on(&mut self, note: u8, velocity: u8, channel: u8) {
        if let Ok(mut s) = self.synth.lock() {
            s.note_on(channel as i32, note as i32, velocity as i32);
        }
    }

    fn note_off(&mut self, note: u8, channel: u8) {
        if let Ok(mut s) = self.synth.lock() {
            s.note_off(channel as i32, note as i32);
        }
    }

    fn set_instrument(&mut self, instrument: u8, channel: u8) {
        if let Ok(mut s) = self.synth.lock() {
            s.process_midi_message(channel as i32, 0xC0, instrument as i32, 0);
        }
    }

    fn control_change(&mut self, control: u8, value: u8, channel: u8) {
        if let Ok(mut s) = self.synth.lock() {
            s.process_midi_message(channel as i32, 0xB0, control as i32, value as i32);
        }
    }
}
