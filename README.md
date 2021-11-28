# ⚡ Introduction
Auritia is a DAW coded in Rust and Vue in hopes of having cross platform compatability, while also providing enough features for anyone to use professionally 

![Auritia](https://cdn.discordapp.com/attachments/911762334979084368/914499510741381130/unknown.png)

# ✨ Installation & Contributing

1.  Install [WebView](https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/b97b52c3-9a66-419c-9ef0-90e3a3f72c5c/MicrosoftEdgeWebview2Setup.exe) if you're not on Windows 11
2.  Install Node deps `npm i`
3.  To run the dev server do `npm run tauri dev`

# 🛠 Compiling

## Linux

1. Install bloatware dependancies
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
2. Compile with `npm run compile`
3. The compiled bundle will be in `src-tauri/target/release/bundle/<auritia>.deb`
