# ADR-001 — Selection of Rust and Slint for Desktop UI and Engine

## Status: Accepted
**Date:** 2026-10-07  
**Author:** Himal  

## Context
A sticky note is not an application; it is digital paper on a desktop. It must consume negligible memory (<50 MB RAM), incur ~0% idle CPU, start under 1 second, and provide cross-platform native window rendering. Electron, Chromium wrappers, and standard webview frameworks consume hundreds of megabytes of memory and introduce large garbage collection runtimes.

## Decision
Build the application with **Rust** as the language and **Slint** as the native desktop GUI framework.

## Alternatives Considered
1. **Electron:** Discarded due to extreme RAM usage (>150MB+ idle) and slow startup.
2. **Tauri:** Discarded because while better than Electron, it still runs the OS webview (Edge/WebKit), consuming 40-80MB RAM and web DOM overhead.
3. **C++ / Qt:** High memory and complex cross-platform distribution licensing; Rust provides memory safety and modern package management.

## Consequences
- Ultra-low memory usage (15-30 MB idle).
- Zero-cost native compilation without web engine bloat.
- Clean Slint declarative UI files compiled directly into native Rust machine code.
