# Volumize

<p align="center">
    <img height="150" src="./public/icon.png"/>
</p>

<p align="center">
  <strong>Control per-application volume levels from your phone</strong>
</p>
<p align="center">
    <i>
        A cross-platform desktop and mobile app for controlling individual application volumes remotely.
    </i>
</p>

## Overview

This project allows you to control individual application volumes on desktop from your mobile device. The project consists of two components:

- **Server (Desktop)**: Desktop application that manages system volume control with system tray functionality.
- **Client (Mobile)**: Mobile app that connects to the desktop server via WebSocket to adjust volume levels remotely.

**🚧 Currently, the server only supports the Windows platform.** 🚧

### Architecture overview

```rs
┌──────────────────┐                  ┌──────────────────┐
│   Desktop App    │                  │   Mobile App     │
│   (Server)       │    WebSocket     │   (Client)       │
│                  │ ←──────────────→ │                  │
│ • Volume Control │                  │ • Remote Control │
│ • System Tray    │                  │ • Settings       │
│                  │                  │ • Status         │
└──────────────────┘                  └──────────────────┘
```

## Why this project exists

Ever wanted to adjust your desktop application volumes without:

- Leaving your couch?
- Having to switch to your desktop?

But for me, the main reason is this:

One of the worst parts of using Windows is the volume mixer UI. Isn't it **really** annoying to adjust individual application volumes by having to click twice + scroll + animations instead of just 1-2 clicks?

## Tech stack

<ul>
    <!-- ===================== Frontend ====================== -->
    <li align="left">
        <strong align="left">Frontend: </strong>
        <sub>
            <img width="20px" src="https://skillicons.dev/icons?i=react" />
        </sub>
        React +
        <sub>
            <img width="20px" src="https://skillicons.dev/icons?i=typescript" />
        </sub>
        TypeScript +
        <sub>
            <img width="20px" src="https://skillicons.dev/icons?i=vite" />
        </sub>
        Vite +
        <sub>
            <img width="20px" src="https://skillicons.dev/icons?i=less" />
        </sub>
        Less
    </li>
    <!-- ====================== Backend ====================== -->
    <li align="left">
        <strong align="left">Backend: </strong>
        <sub>
            <img width="20px" src="https://skillicons.dev/icons?i=rust" />
        </sub>
        Rust +
        <sub>
            <img width="20px" src="https://skillicons.dev/icons?i=tauri" />
        </sub>
        Tauri
    </li>
    <!-- ==================== Communication ==================== -->
    <li align="left">
        <strong align="left">Communication: </strong>
        <sub>
            <svg width="20px" xmlns="http://www.w3.org/2000/svg"
            aria-label="socket_io" role="img"
            viewBox="0 0 512 512"><path
            d="m0 0H512V512H0"
            fill="#fff"/><circle cx="256" cy="256" r="228" fill="none" stroke="#010101" stroke-width="40"/><path fill="#010101" d="M196 244q80-70 164-134l-90 134zm46 24h74q-80 70-164 134z"/></svg>
            <!-- Currently WebSocket icon is not available -->
            <!--<img width="20px" src="https://skillicons.dev/icons?i=websocket" />-->
        </sub>
        WebSocket +
        <!-- No UDP or mDNS/DNS icons available -->
         <sub>
             <svg width="20px" viewBox="0 0 32 32" xmlns="http://www.w3.org/2000/svg">
               <rect x="20" y="20" width="10" height="2"/>
               <rect x="20" y="24" width="6" height="2"/>
               <path d="M30,17V16A13.9871,13.9871,0,1,0,19.23,29.625l-.46-1.9463A12.0419,12.0419,0,0,1,16,28c-.19,0-.375-.0186-.563-.0273A20.3044,20.3044,0,0,1,12.0259,17Zm-2.0415-2H21.9751A24.2838,24.2838,0,0,0,19.2014,4.4414,12.0228,12.0228,0,0,1,27.9585,15ZM16.563,4.0273A20.3044,20.3044,0,0,1,19.9741,15H12.0259A20.3044,20.3044,0,0,1,15.437,4.0273C15.625,4.0186,15.81,4,16,4S16.375,4.0186,16.563,4.0273Zm-3.7644.4141A24.2838,24.2838,0,0,0,10.0249,15H4.0415A12.0228,12.0228,0,0,1,12.7986,4.4414Zm0,23.1172A12.0228,12.0228,0,0,1,4.0415,17h5.9834A24.2838,24.2838,0,0,0,12.7986,27.5586Z"/>
             </svg>
        </sub>
        mDNS + UDP
    </li>
    <!-- ======================================================= -->
</ul>

<!--- **Frontend**: React + TypeScript + Vite + Less -->
<!--- **Backend**: Rust (Tauri) -->
<!--- **Communication**: WebSocket -->
<!--- **Platforms**: Windows Desktop + Mobile (iOS/Android) -->

## 🚀 Quick start

### Prerequisites

- **Node.js** (v22 LTS or higher)
- **Rust** (v1.90 or latest stable)

```bash
npm install
npm run tauri dev
```

For detailed installation instructions, see [**INSTALL.md**](INSTALL.md).

## Project status

Currently in active development. See [tracker.md](tracker.md) for detailed progress and roadmap.
