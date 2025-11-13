# Installation and Setup Guide

Complete installation and setup instructions for Volumize.

## Table of Contents

- [System Requirements](#system-requirements)
- [Prerequisites](#prerequisites)
    - [Desktop Development](#required-for-desktop-development)
    - [Mobile Development](#additional-for-mobile-development)
- [Desktop Development Setup](#desktop-development-setup)
- [Mobile Development Setup](#mobile-development-setup)
- [Building for Production](#building-for-production)
    - [Desktop](#desktop-windows)
    - [Android](#android-1)
    - [IOS](#ios)
- [Troubleshooting](#troubleshooting)
- [Need Help?](#need-help)

## System Requirements

### Desktop (Server)

- Windows platform only.
- 🚧 Linux and macOS are not supported yet (hopefully soon). 🚧

### Mobile (Client)

- **iOS**: version 13.0 or higher
- **Android**: API level 28 (Android 9.0) or higher

## Prerequisites

Before you begin, ensure you have the following installed:

### Required for Desktop Development

1. **Node.js** (v22 or higher)

    ```bash
    node --version
    ```

    Download from https://nodejs.org/

2. **Rust** (v1.90 or latest stable)

    ```bash
    rustc --version
    ```

    Download from https://rust-lang.org/tools/install/

### Additional for Mobile Development

#### **For Android Requirements**:

- Android Studio
- Android SDK (API level 28 or higher)
- Java Development Kit (JDK 20+)
- Set `ANDROID_HOME` and `NDK_HOME` environment variable

1. Install [Java Development Kit](https://adoptium.net/) latest LTS version
2. Install [Android Studio](https://developer.android.com/studio) latest version
3. Inside Android Studio, open the SDK Manager and install the following components:
    - **Android SDK Platform**
    - **Android SDK Platform-Tools**
    - **NDK (Side by side)**
    - **Android SDK Build-Tools**
    - **Android SDK Command-line Tools**

4. Set the `ANDROID_HOME` and `NDK_HOME` environment variables

    **For Linux**:

    ```bash
    export ANDROID_HOME="$HOME/Android/Sdk"
    export NDK_HOME="$ANDROID_HOME/ndk/$(ls -1 $ANDROID_HOME/ndk)"
    ```

    **For Windows**:

    ```powershell
    [System.Environment]::SetEnvironmentVariable("ANDROID_HOME", "$env:LocalAppData\Android\Sdk", "User")
    $VERSION = Get-ChildItem -Name "$env:LocalAppData\Android\Sdk\ndk" | Select-Object -Last 1
    [System.Environment]::SetEnvironmentVariable("NDK_HOME", "$env:LocalAppData\Android\Sdk\ndk\$VERSION", "User")
    ```

#### **For iOS Requirements**:

🚧 UNTESTED 🚧

- Xcode 13 or higher
- Xcode Command Line Tools
- CocoaPods

1. Install [Homebrew](https://brew.sh/)
2. Install [CocoaPods](https://cocoapods.org/) using Homebrew

    ```bash
    brew install cocoapods
    ```

## Desktop Development Setup

1. **Clone the repository**

    ```bash
    git clone https://github.com/Rainymy/volumize.git
    cd volumize
    ```

2. **Install dependencies**

    ```bash
    npm install
    ```

3. **Run the development server**

    ```bash
    npm run tauri dev
    ```

    The desktop app should launch automatically.

## Mobile Development Setup

First follow the instructions for the [Desktop Development Setup](#desktop-development-setup).

### Android

1. Ensure `ANDROID_HOME` and `NDK_HOME` are set correctly:

    **For Linux**:

    ```bash
    echo $ANDROID_HOME
    echo $NDK_HOME
    ```

    **For Windows**:

    ```bash
    echo $env:ANDROID_HOME
    echo $env:NDK_HOME
    ```

2. **Add the Android targets with `rustup`**

    ```bash
    rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
    ```

3. **Setup Android environment**

    ```bash
    npm run tauri android init
    ```

4. **Run on Android emulator or device**

    ```bash
    npm run tauri android dev
    ```

    If you want to run on a physical device, you need to configure it in Android Studio.

### iOS (macOS only, Not Tested)

1. **Add the iOS targets with `rustup`**

    ```bash
    rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
    ```

2. **Setup iOS environment**

    ```bash
    npm run tauri ios init
    ```

3. **Run on iOS device/simulator**
    ```bash
    npm run tauri ios dev
    ```

## Building for Production

### Desktop (Windows)

```bash
npm run tauri build
```

The built executable will be in `src-tauri/target/release/`.

### Android

```bash
npm run tauri android build
```

The APK will be in `src-tauri/gen/android/app/build/outputs/apk/`.

### iOS

```bash
npm run tauri ios build
```

Open the Xcode project and archive from there.

## Troubleshooting

### Mobile: WebSocket connection fails

1. Ensure desktop and mobile are on the same network
2. Check firewall settings allow the connection
3. Verify the server port is not blocked

## Need Help?

- [Open an issue](https://github.com/Rainymy/volumize/issues)
