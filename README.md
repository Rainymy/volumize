# Volumize

A cross-platform desktop and mobile app for controlling per-application volume levels remotely.

## Overview

This Tauri application allows you to control individual application volumes on Windows from your mobile device. The project consists of two components:

- **Server (Desktop)**: Windows app that manages system volume control with system tray functionality
- **Client (Mobile)**: Mobile app that connects to the desktop server via WebSocket to adjust volume levels remotely

## Features

- ✅ Per-application volume control on Windows
- ✅ Desktop UI for volume management
- 🚧 WebSocket communication between desktop and mobile
- 🚧 Mobile app interface
- 🚧 System tray integration
- 🚧 Auto-discovery of desktop server

## Tech Stack

- **Frontend**: React + TypeScript + Vite + Less
- **Backend**: Rust (Tauri)
- **Communication**: WebSocket
- **Platforms**: Windows Desktop + Mobile (iOS/Android)

## Development Setup

### Prerequisites
- Node.js
- Rust

### Getting Started

1. Clone the repository
2. Install dependencies:
   ```bash
   npm install
   ```
3. Run desktop development server:
   ```bash
   npm run tauri dev
   ```
4. For mobile development:
   ```bash
   npm run tauri android dev  # or ios dev
   ```

## Project Status

Currently in active development. See [tracker.md](tracker.md) for detailed progress and todo items.

## Architecture

```
┌─────────────────┐                  ┌─────────────────┐
│   Desktop App   │                  │   Mobile App    │
│   (Server)      │    WebSocket     │   (Client)      │
│                 │ ←──────────────→ │                 │
│ • Volume Control│                  │ • Remote Control│
│ • System Tray   │                  │ • Settings      │
│ • Settings      │                  │ • Status        │
└─────────────────┘                  └─────────────────┘
```