# Skill: dependency-cleanup

Audit `Cargo.toml` and `Cargo.lock`:
1. Check for unused crates in workspace dependencies.
2. Run `cargo audit` to detect known vulnerabilities.
3. Check binary footprint impact of any newly added dependency.
4. Verify zero network-related dependencies are included.
