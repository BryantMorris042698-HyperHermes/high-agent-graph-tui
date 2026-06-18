//! high-agent-graph-tui
//! Graph_x_0x0 core library — mathematical codebase intelligence for high-agent + Hermes ecosystem.
//! Public API re-exports the graph engine, regime orchestrator, and snapshot types.

pub mod graph;
pub mod core;
pub mod orchestrator;

pub use graph::{DirectedGraph, Node, Edge, GraphSnapshot, SegmentedRegimeDetector};
pub use core::RegimeEngine;
pub use orchestrator::{Orchestrator, Regime, RegimeTransition};