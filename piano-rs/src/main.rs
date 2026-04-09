mod channel;
mod config;
mod midi;
mod models;
mod notes_manager;
mod piano;

use accesskit::{Live, NodeBuilder, NodeId, Role, Tree, TreeUpdate};
use accesskit_winit::Adapter;
use anyhow::Result;
use std::num::NonZeroU32;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use config::{find_config_file, Config};
use midi::driver_midir::MidirOutput;
use piano::Piano;

const ROOT_ID:   NodeId = NodeId(0);
const STATUS_ID: NodeId = NodeId(1);
const HINTS_ID:  NodeId = NodeId(2);

#[derive(Debug)]
enum AppEvent {
    SetInstrument { channel: u8, instrument: u8 },
    AccessKit(accesskit_winit::Event),
}

impl From<accesskit_winit::Event> for AppEvent {
    fn from(e: accesskit_winit::Event) -> Self { AppEvent::AccessKit(e) }
}

const GM_INSTRUMENTS: [&str; 128] = [
    "Acoustic Grand Piano", "Bright Acoustic Piano", "Electric Grand Piano",
    "Honky-tonk Piano", "Electric Piano 1", "Electric Piano 2", "Harpsichord", "Clavi",
    "Celesta", "Glockenspiel", "Music Box", "Vibraphone", "Marimba", "Xylophone",
    "Tubular Bells", "Dulcimer",
    "Drawbar Organ", "Percussive Organ", "Rock Organ", "Church Organ", "Reed Organ",
    "Accordion", "Harmonica", "Tango Accordion",
    "Nylon Guitar", "Steel Guitar", "Jazz Guitar", "Clean Guitar", "Muted Guitar",
    "Overdriven Guitar", "Distortion Guitar", "Guitar Harmonics",
    "Acoustic Bass", "Finger Bass", "Pick Bass", "Fretless Bass",
    "Slap Bass 1", "Slap Bass 2", "Synth Bass 1", "Synth Bass 2",
    "Violin", "Viola", "Cello", "Contrabass", "Tremolo Strings",
    "Pizzicato Strings", "Orchestral Harp", "Timpani",
    "String Ensemble 1", "String Ensemble 2", "Synth Strings 1", "Synth Strings 2",
    "Choir Aahs", "Voice Oohs", "Synth Voice", "Orchestra Hit",
    "Trumpet", "Trombone", "Tuba", "Muted Trumpet", "French Horn",
    "Brass Section", "Synth Brass 1", "Synth Brass 2",
    "Soprano Sax", "Alto Sax", "Tenor Sax", "Baritone Sax",
    "Oboe", "English Horn", "Bassoon", "Clarinet",
    "Piccolo", "Flute", "Recorder", "Pan Flute", "Blown Bottle",
    "Shakuhachi", "Whistle", "Ocarina",
    "Square Lead", "Sawtooth Lead", "Calliope Lead", "Chiff Lead",
    "Charang Lead", "Voice Lead", "Fifths Lead", "Bass+Lead",
    "New Age Pad", "Warm Pad", "Polysynth Pad", "Choir Pad",
    "Bowed Pad", "Metallic Pad", "Halo Pad", "Sweep Pad",
    "Rain", "Soundtrack", "Crystal", "Atmosphere", "Brightness",
    "Goblins", "Echoes", "Sci-fi",
    "Sitar", "Banjo", "Shamisen", "Koto", "Kalimba",
    "Bagpipe", "Fiddle", "Shanai",
    "Tinkle Bell", "Agogo", "Steel Drums", "Woodblock", "Taiko Drum",
    "Melodic Tom", "Synth Drum", "Reverse Cymbal",
    "Guitar Fret Noise", "Breath Noise", "Seashore", "Bird Tweet",
    "Telephone Ring", "Helicopter", "Applause", "Gunshot",
];

fn instrument_name(number: u8) -> &'static str {
    GM_INSTRUMENTS[number as usize]
}

const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

fn note_name(number: u8) -> String {
    let name = NOTE_NAMES[(number % 12) as usize];
    let octave = number / 12;
    format!("{}{}", name, octave)
}

const HINTS_TEXT: &str = "PgUp/Dn:canal  Backspace:multi  Delete:rm canal  F2:instrumento  F8/F9:vol  Tab:pan  Setas:oitava/inst  Shift+Seta:semitom";

fn build_tree_update(status: &str, hints: &str, initial: bool) -> TreeUpdate {
    let mut root_b = NodeBuilder::new(Role::Window);
    root_b.set_name("Virtual Piano");
    root_b.set_children(vec![STATUS_ID, HINTS_ID]);

    let mut status_b = NodeBuilder::new(Role::Label);
    status_b.set_value(status);
    status_b.set_live(Live::Polite);

    let mut hints_b = NodeBuilder::new(Role::Label);
    hints_b.set_value(hints);

    TreeUpdate {
        nodes: vec![
            (ROOT_ID, root_b.build()),
            (STATUS_ID, status_b.build()),
            (HINTS_ID, hints_b.build()),
        ],
        tree:  if initial { Some(Tree::new(ROOT_ID)) } else { None },
        focus: ROOT_ID,
    }
}

fn draw_text(buf: &mut [u32], stride: usize, height: usize, x: usize, y: usize, text: &str, color: u32) {
    use font8x8::UnicodeFonts;
    let mut cx = x;
    for ch in text.chars() {
        if cx + 8 > stride { break; }
        if let Some(glyph) = font8x8::BASIC_FONTS.get(ch) {
            for (row, &byte) in glyph.iter().enumerate() {
                let py = y + row;
                if py >= height { break; }
                for col in 0..8usize {
                    if byte & (1 << col) != 0 {
                        let px = cx + col;
                        if px < stride {
                            buf[py * stride + px] = color;
                        }
                    }
                }
            }
        }
        cx += 9;
    }
}

struct App {
    piano: Piano,
    window: Option<Arc<Window>>,
    surface: Option<softbuffer::Surface<Arc<Window>, Arc<Window>>>,
    adapter: Option<Adapter>,
    shift_pressed: bool,
    proxy: EventLoopProxy<AppEvent>,
    dialog_open: bool,
    current_channel: u8,
    active_channels: Vec<u8>,
    multi_voice: bool,
}

impl App {
    fn new(piano: Piano, proxy: EventLoopProxy<AppEvent>) -> Self {
        App {
            piano,
            window: None,
            surface: None,
            adapter: None,
            shift_pressed: false,
            proxy,
            dialog_open: false,
            current_channel: 0,
            active_channels: vec![0],
            multi_voice: false,
        }
    }

    fn request_redraw(&mut self) {
        if let Some(w) = &self.window {
            w.request_redraw();
        }
        let status = self.build_status_line();
        let hints  = HINTS_TEXT;
        if let Some(adapter) = &mut self.adapter {
            adapter.update_if_active(|| build_tree_update(&status, hints, false));
        }
    }

    fn build_status_line(&self) -> String {
        let ch = self.current_channel;
        let note  = note_name(self.piano.get_start_note(ch));
        let instr = self.piano.get_instrument(ch);
        let vol   = self.piano.get_volume(ch);
        let pan   = self.piano.get_direction(ch).label();
        let multi = if self.multi_voice { "  [MULTI-VOICE]" } else { "" };
        format!(
            "Canal:{ch}{multi}   [{note}]   #{n}: {name}   Vol:{vol}   Pan:{pan}",
            n    = instr + 1,
            name = instrument_name(instr),
        )
    }

    fn render(&mut self) {
        let status = self.build_status_line();

        let Some(window)  = &self.window  else { return };
        let Some(surface) = &mut self.surface else { return };

        let size   = window.inner_size();
        let width  = size.width;
        let height = size.height;
        if width == 0 || height == 0 { return; }

        let _ = surface.resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let Ok(mut buf) = surface.buffer_mut() else { return };
        let w = width as usize;
        let h = height as usize;

        buf.fill(0xFF0F0F1A);

        let bar_h = 52usize;
        let bar_y = h.saturating_sub(bar_h);
        let sep_y = bar_y.saturating_sub(1);

        for y in bar_y..h {
            for x in 0..w {
                buf[y * w + x] = 0xFF1A1A2E;
            }
        }
        if sep_y < h {
            for x in 0..w {
                buf[sep_y * w + x] = 0xFF3355AA;
            }
        }

        draw_text(&mut buf, w, h, 8, bar_y + 8,  &status,    0xFFE8E8FF);
        draw_text(&mut buf, w, h, 8, bar_y + 26, HINTS_TEXT, 0xFF6666AA);

        let _ = buf.present();
    }

    fn next_channel(&mut self) {
        if self.current_channel >= 15 { return; }
        self.piano.all_notes_off();
        self.current_channel += 1;
        if !self.active_channels.contains(&self.current_channel) {
            self.active_channels.push(self.current_channel);
            self.piano.ensure_channel(self.current_channel);
        }
        self.request_redraw();
    }

    fn previous_channel(&mut self) {
        if self.current_channel == 0 { return; }
        self.piano.all_notes_off();
        self.current_channel -= 1;
        self.request_redraw();
    }

    fn delete_current_channel(&mut self) {
        if self.current_channel == 0 { return; }
        self.piano.delete_channel(self.current_channel);
        self.active_channels.retain(|&c| c != self.current_channel);
        self.current_channel -= 1;
        self.request_redraw();
    }

    fn toggle_multi_voice(&mut self) {
        self.piano.all_notes_off();
        self.multi_voice = !self.multi_voice;
        self.request_redraw();
    }

    fn open_instrument_dialog(&mut self) {
        if self.dialog_open { return; }
        self.dialog_open = true;

        let proxy        = self.proxy.clone();
        let channel      = self.current_channel;
        let current      = self.piano.get_instrument(channel) + 1;
        let current_name = instrument_name(self.piano.get_instrument(channel)).to_owned();

        std::thread::spawn(move || {
            let default_input = current.to_string();
            loop {
                let input = tinyfiledialogs::input_box(
                    "Selecionar Instrumento",
                    &format!("Numero do instrumento (1-128):\n(atual: #{current} - {current_name})"),
                    &default_input,
                );
                match input {
                    None => {
                        let _ = proxy.send_event(AppEvent::SetInstrument { channel, instrument: u8::MAX });
                        return;
                    }
                    Some(raw) => match raw.trim().parse::<i32>() {
                        Err(_) => {
                            tinyfiledialogs::message_box_ok(
                                "Valor invalido",
                                "Digite apenas numeros inteiros.",
                                tinyfiledialogs::MessageBoxIcon::Warning,
                            );
                        }
                        Ok(n) if n < 1 || n > 128 => {
                            tinyfiledialogs::message_box_ok(
                                "Fora do intervalo",
                                "O numero do instrumento deve estar entre 1 e 128.",
                                tinyfiledialogs::MessageBoxIcon::Warning,
                            );
                        }
                        Ok(n) => {
                            let _ = proxy.send_event(AppEvent::SetInstrument { channel, instrument: (n - 1) as u8 });
                            return;
                        }
                    },
                }
            }
        });
    }

    fn handle_note_on(&mut self, key: &str) {
        if self.multi_voice {
            for &ch in &self.active_channels.clone() {
                self.piano.note_on(key, ch);
            }
        } else {
            self.piano.note_on(key, self.current_channel);
        }
    }

    fn handle_note_off(&mut self, key: &str) {
        if self.multi_voice {
            for &ch in &self.active_channels.clone() {
                self.piano.note_off(key, ch);
            }
        } else {
            self.piano.note_off(key, self.current_channel);
        }
    }
}

impl ApplicationHandler<AppEvent> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attrs = WindowAttributes::default()
            .with_title("Virtual Piano")
            .with_inner_size(winit::dpi::LogicalSize::new(820u32, 120u32))
            .with_min_inner_size(winit::dpi::LogicalSize::new(400u32, 80u32))
            .with_visible(false);

        match event_loop.create_window(attrs) {
            Err(e) => { eprintln!("Erro ao criar janela: {e}"); event_loop.exit(); }
            Ok(window) => {
                let window = Arc::new(window);
                let context = softbuffer::Context::new(window.clone()).expect("softbuffer context");
                let surface = softbuffer::Surface::new(&context, window.clone()).expect("softbuffer surface");
                let adapter = Adapter::with_event_loop_proxy(&window, self.proxy.clone());
                window.set_visible(true);
                self.window  = Some(window);
                self.surface = Some(surface);
                self.adapter = Some(adapter);
                self.render();
            }
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: AppEvent) {
        match event {
            AppEvent::SetInstrument { channel, instrument } => {
                self.dialog_open = false;
                if instrument != u8::MAX {
                    self.piano.set_instrument(instrument, channel);
                    self.request_redraw();
                }
            }
            AppEvent::AccessKit(ak_event) => {
                if let accesskit_winit::WindowEvent::InitialTreeRequested = ak_event.window_event {
                    let status = self.build_status_line();
                    if let Some(adapter) = &mut self.adapter {
                        adapter.update_if_active(|| build_tree_update(&status, HINTS_TEXT, true));
                    }
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let (Some(adapter), Some(window)) = (&mut self.adapter, &self.window) {
            adapter.process_event(window, &event);
        }

        match event {
            WindowEvent::CloseRequested => {
                self.piano.all_notes_off();
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.render();
            }

            WindowEvent::Resized(_) => {
                self.request_redraw();
            }

            WindowEvent::ModifiersChanged(modifiers) => {
                self.shift_pressed = modifiers.state().shift_key();
            }

            WindowEvent::KeyboardInput { event, is_synthetic, .. } => {
                if is_synthetic || event.repeat { return; }
                let pressed = event.state == ElementState::Pressed;
                let ch = self.current_channel;

                match &event.logical_key {
                    Key::Named(NamedKey::ArrowUp) if pressed => {
                        if self.shift_pressed { self.piano.semitone_up(1, ch); }
                        else                  { self.piano.octave_up(ch); }
                        self.request_redraw();
                    }
                    Key::Named(NamedKey::ArrowDown) if pressed => {
                        if self.shift_pressed { self.piano.semitone_down(1, ch); }
                        else                  { self.piano.octave_down(ch); }
                        self.request_redraw();
                    }
                    Key::Named(NamedKey::ArrowRight) if pressed => {
                        self.piano.next_instrument(ch);
                        self.request_redraw();
                    }
                    Key::Named(NamedKey::ArrowLeft) if pressed => {
                        self.piano.previous_instrument(ch);
                        self.request_redraw();
                    }
                    Key::Named(NamedKey::PageUp)    if pressed => { self.next_channel(); }
                    Key::Named(NamedKey::PageDown)  if pressed => { self.previous_channel(); }
                    Key::Named(NamedKey::Delete)    if pressed => { self.delete_current_channel(); }
                    Key::Named(NamedKey::Backspace) if pressed => { self.toggle_multi_voice(); }
                    Key::Named(NamedKey::F2)        if pressed => { self.open_instrument_dialog(); }
                    Key::Named(NamedKey::F8)        if pressed => {
                        self.piano.volume_down(ch);
                        self.request_redraw();
                    }
                    Key::Named(NamedKey::F9)        if pressed => {
                        self.piano.volume_up(ch);
                        self.request_redraw();
                    }
                    Key::Named(NamedKey::Tab)       if pressed => {
                        self.piano.pan(!self.shift_pressed, ch);
                        self.request_redraw();
                    }
                    Key::Character(s) => {
                        let k = s.to_uppercase().to_string();
                        if pressed { self.handle_note_on(&k); } else { self.handle_note_off(&k); }
                    }
                    Key::Dead(Some(c)) => {
                        let k = c.to_uppercase().to_string();
                        if pressed { self.handle_note_on(&k); } else { self.handle_note_off(&k); }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

fn main() -> Result<()> {
    let config_path = find_config_file();
    let config = match Config::load(&config_path) {
        Ok(c)  => c,
        Err(e) => {
            eprintln!("Aviso: nao foi possivel ler config.ini ({e}). Usando padroes.");
            Config::default()
        }
    };

    let keymap_path = config.get_keymap_file_path();
    println!("Keymap: {}", keymap_path.display());

    let midi_output = MidirOutput::new()?;
    let mut piano = Piano::new(&keymap_path, Box::new(midi_output))?;
    piano.set_instrument(0, 0);

    println!("Piano iniciado.");

    let event_loop = EventLoop::<AppEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let proxy = event_loop.create_proxy();
    let mut app = App::new(piano, proxy);
    event_loop.run_app(&mut app)?;

    Ok(())
}
