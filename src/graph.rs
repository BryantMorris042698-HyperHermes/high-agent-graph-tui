//! Layer 1: The Graph Core (graph.rs — core mathematical engine)
//! Pure computation. G = (V, E), modularity, coupling, cyclomatic, Φ(G), deviation detection.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub module: String,
    pub cyclomatic: f64,
    pub quality: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: f64,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DirectedGraph {
    pub nodes: HashMap<String, Node>,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GraphSnapshot {
    pub timestamp: u64,
    pub q: f64,           // modularity
    pub coupling: f64,    // mean inter-module coupling
    pub mean_v: f64,      // mean cyclomatic
    pub mean_quality: f64,
    pub n_nodes: usize,
    pub n_edges: usize,
    pub regime: String,
}

#[derive(Clone, Debug)]
pub struct SegmentedRegimeDetector {
    window: Vec<GraphSnapshot>,
    window_size: usize,
    threshold: f64,
}

impl SegmentedRegimeDetector {
    pub fn new(window_size: usize, threshold: f64) -> Self {
        Self { window: Vec::new(), window_size, threshold }
    }

    pub fn feed(&mut self, snapshot: GraphSnapshot) -> Option<Deviation> {
        self.window.push(snapshot.clone());
        if self.window.len() > self.window_size {
            self.window.remove(0);
        }
        if self.window.len() < 5 { return None; }

        // Compute baseline stats (simplified)
        let mean_q: f64 = self.window.iter().map(|s| s.q).sum::<f64>() / self.window.len() as f64;
        let mean_c: f64 = self.window.iter().map(|s| s.coupling).sum::<f64>() / self.window.len() as f64;
        let mean_v: f64 = self.window.iter().map(|s| s.mean_v).sum::<f64>() / self.window.len() as f64;

        let var_q = self.window.iter().map(|s| (s.q - mean_q).powi(2)).sum::<f64>() / self.window.len() as f64;
        let sigma_q = var_q.sqrt().max(0.0001);

        let z_q = (snapshot.q - mean_q) / sigma_q;
        let z_c = (snapshot.coupling - mean_c) / 0.1; // placeholder sigma
        let z_v = (snapshot.mean_v - mean_v) / 2.0;

        if z_q.abs() > self.threshold || z_c.abs() > self.threshold || z_v.abs() > self.threshold {
            let direction = if z_q > 0.0 && z_c < 0.0 && z_v < 0.0 { "Improving" } else if z_q < 0.0 || z_c > 0.0 || z_v > 0.0 { "Degrading" } else { "Mixed" };
            Some(Deviation { z_q, z_c, z_v, direction: direction.to_string() })
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Deviation {
    pub z_q: f64,
    pub z_c: f64,
    pub z_v: f64,
    pub direction: String,
}

impl DirectedGraph {
    pub fn new() -> Self { Self::default() }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }

    /// Newman-Girvan modularity (simplified single-module baseline for demo)
    pub fn modularity(&self) -> f64 {
        if self.nodes.is_empty() { return 0.0; }
        // Placeholder: real implementation would partition by module and compute Σ[(L_c/L) − (d_c/2L)²]
        let l = self.edges.len() as f64;
        if l == 0.0 { return 0.0; }
        // Demo: assume one big module for baseline
        0.65 + (self.nodes.len() as f64 / 100.0).min(0.25)
    }

    pub fn mean_coupling(&self) -> f64 {
        if self.edges.is_empty() { return 0.0; }
        let cross = self.edges.iter().filter(|e| {
            if let (Some(n1), Some(n2)) = (self.nodes.get(&e.from), self.nodes.get(&e.to)) {
                n1.module != n2.module
            } else { false }
        }).count() as f64;
        cross / self.nodes.len() as f64
    }

    pub fn mean_cyclomatic(&self) -> f64 {
        if self.nodes.is_empty() { return 0.0; }
        self.nodes.values().map(|n| n.cyclomatic).sum::<f64>() / self.nodes.len() as f64
    }

    pub fn mean_quality(&self) -> f64 {
        if self.nodes.is_empty() { return 0.0; }
        self.nodes.values().map(|n| n.quality).sum::<f64>() / self.nodes.len() as f64
    }

    pub fn snapshot(&self, regime: &str) -> GraphSnapshot {
        GraphSnapshot {
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
            q: self.modularity(),
            coupling: self.mean_coupling(),
            mean_v: self.mean_cyclomatic(),
            mean_quality: self.mean_quality(),
            n_nodes: self.nodes.len(),
            n_edges: self.edges.len(),
            regime: regime.to_string(),
        }
    }

    /// Core multi-objective score
    pub fn multi_objective(&self, coeffs: &Coeffs) -> f64 {
        let snap = self.snapshot("current");
        coeffs.alpha * snap.q - coeffs.beta * snap.coupling - coeffs.gamma * snap.mean_v
    }
}

#[derive(Clone, Debug)]
pub struct Coeffs {
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
}

impl Default for Coeffs {
    fn default() -> Self {
        Self { alpha: 1.0, beta: 0.6, gamma: 0.4 } // Hybrid
    }
}