# docs — Project Architecture & Control Center

Canonical source of truth for architecture, specifications, rules, and tracking.

## Read order

1. `README.md` (Project README — read first)
2. `AGENTS.md`
3. `docs/README.md` (this file)
4. `context/project.md` → `stack.md` → `architecture.md`
5. `product/PRD.md`
6. `config/protected.yaml` + `config/permissions.yaml`
7. `rules/core.md`
8. The matching task in `tasks/queued/`

## Map

```
docs/
├── config/          # project.yaml, commands.yaml, paths.yaml, permissions.yaml, protected.yaml
├── context/         # project, product, stack, architecture, structure, conventions, dependencies, health, glossary
├── product/         # PRD, principles, anti-feature-creep, requirements, user-flows, design-system
├── planning/        # roadmap, milestones, current
├── specs/           # features (FEAT-001 to 005), components
├── tasks/           # queued (TASK-001 to 010), active, blocked, completed
├── decisions/       # ADR-001 to 004 + index
├── rules/           # core, coding, frontend, performance, security, testing, git
├── skills/          # planning, implementation, debugging, testing, refactoring, etc.
└── checklists/      # feature, bug, release, security, production
```

## PASS definition

```
PASS = tests passed + expected behavior verified + scope respected
     + no suspicious changes + documentation updated
```
