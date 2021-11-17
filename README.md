<p>
    <img alt="Vue" height="32px" src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/vue/vue.png" />
    <img alt="Typescript" height="32px" src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/typescript/typescript.png" />
    <img alt="Vite" height="32px" src="https://seeklogo.com/images/V/vite-logo-BFD4283991-seeklogo.com.png" />
    <img alt="Rust" height="32px" src="https://raw.githubusercontent.com/github/explore/80688e429a7d4ef2fca1e82350fe8e3517d3494d/topics/rust/rust.png" />
    <img alt="Tauri" height="32px" src="https://raw.githubusercontent.com/tauri-apps/tauri/HEAD/app-icon.png" />
</p>

## Steps

1.  Install [WebView](https://msedge.sf.dl.delivery.mp.microsoft.com/filestreamingservice/files/b97b52c3-9a66-419c-9ef0-90e3a3f72c5c/MicrosoftEdgeWebview2Setup.exe) if you're not on Windows 11
2.  Install Node deps `npm i`
3.  To run the dev server do `npm run tauri dev`

# Compiling

## Linux

1. You will need to install Tauri's dependancies
```
sudo apt update && sudo apt install -y libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libappindicator3-dev \
    patchelf \
    librsvg2-dev
```
2. Install ALSA audio drivers
```
sudo apt install -y alsa-utils libasound2-dev
```
3. Install NPM Dependancies
```
npm i
```
4. Compile with
```
npm run compile
```
