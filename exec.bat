@echo off
cd /d "%~dp0"
uv run --locked python src/main.py
