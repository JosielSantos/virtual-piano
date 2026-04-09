# piano-rs

Versão Rust do Virtual Piano — MVP com funcionalidades básicas.

## Funcionalidades

- Tocar notas MIDI ao pressionar teclas (keymap `.kmp` compatível com a versão Python)
- **↑ / ↓** — subir/descer uma oitava (+12 / -12 semitons)
- **Shift + ↑ / ↓** — subir/descer um semitom
- Saída MIDI via `midir` (usa o sintetizador MIDI do sistema)

## Pré-requisitos

- [Rust](https://rustup.rs/) (edition 2021)
- Um sintetizador MIDI ativo no sistema:
  - **Windows**: Microsoft GS Wavetable Synth (já disponível) ou [VirtualMIDISynth](https://coolsoft.altervista.org/virtualmidisynth)

## Como executar

Execute a partir do diretório `piano-rs/` (ou `virtual-piano/`):

```bash
# A partir de piano-rs/
cargo run --release

# O app busca config.ini em ./ e depois em ../
```

O título da janela exibe a nota inicial atual (ex.: `C3`).

## Estrutura

```
src/
├── main.rs           # Entry point — event loop winit
├── piano.rs          # Fachada Piano
├── channel.rs        # Estado do canal (notas ativas, transposição)
├── notes_manager.rs  # Carrega keymap .kmp + lógica de transposição
├── config.rs         # Leitura de config.ini
├── models/
│   └── note.rs       # Struct Note (MIDI 0–127)
└── midi/
    ├── mod.rs         # Trait MidiOutput
    └── driver_midir.rs# Implementação com midir
```

## Configuração

O arquivo `config.ini` (na raiz do repositório) é lido automaticamente:

```ini
[general]
keymap_file_path=pianoeletronico.kmp

[midi]
output_driver=-1
```
