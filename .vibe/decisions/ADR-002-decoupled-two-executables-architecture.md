# ADR-002 — Decoupled Two-Executables Architecture

## Status: Accepted
**Date:** 2026-10-07  
**Author:** Himal  

## Context
Exposing complex preferences and menus directly within a sticky note destroys its physical, minimalist appearance. Conversely, running a persistent settings process wastes system memory when preferences are rarely changed.

## Decision
Split the software into two distinct executables built from a single Rust codebase:
1. `Sticky Note.exe`: Manages active notes on the desktop layer, runs continuously with minimal footprint.
2. `Sticky Note Settings.exe`: Standalone settings utility that runs strictly on-demand, modifies `config.json`, and exits immediately.

## Alternatives Considered
1. **Embedding settings into the note window:** Ruined note aesthetic with gears/toolbars.
2. **Single process with hidden settings window:** Wastes memory keeping the settings window tree resident in RAM.

## Consequences
- The note UI remains 100% clean and distraction-free.
- The settings binary has zero idle cost when closed.
- Both binaries share `config.json` via the shared `sticky-note-core` crate.
