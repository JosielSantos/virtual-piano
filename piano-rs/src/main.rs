mod channel;
mod config;
mod midi;
mod models;
mod notes_manager;
mod piano;

use anyhow::Result;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    keyboard::{Key, NamedKey},
    window::{Window, WindowAttributes, WindowId},
};

use config::{find_config_file, Config};
use midi::driver_rustysynth::RustySynthDriver;
use midi::driver_midir::MidirDriver;
use piano::Piano;

#[derive(Debug)]
enum AppEvent {
    SetInstrument { channel: u8, instrument: u8 },
}

const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

fn note_name(number: u8) -> String {
    let name = NOTE_NAMES[(number % 12) as usize];
    let octave = number / 12;
    format!("{}{}", name, octave)
}

struct App {
    piano: Piano,
    window: Option<Window>,
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
            shift_pressed: false,
            proxy,
            dialog_open: false,
            current_channel: 0,
            active_channels: vec![0],
            multi_voice: false,
        }
    }

    fn update_title(&self) {
        let Some(w) = &self.window else { return };
        let ch    = self.current_channel;
        let note  = note_name(self.piano.get_start_note(ch));
        let instr = self.piano.get_instrument(ch);
        let vol   = self.piano.get_volume(ch);
        let pan   = self.piano.get_direction(ch).label();
        let multi = if self.multi_voice { " [MULTI]" } else { "" };
        w.set_title(&format!(
            "Virtual Piano | Ch:{ch}{multi}  [{note}]  #{n}  Vol:{vol}  Pan:{pan}",
            n = instr + 1,
        ));
    }

    fn next_channel(&mut self) {
        if self.current_channel >= 15 { return; }
        self.piano.all_notes_off();
        self.current_channel += 1;
        if !self.active_channels.contains(&self.current_channel) {
            self.active_channels.push(self.current_channel);
            self.piano.ensure_channel(self.current_channel);
        }
        self.update_title();
    }

    fn previous_channel(&mut self) {
        if self.current_channel == 0 { return; }
        self.piano.all_notes_off();
        self.current_channel -= 1;
        self.update_title();
    }

    fn delete_current_channel(&mut self) {
        if self.current_channel == 0 { return; }
        self.piano.delete_channel(self.current_channel);
        self.active_channels.retain(|&c| c != self.current_channel);
        self.current_channel -= 1;
        self.update_title();
    }

    fn toggle_multi_voice(&mut self) {
        self.piano.all_notes_off();
        self.multi_voice = !self.multi_voice;
        self.update_title();
    }

    fn open_instrument_dialog(&mut self) {
        if self.dialog_open { return; }
        self.dialog_open = true;

        let proxy   = self.proxy.clone();
        let channel = self.current_channel;
        let current = self.piano.get_instrument(channel) + 1;

        std::thread::spawn(move || {
            let default_input = current.to_string();
            loop {
                let input = tinyfiledialogs::input_box(
                    "Selecionar Instrumento",
                    &format!("Numero do instrumento (1-128):\n(atual: #{current})"),
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
            .with_inner_size(winit::dpi::LogicalSize::new(400u32, 80u32));

        match event_loop.create_window(attrs) {
            Err(e) => { eprintln!("Erro ao criar janela: {e}"); event_loop.exit(); }
            Ok(window) => {
                self.window = Some(window);
                self.update_title();
            }
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: AppEvent) {
        let AppEvent::SetInstrument { channel, instrument } = event;
        self.dialog_open = false;
        if instrument != u8::MAX {
            self.piano.set_instrument(instrument, channel);
            self.update_title();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.piano.all_notes_off();
                event_loop.exit();
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
                        self.update_title();
                    }
                    Key::Named(NamedKey::ArrowDown) if pressed => {
                        if self.shift_pressed { self.piano.semitone_down(1, ch); }
                        else                  { self.piano.octave_down(ch); }
                        self.update_title();
                    }
                    Key::Named(NamedKey::ArrowRight) if pressed => {
                        self.piano.next_instrument(ch);
                        self.update_title();
                    }
                    Key::Named(NamedKey::ArrowLeft) if pressed => {
                        self.piano.previous_instrument(ch);
                        self.update_title();
                    }
                    Key::Named(NamedKey::PageUp)    if pressed => { self.next_channel(); }
                    Key::Named(NamedKey::PageDown)  if pressed => { self.previous_channel(); }
                    Key::Named(NamedKey::Delete)    if pressed => { self.delete_current_channel(); }
                    Key::Named(NamedKey::Backspace) if pressed => { self.toggle_multi_voice(); }
                    Key::Named(NamedKey::F2)        if pressed => { self.open_instrument_dialog(); }
                    Key::Named(NamedKey::F8)        if pressed => {
                        self.piano.volume_down(ch);
                        self.update_title();
                    }
                    Key::Named(NamedKey::F9)        if pressed => {
                        self.piano.volume_up(ch);
                        self.update_title();
                    }
                    Key::Named(NamedKey::Tab)       if pressed => {
                        self.piano.pan(!self.shift_pressed, ch);
                        self.update_title();
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

    let midi_driver: Box<dyn midi::MidiDriver> = match config.get_midi_driver().as_str() {
        "rustysynth" => {
            let sf_path = config.get_soundfont_path().unwrap_or_else(|| {
                eprintln!("ERRO: [soundfont] file_path e obrigatorio com driver=rustysynth");
                std::process::exit(1);
            });
            Box::new(RustySynthDriver::new(&sf_path)?)
        }
        _ => Box::new(MidirDriver::new()?),
    };

    let mut piano = Piano::new(&keymap_path, midi_driver)?;
    piano.set_instrument(0, 0);

    println!("Piano iniciado.");

    let event_loop = EventLoop::<AppEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let proxy = event_loop.create_proxy();
    let mut app = App::new(piano, proxy);
    event_loop.run_app(&mut app)?;

    Ok(())
}
