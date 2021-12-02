

# ⚡ Introduction
<img align="left" src="https://media.discordapp.net/attachments/915542222680764426/916035335702585374/unknown.png?width=256&height=256">

Auritia is a DAW inspired by [Ableton](https://ableton.com/) coded in [Rust](https://www.rust-lang.org/) and [Vue](https://vuejs.org/) in hopes of having cross platform compatability, while also providing enough features for anyone to use professionally
<br>
<br>
The project right now is at a very early stage and theres no ability to drop files in the timeline due to a limitation of [winit](https://github.com/rust-windowing/winit) that [tauri](https://tauri.studio/) uses

So right now we are focusing on creating the UI and everything else around the DAW until the time comes that we can proceed with the audio engine

![Auritia](https://cdn.discordapp.com/attachments/911762334979084368/914499510741381130/unknown.png)

# ✨ Installation & Contributing

- Install [WebView](https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/b97b52c3-9a66-419c-9ef0-90e3a3f72c5c/MicrosoftEdgeWebview2Setup.exe) if you're not on Windows 11
- Install Node deps `npm i`
- To run the dev server do `npm run tauri dev`

# ⌨ Shortcuts

## General

- `CTRL + ,` -> Preferences
- `F11` -> Fullscreen
- `F1` -> Open Samples Picker
- `F2` -> Open Plugins Picker
- `Alt + Enter` -> Toggle Maximize
- `Ctrl + M` -> Toggle Metronome
- `Ctrl + L` -> Toggle Metronome
- `Ctrl + N` -> New Project
- `Ctrl + O` -> Open Project
- `Ctrl + S` -> Save Project
- `Ctrl + Alt + S` -> Save Project As

## Timeline

- `Shift + Scroll` -> Vertical Zoom In/Out
- `Ctrl + 1` -> Lower subgrid division
- `Ctrl + 2` -> Raise subgrid division

# 🛠 Compiling

## 🐧 Linux

- Install bloatware dependancies

```
  sudo apt update && sudo apt install -y libwebkit2gtk-4.0-dev \
   build-essential \
   curl \
   wget \
   libssl-dev \
   libgtk-3-dev \
   libappindicator3-dev \
   patchelf \
   librsvg2-dev \
   alsa-utils \
   libasound2-dev
```

- Install bloat `npm i`
- Compile with `npm run compile`
- The compiled bundle will be in `src-tauri/target/release/bundle/deb/<auritia>.deb`

## 💻 Windows

- Install [Visual Studio C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [Rustc and Cargo Package Manager](https://win.rustup.rs/x86_64)
- Install bloat `npm i`
- Compile with `npm run compile`
- The compiled bundle will be in `src-tauri/target/release/bundle/msi/<auritia>.msi`
