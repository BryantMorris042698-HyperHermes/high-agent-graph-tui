# Graph_x_0x0 — High-Agent Graph Analysis TUI

**From Φ(G) to Pixels: A Mathematical Codebase Intelligence Engine for Autonomous Agents**

Graph_x_0x0 is the core analytical engine and terminal user interface (TUI) for the high-agent ecosystem. It models any codebase as a directed graph `G = (V, E)` where nodes are functions and edges represent calls, imports, and shared state. It computes a multi-objective score **Φ(G)** that drives regime-aware optimization, deviation detection, and explainable "Theory Mode" insights.

This project was built as part of the high-agent initiative, integrating deeply with Hermes AI agent (Nous Research) running locally via Termux on Android (Galaxy Z Fold7), self-improving loops, and cross-device workflows (phone guiding laptop installs, Lenovo Tab Pro tablet as coding hub, GitHub Codespaces for HyprHermes).

## The Mathematical Foundation

Everything starts with one equation:

```
Φ(G) = α·Q(G) − β·Č(G) − γ·mean(V)
```

Where:
- **G = (V, E)** — Codebase modeled as directed graph. Nodes = functions/methods. Edges = calls/imports/dependencies with weights (call frequency × params × shared state).
- **Q(G)** — Newman-Girvan modularity: measures how well functions cluster within logical modules. Higher = better cohesion and separation of concerns.
- **Č(G)** — Mean inter-module coupling per node. Lower = less spaghetti code, better encapsulation.
- **V** — Cyclomatic complexity (McCabe). Counts branches, loops, conditionals per function (`E − N + 2P`).
- **α, β, γ** — Regime-dependent coefficients that dynamically shift optimization priorities (e.g., favor cohesion in review mode, minimize coupling in rapid iteration).

This is not cosmetic. Every decision the TUI surfaces — regime switches, deviation alerts, Theory Mode explanations — traces directly back to live Φ(G) computations on the graph.

## Project Structure

```
high-agent-graph-tui/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── bin/
│   │   └── tui.rs
│   ├── graph.rs          # 560 lines: G=(V,E), metrics, modularity, coupling, deviation detection
│   ├── core.rs           # 536 lines: RegimeEngine + Theory Mode + task processing
│   ├── orchestrator.rs   # 264 lines: piecewise regime switching + hysteresis
│   └── tui.rs            # 479 lines: ratatui dashboard (Catppuccin Mocha, 4 tabs)
├── python/
│   └── high_agent_engine/
│       ├── graph.py
│       ├── engine.py
│       └── regime.py
├── examples/
│   └── basic.rs
├── benches/
│   └── graph_bench.rs
└── scripts/
    ├── self-improving-loop.py
    └── high-agent-repl.py
```

## Layer 1: The Graph Core (`graph.rs`)

Pure computation engine. No UI, no I/O.

**Key Primitives:**

```rust
pub struct Node {
    pub id: String,         // function name
    pub module: String,     // which module/package
    pub cyclomatic: f64,    // E − N + 2P
    pub quality: f64,       // 0–1 normalized score
}

pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: f64,        // call frequency × params × shared state
}

pub struct DirectedGraph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}
```

**Core Methods:**
- `modularity()` — Newman-Girvan Q: `Σ[(L_c/L) − (d_c/2L)²]` per module
- `mean_coupling()` — Average cross-module edge weight per node
- `total_cyclomatic()` / `mean_quality()` — Aggregations
- `snapshot()` — Produces `GraphSnapshot` for deviation detector
- `multi_objective(&snapshot, &coeffs)` — Computes Φ(G)

**Deviation Detection (Segmented Regression):**

`SegmentedRegimeDetector` maintains a sliding window of recent snapshots. On each `feed()`:
1. Compute mean + σ for Q, Č, V, quality over window
2. Compute z-scores for new snapshot vs baseline
3. If any `|z| > 2.0`: change-point detected
4. Classify: Improving / Degrading / Mixed

This triggers regime re-evaluation exactly like statistical process control applied to software architecture.

## Layer 2: The Regime Engine (`core.rs` + `orchestrator.rs`)

Three regimes with tuned coefficient profiles:

| Regime   | α   | β   | γ   | Strategy                              | Use Case                  |
|----------|-----|-----|-----|---------------------------------------|---------------------------|
| Simple   | 0.8 | 0.9 | 0.2 | Minimize coupling. Fast iteration.    | Rapid prototyping         |
| Advanced | 1.2 | 0.5 | 0.3 | Maximize modularity. PR review mode.  | Code review / refactoring |
| Hybrid   | 1.0 | 0.6 | 0.4 | Balance all terms. Team handoff mode. | Collaboration / handoff   |

**Orchestrator Responsibilities:**
- `best_regime(snapshot)` — Evaluates all 3 regimes, picks maximizer of Φ(G)
- `evaluate_and_switch(graph)` — Switches if improvement exceeds hysteresis (0.05)
- `detect_and_switch(graph)` — Chains detector → deviation → re-evaluate
- Logs every transition as `RegimeTransition { from, to, reason, phi_before, phi_after }`

**RegimeEngine Extras:**
- `seed_graph()` — Initial 7-node / 8-edge model (core/cli/skills)
- `theory_explain()` — Generates full formatted Theory Mode output (Tab 3)
- `process_task(task)` — Perturbs graph based on task type (refactor/feature/test)
- `simulate_deviation_and_switch()` — Demo mode cycling mutations
- `load_from_json(json)` — Loads real codebase graph from Python crawler

## Layer 3: Python Mirror Engine

Full parity implementation for Termux / rapid prototyping environments where Rust compilation is heavy:

```
high_agent_engine/
├── graph.py    — Identical G=(V,E) + Φ(G) computation
├── engine.py   — RegimeEngine with detect_and_evaluate()
└── regime.py   — Regime enum, hysteresis, SegmentedRegimeDetector
```

**Supporting Scripts (Proven End-to-End):**
- `self-improving-loop.py` — Baseline → verify → commit → rollback cycle. Measures Φ(G), auto-commits improvements, rolls back degradations. Tested on `test_loop`.
- `high-agent-repl.py` — Live orchestrator REPL with `sweep`, `theory`, `history`, `watch` commands. Supports `--daemon` for persistent background evaluation (writes JSON state for TUI consumption).

## Layer 4: The TUI Dashboard (`tui.rs` — ratatui 0.29 + crossterm 0.28)

**Theme:** Catppuccin Mocha (24 named `Color::Rgb` constants — single source of truth for theming).

**Tabs:**
1. **Dashboard** — Φ(G) headline (color-coded green/yellow/red) + 3 Gauge widgets (Q green↑, Č red↓, V yellow) + detail panel with exact metrics + key hints.
2. **Graph** — Placeholder for future GPU-accelerated visualization (node/edge counts, module clusters).
3. **Theory** — Live formatted mathematical explanation with current values plugged in.
4. **History** — Reversed timeline of snapshots with trend arrows (↑ improving, ↓ degrading).

**Layout:** Vertical split (tab bar → content → status bar).

**Event Loop:** Single-threaded, blocking but responsive. `r` key manually refreshes from daemon JSON. `1-4` switch tabs, `?` help overlay, `q` quit.

**Status Bar:** `Graph_x_0x0  <codebase>  Φ=+0.1234` (regime-colored badge) or help key reference.

**Key Separation of Concerns:** The TUI is a pure projection. It never computes Φ(G). The engine runs headless (daemon/cron/CI). Daemon writes JSON; TUI reads it on demand. Perfect for agent-orchestrated workflows.

## Layer 5: Build System (`Cargo.toml`)

```toml
[dependencies]
ratatui = { version = "0.29", optional = true }
crossterm = { version = "0.28", optional = true }

[features]
default = []
tui = ["dep:ratatui", "dep:crossterm"]

[[bin]]
name = "high-agent-tui"
path = "src/bin/tui.rs"
required-features = ["tui"]
```

- Termux / minimal: `cargo build` (core + CLI only)
- Desktop (OMEN/Arch): `cargo build --features tui`
- TUI binary gated so it fails clearly without the feature flag.

## How to Build Your Own Agent TUI (Blueprint)

1. **Data Model First** — Plain struct with `Default` as single source of truth (`Metrics`, `App`).
2. **Layout Pattern** — `Layout::default().direction(Vertical)` with constraints. Start simple.
3. **Palette Module** — Named color constants. Change theme in one place.
4. **Pure `draw(frame, app)`** — No side effects. Dispatch per tab.
5. **Event Loop** — `terminal.draw` → `event::poll` → `handle_key` (pure match). Offload heavy work to threads.
6. **Feature Gate Heavy Deps** — Keep core portable.
7. **Separate Binary** — `src/bin/tui.rs` keeps library clean.

## Connections to Past High-Agent & Hermes Work

This TUI was designed as the **visual governance layer** for the broader high-agent ecosystem developed across our sessions:

- **Hermes Integration (Termux on Galaxy Z Fold7 + Lenovo Tab Pro)**: The Python mirror + `--daemon` mode allows Hermes (running locally on Android) to drive `self-improving-loop.py` or call `high-agent-repl.py` commands. F9 keybind in Hyprland launches Hermes; it can now trigger regime evaluation or read Φ(G) trends.
- **Self-Improving Loops**: The `self-improving-loop.py` (baseline → verify Φ(G) → commit/rollback) is the closed-loop agent behavior. Graph_x_0x0 provides the objective function.
- **Theory Mode & Explainability**: Directly supports truthful recall and high-fidelity explanations (targeting 96%+ in MemePlace/Honcho/Holographic memory experiments).
- **Multi-Device Architecture**: Phone (Hermes) guides laptop (Arch install + high-agent-rs), tablet acts as central hub. TUI runs on laptop; daemon feeds phone REPL or Codespaces.
- **MemePlace SLM Accelerator / Honcho / Holographic**: Future extension — use graph metrics as structured knowledge base entries ("deep well" spatial metaphor). Φ(G) deviations become retrievable "rooms" for prompt augmentation in SLMs.
- **High-Agent Orchestration**: `RegimeEngine::process_task()` and `orchestrator` already support task-driven graph perturbation — ready for agent swarms delegating refactor/feature/test work with measurable architecture impact.

## Future Roadmap (Agent-Driven)

- Real codebase ingestion via Python crawler (tree-sitter / rust-analyzer) → JSON → `load_from_json`
- GPU-accelerated Graph tab (egui or wgpu viz of clusters + force-directed layout)
- Hermes "superpowers" plugin: native function calling into RegimeEngine
- BTRFS snapshot integration for safe rollback of low-Φ changes
- Multi-repo federation (analyze entire workspace or monorepo)
- Web dashboard mirror (for tablet / remote viewing)
- Automated PR suggestions when Advanced regime detects high coupling

## Getting Started

```bash
# Core only (Termux friendly)
cargo build

# Full TUI (desktop)
cargo build --features tui

# Run TUI
cargo run --features tui --bin high-agent-tui

# Python side (after installing deps)
python3 python/high_agent_engine/engine.py --daemon
python3 scripts/self-improving-loop.py
python3 scripts/high-agent-repl.py --help
```

## License & Attribution

Built collaboratively with Bryant Issiah Morris Jr. as part of the HyperHermes / high-agent initiative. All mathematical foundations, regime logic, and TUI architecture originated in iterative sessions focused on local-first, explainable, self-governing AI tooling.

**Repository:** https://github.com/BryantMorris042698-HyperHermes/high-agent-graph-tui

---

*“The TUI is a projection of the mathematical model. The engine has no idea what a terminal is.”*

This separation is what makes Graph_x_0x0 a true **Agent TUI** — headless intelligence that agents (Hermes, future swarms) can drive, observe, and improve autonomously.