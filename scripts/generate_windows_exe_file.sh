#!/usr/bin/sh

FS_URL="https://github.com/FluidSynth/fluidsynth/releases/download/v2.2.3/fluidsynth-2.2.3-win10-x64.zip"
FS_DIR="fs"
FS_ZIP_FILE="FluidSynth.zip "

# Download FluidSynth
echo "Downloading FluidSynth..."
wget -q -O $FS_ZIP_FILE $FS_URL
echo "Extracting File..."
unzip -qq -o -d $FS_DIR $FS_ZIP_FILE
uv run --locked python -m PyInstaller --onefile --windowed --name virtual-piano src/main.py
#rm -rf $FS_DIR $FS_ZIP_FILE
