use crate::{
    agent::Agent,
    party::Party,
    probability::{Preferences, Probability},
};
use rand::distr::Distribution;
use rand_distr::Normal;
use std::collections::HashSet;

#[derive(PartialEq, Eq)]
pub enum VotingSystem {
    FPTP, // First Past The Post
}
#[derive(Debug)]
pub enum ElectionResult {
    FPTP(usize, Vec<usize>), // (winner, votes per party)
}

pub trait Network {
    fn get_agents(&self) -> &[Agent];

    fn get_agents_mut(&mut self) -> &mut [Agent];

    /*
     * Network is generated with all agents having no vote history
     * This function initializes for 1-new_voters percent of all the voters
     * the last party they voted for
     */
    fn initialize_previous_votes(&mut self, parties: &Vec<Party>, new_voters: Probability) {
        for agent in self.get_agents_mut() {
            if rand::random_range(0.0..1.0) > new_voters.get_value() {
                agent.set_last_vote(
                    agent
                        .get_party_preferences_distance(parties)
                        .get_vote(Probability::new(rand::random_range(0.0..1.0))),
                );
            }
        }
    }

    /*
     * Holds an election on the network with the given parties and the given votingsystem,
     * returns an election result.
     */
    fn hold_election(&mut self, parties: &Vec<Party>, system: VotingSystem) -> ElectionResult {
        match system {
            VotingSystem::FPTP => {
                let mut votes: Vec<usize> = vec![0; parties.len()];
                let agents = self.get_agents_mut();

                // Get each agent's vote
                for agent in 0..agents.len() {
                    // Find the support for all the parties among this agent's neighbors
                    let mut neighbor_support: Vec<u32> = vec![0; parties.len()];
                    for neighbor_index in agents[agent].get_friends() {
                        // Only include party support if neighbor actually voted
                        if let Some(vote) = agents[neighbor_index].get_last_vote() {
                            neighbor_support[vote] += 1;
                        }
                    }

                    // Turn neighbor support into percentages
                    let normalized_neighbor_support =
                        Preferences::new_normalize(&neighbor_support).unwrap();

                    // Get this agent's vote
                    let vote = agents[agent].vote(parties, normalized_neighbor_support);
                    votes[vote] += 1;
                }

                // Get the party with the most votes
                // If there is a tie, the party appearing latest in the list wins
                let winner = votes
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(index, _)| index)
                    .unwrap();

                for agent in agents {
                    agent.update_last_vote();
                }

                ElectionResult::FPTP(winner, votes)
            }
        }
    }
}

/*
 * Returns modulo m of (a + b)
 */
fn wrapping_add(a: usize, b: usize, m: usize) -> usize {
    (a + b) % m
}

/*
 * Returns modulo m of (a - b)
 * Assumes that a > (-m)
 */
fn wrapping_sub(a: usize, b: usize, m: usize) -> usize {
    (a + m - (b % m)) % m
}

pub struct WattsStrogatz {
    agents: Vec<Agent>,
}

impl WattsStrogatz {
    pub fn new(
        num_nodes: usize,
        mean_degree: usize,
        beta: f64,
        num_preferences: usize,
    ) -> Result<WattsStrogatz, String> {
        // Check all the requirements for the Watts-Strogatz model
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
            return Err(format!(
                "The following must hold: N >= K >= ln N >= 1, currently: {} >= {} >= {} >= 1",
                num_nodes,
                mean_degree,
                (num_nodes as f64).ln()
            ));
        }

        let mut network = WattsStrogatz { agents: vec![] };

        let mut rng = rand::rng();
        let normal = Normal::new(0.5, 0.13).unwrap();

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

            // Initialize this agent's preferences with a normal distribution
            let values: Vec<f64> = (0..num_preferences)
                .map(|_| normal.sample(&mut rng))
                .collect();

            network
                .agents
                .push(Agent::new(&friends, Preferences::new(&values)));
        }

        Ok(network)
    }
}

impl Network for WattsStrogatz {
    fn get_agents(&self) -> &[Agent] {
        &self.agents
    }
    fn get_agents_mut(&mut self) -> &mut [Agent] {
        &mut self.agents
    }
}
