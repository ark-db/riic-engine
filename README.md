# RIIC Engine

RIIC Engine is a standalone desktop app for simulating Arknights base setups. The project is currently in development.

## Developer Quickstart

1. Make sure [Node.js](https://nodejs.org/en/) and [Rust](https://www.rust-lang.org/tools/install/) (1.57+) are installed
2. Run `npm i` to install dependencies
3. Run `npm run tauri dev` to start the app in dev mode or `npm run tauri build` to build the app

To update game assets used in the app:

1. Make sure [Python](https://www.python.org/downloads/) (3.11+) and [Poetry](https://python-poetry.org/docs/#installation) are installed
2. Run `poetry env use 3.11` to set up a virtual environment
3. Run `poetry install` to install dependencies
4. Run `poetry run python scripts/main.py` to fetch the latest assets

## License

Licensed under version 3 of the GNU General Public License or (at your option) any later version. See [LICENSE](LICENSE) for more details.
