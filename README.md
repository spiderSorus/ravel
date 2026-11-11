# Ravel — Deterministic Harmonic ECS & Multiverse Simulation Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

**Ravel** is a deterministic, procedural multiverse simulation engine and experimental sandbox written in **Rust** (`Bevy 0.14` + custom ECS). Instead of static scripted behaviors, Ravel simulates an evolving ecological universe governed by a **Physics of Quality** modeled on the **64 Hexagrams of the I Ching ("Harmonic Chords")** and resolution-gated epochs of complexity.

Built for **simulation developers, emergent-systems researchers, and Rust game-architecture tinkerers**, Ravel includes both a reusable core ECS library (`simulator/ravel_ecs`) and an interactive multi-experiment visual simulator (`simulator/ravel_sim`) featuring the 64-spoke **Rave Mandala Cosmic Radar** and multi-universe comparison views (`Multiverse A / B / C`).

---

## Built With

- **Language**: **Rust** (Cargo workspace in `simulator/`).
- **Engine & Rendering**: [Bevy `0.14`](https://bevyengine.org/) (`bevy`, `bevy_svg`) for real-time 2D/UI visualization, camera management, and interactive experiment switching.
- **Core Simulation Library (`ravel_ecs`)**:
  - Unified **Ravel Object Model (ROM)**: Every entity (`Object / Yin` vs `Action / Yang`) carries a 64-dimensional integer `Chord` (`[i32; 64]`), a bit-depth `Resolution` tier ($R \in \{1, 2, 4, 8, 16, 32, 64\}$), and a spatial `Scope`.
  - **Harmonic Homeostasis Spawner**: Continuously samples the `GlobalChord` across all active entities and spawns complementary entities (Supply & Demand homeostasis) to balance ecological spikes.
- **Serialization & Determinism**: `serde` (`derive`), `rand 0.8`, and `downcast-rs`.

---

## Repository Structure

- `simulator/` — Cargo workspace containing:
  - `ravel_ecs/` — Core Entity Component System, 64-Gate I Ching database (`src/iching.rs`), Chord math, Taijitu polarity, and ecological spawner logic.
  - `ravel_sim/` — Interactive Bevy simulation viewer, Multiverse Manager (`Multiverse A/B/C`), Rave Mandala 64-spoke wheel renderer, and UI controls.
  - `ravel_physics_whitepaper.txt` — Technical whitepaper specifying the Ravel Object Model, Resolution Ladder, and Harmonic Homeostasis math.
- `rava/` — CLI launcher script (`./rava`) for building and running the simulator.
- `outline.md` & `notes_00.md` — Architectural specifications and design notes.

---

## How to Run

### Prerequisites
- **Rust toolchain** (`rustup`, `cargo`, `rustc >= 1.78`)
- Standard OS graphics/windowing dependencies for Bevy (Metal on macOS, Vulkan/X11/Wayland on Linux, DX12 on Windows).

### Build & Run the Interactive Simulator
```bash
# Run directly via the launcher script:
./rava

# Or via Cargo inside the simulator workspace:
cd simulator
cargo run --release -p ravel_sim
```

### Run Library Checks & Tests
```bash
cd simulator
cargo test --workspace
```

---

## Core Concepts (The Physics of Quality)

1. **The 64-Gate Harmonic Chord (`[i32; 64]`)**:
   - `+` (Yang / Active): The entity generates or radiates that archetype.
   - `-` (Yin / Receptive): The entity consumes or seeks that archetype.
   - `0` (Dormant): The entity does not interact on that channel.
2. **The Resolution Ladder ($R = 1 \dots 64$)**:
   - The simulation progresses through 7 Epochs (`Primordial R=1` $\rightarrow$ `Duality R=2` $\rightarrow$ `Elemental R=4` $\rightarrow$ `Tribal R=8` $\rightarrow$ `Societal R=16` $\rightarrow$ `Planetary R=32` $\rightarrow$ `Universal R=64`), downsampling chords to match each entity's perceptual bandwidth.
3. **Harmonic Homeostasis (Ecological Spawner)**:
   - The universe acts as a continuous sensor: $\text{GlobalChord}_i = \sum \text{Entity.Chord}_i$. When an imbalance spikes (e.g. Gate 30 Fire), the spawner synthesizes counter-chord entities (e.g. Gate 29 Water/Abyss) to restore equilibrium.

---

## Roadmap & Planned Work Ahead (45+ Hours)

- [ ] **Phase 1 — Resolution Ladder & Downsampling Math (`ravel_ecs`) (~8 hrs)**
  - [ ] Implement lossless-to-blocky chord downsampling across all 7 Epochs ($R = 1, 2, 4, 8, 16, 32, 64$) and harmonic inversion rules (flipping constructive vs. destructive interactions based on target dissonance).
  - [ ] Add comprehensive unit and property-based tests for `Chord` arithmetic, dot-product resonance, and mutation matrices.
- [ ] **Phase 2 — Astronomical Clock & Planetary Interference Weather (~8 hrs)**
  - [ ] Implement the 64-House orbital clock in `ravel_ecs` simulating planetary transits that apply global interference buffs/attenuations to specific Gates.
  - [ ] Add randomized high-frequency noise ("cosmic static") that disproportionately perturbs high-resolution ($R=64$) interactions compared to resilient low-resolution ($R=4$) interactions.
- [ ] **Phase 3 — Procedural Name & Species Generator (~7 hrs)**
  - [ ] Build the combinatoric Name Engine combining dominant Yin/Yang Gate themes (e.g., `Gate 30 Fire + Gate 1 Creative`) to procedurally name emergent species, actions, and trade currencies.
  - [ ] Implement dynamic Yin/Yang currency flows and trade harmony multipliers between agents.
- [ ] **Phase 4 — Interactive Rave Mandala Radar & Bit-Depth Vision Shaders (`ravel_sim`) (~10 hrs)**
  - [ ] Upgrade the 64-spoke Rave Mandala radar chart in `ravel_sim` with real-time per-gate hover inspection, polarity color coding (Gold = Yang, Cyan = Yin), and historical imbalance sparklines.
  - [ ] Implement Bit-Depth Vision shaders rendering neighboring entities according to the observer's current Resolution tier and chord alignment.
- [ ] **Phase 5 — H3 Spatial Indexing, Deterministic Snapshots & gRPC Node API (~10 hrs)**
  - [ ] Integrate H3 hexagonal spatial indexing for territory partitioning and local-scope chord aggregation.
  - [ ] Add deterministic simulation state save/load snapshots (`serde` JSON/bincode) and headless benchmark runner.

---

## License

Released under the [MIT License](LICENSE).
