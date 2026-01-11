use crate::{agent::Agent, probability::Preferences};
use std::collections::HashSet;

pub struct WattsStrogatz {
    agents: Vec<Agent>,
}

fn wrapping_add(a: usize, b: usize, m: usize) -> usize {
    (a + b) % m
}

fn wrapping_sub(a: usize, b: usize, m: usize) -> usize {
    (a + m - (b % m)) % m
}

impl WattsStrogatz {
    pub fn new(
        num_nodes: usize,
        mean_degree: usize,
        beta: f64,
        num_preferences: usize,
    ) -> Result<WattsStrogatz, String> {
        if mean_degree % 2 == 1 {
            return Err(String::from("Mean degree must be an even integer"));
        }
        if !(0.0..=1.0).contains(&beta) {
            return Err(String::from("Beta must be a number in [0,1]"));
        }
        if num_nodes < mean_degree
            || (mean_degree as f64) < (num_nodes as f64).ln()
            || (num_nodes as f64).ln() < 1.0
        {
            println!(
                "{} >= {} >= {} >= 1",
                num_nodes,
                mean_degree,
                (num_nodes as f64).ln()
            );
            return Err(String::from("The following must hold: N >= K >= ln N >= 1"));
        }

        let mut network = WattsStrogatz { agents: vec![] };

        // Construct a regular ring latice
        for n in 0..num_nodes {
            // Initialize neighbors
            let mut neighbors: HashSet<usize> = (1..=mean_degree / 2)
                .flat_map(|offset| {
                    [
                        wrapping_sub(n, offset, num_nodes),
                        wrapping_add(n, offset, num_nodes),
                    ]
                })
                .collect();

            if mean_degree != num_nodes {
                for offset in 1..=mean_degree / 2 {
                    // For each right-hand neighbor
                    // randomize neighbor with chance beta
                    if beta > rand::random_range(0.0..=1.0) {
                        let old_neighbor = wrapping_add(n, offset, num_nodes);
                        let new_neighbor = loop {
                            // Loop till we find a valid candidate
                            let candidate = rand::random_range(0..num_nodes);
                            if candidate != n && !neighbors.contains(&candidate) {
                                break candidate;
                            }
                        };

                        neighbors.remove(&old_neighbor);
                        neighbors.insert(new_neighbor);
                    }
                }
            }

            // Initialize all friends with a strength of 1.0
            let friends: Vec<(usize, f64)> =
                neighbors.iter().map(|neighbor| (*neighbor, 1.0)).collect();

            // Initialize this agent's preferences
            let values: Vec<f64> = (0..num_preferences)
                .map(|_| rand::random_range(0.0..=1.0))
                .collect();

            network
                .agents
                .push(Agent::new(&friends, Preferences::new(&values)));
        }

        Ok(network)
    }

    pub fn get_agents(&self) -> &Vec<Agent> {
        &self.agents
    }
}
