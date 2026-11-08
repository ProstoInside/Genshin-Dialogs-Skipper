# GDS - Genshin Dialogs Skipper

<div align="center">

**English** | [Русский](README_RU.md)

![license](https://img.shields.io/badge/LICENSE-MIT-brightgreen?style=for-the-badge)
![version](https://img.shields.io/badge/VERSION-v0.3.0-blue?style=for-the-badge)

</div>

## About the program

* This program allows you to skip dialogues in the game Genshin Impact. It is written in the Rust programming language.
* It functions similarly to an auto-clicker, but it skips dialogues using a specific, cautious pattern.
* The sequence is: Algorithm -> Keyboard -> Game.

* Every timing is randomized, so the input looks natural while staying fast:
  * 80–140 ms pause between cycles;
  * 20–40 ms pause between keys;
  * each key is held for 25–55 ms, just like a real press.
* GDS doesn't read or modify game memory and doesn't inject anything into the game — it only sends key presses, like a regular keyboard.


> [!CAUTION]
> 
> For those planning to download and use the program:
> The risk of a ban is minimal!
> However, as the creator, I cannot guarantee that you definitely won't get banned.
> If you choose to install and use the program, you do so entirely at your own risk.

## 🛠️ Installation

1. Go to the [Releases](https://github.com/ProstoInside/Genshin-Dialogs-Skipper/releases) category and download the latest version of the gds.exe file.

2. After downloading, you can move the executable file anywhere you want, for example, to drive C
  * This is optional
  * You can run the program directly from the downloads folder.

3. Launch the program by double-clicking it, and grant permission to run it as an administrator.
  * You must run it as an administrator so that the game recognizes the program's clicks as your own.

4. Next, follow the instructions in [INSTRUCTIONS.md](INSTRUCTIONS.md) ([на русском](INSTRUCTIONS_RU.md)) to properly configure the program for an enjoyable gaming experience.


## 🛠️ Developer Instructions

1. Clone the repository:

```bash
git clone https://github.com/ProstoInside/Genshin-Dialogs-Skipper.git
cd Genshin-Dialogs-Skipper
```

   Or download it as a ZIP (**Code → Download ZIP**), extract it to a folder of your choice and open a terminal in that folder.

### Requirements

* Windows 10/11
* [Rust](https://rustup.rs) (stable, MSVC toolchain)
* [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the **"Desktop development with C++"** workload (includes the Windows SDK — `rc.exe` is required to embed the icon and manifest)

### Build

```bash
cargo build --release
```

The finished program will be at `target\release\gds.exe`.

### Other commands

```bash
# Debug build with a console window (does not request admin rights)
cargo run

# Run tests
cargo test
```

> [!NOTE]
> The release build requires administrator rights (UAC prompt on launch).
> Running `cargo run --release` from a non-admin terminal will fail with error 740 — run the terminal as administrator or launch `gds.exe` directly.

