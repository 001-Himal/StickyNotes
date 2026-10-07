# ADR-003 — Self-Contained Local JSON Storage

## Status: Accepted
**Date:** 2026-10-07  
**Author:** Himal  

## Context
Scattering files across OS-specific user folders (like Windows Documents or AppData) makes tracking, moving, and backing up notes cumbersome. Using cloud databases or local SQL engines (SQLite) adds binary size and unnecessary complexity for a simple note widget.

## Decision
Keep all data self-contained within the application's root directory under `./Sticky Note Notes/Notes.json`, with atomic temporary file swapping (`.tmp` write followed by rename).

## Alternatives Considered
1. **Windows Documents directory (`%USERPROFILE%\Documents\`):** Clutters user files and varies between operating systems.
2. **SQLite database:** Overkill for lightweight JSON text notes; prevents easy manual user inspection or editing.
3. **Cloud database:** Rejected due to offline-first and anti-bloat principles.

## Consequences
- Fully portable application folder.
- Human-readable and transparent data storage.
- Atomic file writes prevent any data corruption during unexpected system reboots.
