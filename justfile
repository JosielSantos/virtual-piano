default:
    @just --list

install:
    @uv sync --locked

run:
    @uv run --locked python src/main.py

build-windows:
    @uv run --locked python -m PyInstaller --onefile --windowed --name virtual-piano src/main.py
