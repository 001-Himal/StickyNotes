# performance.md

- **Idle Memory Budget:** <50 MB total RAM across all note windows (target: 15–30 MB).
- **Idle CPU:** Exactly ~0.0% CPU when not being actively edited, dragged, or animated.
- **Debounced I/O:** Debounce keystroke disk writes by 300ms to eliminate storage thrashing.
- **Zero Busy-Waiting:** Use native OS window and event pump notifications; zero busy polling loops.
- **Startup Speed:** Cold start to visible window rendered in <1.0 second.
- **Release Optimization:** Ensure release builds configure Link Time Optimization (`lto = true`), codegen units 1, and binary stripping (`strip = true`).
