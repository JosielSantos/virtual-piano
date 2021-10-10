# virtual-piano

Um piano virtual MIDI.

## Usando

Primeiro, copie o arquivo 'config.ini.dist' para 'config.ini'.

A configuração inicial utiliza o driver padrão de MIDI do seu sistema.

Agora instale as dependências: `pip install -r requirements.txt`

Para executar, Rode o comando `python src/main.py`

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

### Dispositivo MIDI (midi.output_driver)

* maior ou igual a 0 - Um driver do sistema
* -1 - Dispositivo padrão do sistema
* -2 - Fluidsynth (necessário configurar a seção 'soundfont')
