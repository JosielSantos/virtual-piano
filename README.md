# virtual-piano

Um piano virtual MIDI.

## Usando

Primeiro, copie o arquivo 'config.ini.dist' para 'config.ini'.

A configuração inicial utiliza o driver padrão de MIDI do seu sistema.

Instale o [uv](https://docs.astral.sh/uv/getting-started/installation/).
Na pasta do projeto, instale as dependências com `uv sync --locked`.
O uv gerencia o Python 3.13 e o ambiente `.venv` automaticamente.

Para executar, rode `uv run --locked python src/main.py` ou, no Windows,
abra `exec.bat`. Não é necessário ativar o ambiente virtual.

Se você usa [just](https://github.com/casey/just), os comandos equivalentes
são `just install` e `just run`. Para gerar o executável no Windows,
use `just build-windows`.

As dependências ficam em `pyproject.toml`, e as versões resolvidas em
`uv.lock`. Versione ambos; ao alterar as dependências, execute `uv lock`
e depois `uv sync --locked`.

O driver FluidSynth também exige a biblioteca nativa do FluidSynth e um
SoundFont configurado. O uv instala o pacote Python, mas não esses arquivos.

### Teclas de atalho

* Setas direita / esquerda: navegar entre os instrumentos
* F2: Selecionar instrumento por número (1 - 128)
* Setas cima / baixo: oitava acima / abaixo no canal atual
* shift + seta cima / baixo: Meio tom acima / abaixo no canal atual
* F8 / F9: abaixa / aumenta o volume do canal atual
* Tab / Shift+Tab: muda a direção no canal atual (direita, esquerda, meio)
* Pageup / Pagedown: navegar entre os canais
* Backspace: Tocar todos os canais selecionados (multi voice)
* Delete: Reseta as configurações do canal atual e volta para o anterior (não funciona no canal 1)

## Configuração

### Mapa de teclas (general.keymap_file_path)

Aqui você pode preencher com um caminho absoluto (/home/a/b/c.kmp) ou com o nome de um arquivo que esteja dentro da pasta 'keymaps'.

### Dispositivo MIDI (midi.output_driver)

* maior ou igual a 0 - Um driver do sistema
* -1 - Dispositivo padrão do sistema
* -2 - Fluidsynth (necessário configurar a seção 'soundfont')
