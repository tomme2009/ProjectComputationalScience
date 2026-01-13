use crate::{
    agent::Agent,
    party::Parties,
    probability::{Preferences, Probability},
};
use rand::distr::Distribution;
use rand_distr::Normal;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq)]
pub enum VotingSystem {
    FPTP,     // First Past The Post
    TwoRound, // Two-round system
}
#[derive(Debug)]
pub enum ElectionResult {
    FPTP(usize, HashMap<usize, usize>), // (winner, votes per party)
    TwoRound(
        usize, // winner
        usize, // runner up
        (
            HashMap<usize, usize>, // votes in first round per party
            HashMap<usize, usize>, // votes in second round per party
        ),
    ),
}

pub trait Network {
    fn get_agents(&self) -> &[Agent];

    fn get_agents_mut(&mut self) -> &mut [Agent];

    /*
     * Network is generated with all agents having no vote history
     * This function initializes for 1-new_voters percent of all the voters
     * the last party they voted for
     */
    fn initialize_previous_votes(&mut self, parties: &Parties, new_voters: Probability) {
        for agent in self.get_agents_mut() {
            if rand::random_range(0.0..1.0) > new_voters.get_value() {
                agent.set_last_vote(
                    agent
                        .get_party_preferences_distance(parties, &parties.get_parties())
                        .choose(),
                );
            }
        }
    }

    fn update_previous_votes(&mut self) {
        for agent in self.get_agents_mut() {
            if let Some(vote) = agent.get_current_vote() {
                agent.set_last_vote(vote);
            } else {
                agent.unset_last_vote();
            }
        }
    }

    /*
     * Holds an election on the network with the given parties and the given votingsystem,
     * returns an election result.
     */
    fn hold_election(&mut self, parties: &Parties, system: VotingSystem) -> ElectionResult {
        match system {
            VotingSystem::FPTP => {
                let party_order = parties.get_parties();

                let mut votes: Vec<usize> = vec![0; parties.len()];
                let agents = self.get_agents_mut();

                // Get each agent's vote
                for agent in 0..agents.len() {
                    // Find the support for all the parties among this agent's neighbors
                    let mut neighbor_support: Vec<u32> = vec![0; parties.len()];
                    for neighbor_index in agents[agent].get_friends() {
                        // Only include party support if neighbor actually voted
                        if let Some(vote) = agents[neighbor_index].get_last_vote() {
                            if let Ok(party_index) = party_order.binary_search(&vote) {
                                neighbor_support[party_index] += 1;
                            }
                        }
                    }

                    // Turn neighbor support into percentages
                    let normalized_neighbor_support = Preferences::new_normalize(&neighbor_support);

                    // Get this agent's vote
                    let vote =
                        agents[agent].vote(parties, &party_order, normalized_neighbor_support);
                    votes[party_order.binary_search(&vote).unwrap()] += 1;
                }

                self.update_previous_votes();

                // Get the party with the most votes
                // If there is a tie, the party appearing latest in the list wins
                let winner = votes
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(index, _)| index)
                    .unwrap();

                let mut votes_per_party: HashMap<usize, usize> =
                    HashMap::with_capacity(party_order.len());
                for (i, party_id) in party_order.iter().enumerate() {
                    votes_per_party.insert(*party_id, votes[i]);
                }

                ElectionResult::FPTP(winner, votes_per_party)
            }
            VotingSystem::TwoRound => {
                let party_order = parties.get_parties();

                let mut votes: Vec<usize> = vec![0; parties.len()];
                let agents = self.get_agents_mut();

                // Get each agent's vote for round 1
                for agent in 0..agents.len() {
                    // Find the support for all the parties among this agent's neighbors
                    let mut neighbor_support: Vec<u32> = vec![0; parties.len()];
                    for neighbor_index in agents[agent].get_friends() {
                        // Only include party support if neighbor actually voted
                        if let Some(vote) = agents[neighbor_index].get_last_vote() {
                            if let Ok(party_index) = party_order.binary_search(&vote) {
                                neighbor_support[party_index] += 1;
                            }
                        }
                    }

                    // Turn neighbor support into percentages
                    let normalized_neighbor_support = Preferences::new_normalize(&neighbor_support);

                    // Get this agent's vote
                    let vote =
                        agents[agent].vote(parties, &party_order, normalized_neighbor_support);
                    votes[party_order.binary_search(&vote).unwrap()] += 1;
                }

                self.update_previous_votes();

                // Reobtain the mutable reference
                let agents = self.get_agents_mut();

                // Find the winner and runner-up of round 1
                let (mut winner_1, mut runner_up) =
                    if votes[0] > votes[1] { (0, 1) } else { (1, 0) };

                // println!("Initial: {} {}", winner_1, runner_up);

                for i in 2..votes.len() {
                    if votes[i] >= votes[winner_1] {
                        runner_up = winner_1;
                        winner_1 = i;
                    } else if votes[i] >= votes[runner_up] {
                        runner_up = i;
                    }
                    // println!("New: {} {}", winner_1, runner_up);
                }

                (winner_1, runner_up) = (party_order[winner_1], party_order[runner_up]);

                // println!("{:?} {:?} {} {}", votes, party_order, winner_1, runner_up);

                let round2_parties: Vec<usize> = vec![winner_1, runner_up];
                let mut round2_votes: Vec<usize> = vec![0; 2];
                // round2_parties.push(parties[winner_1].clone());
                // round2_parties.push(parties[runner_up].clone());

                // Get each agent's vote for round 2
                for agent in 0..agents.len() {
                    // Find the support for all the parties among this agent's neighbors
                    let mut neighbor_support: Vec<u32> = vec![0; 2];
                    for neighbor_index in agents[agent].get_friends() {
                        // Only include party support if neighbor actually voted
                        if let Some(vote) = agents[neighbor_index].get_current_vote() {
                            if vote == winner_1 {
                                neighbor_support[0] += 1;
                            } else if vote == runner_up {
                                neighbor_support[1] += 1;
                            }
                        }
                    }

                    // Turn neighbor support into percentages
                    let normalized_neighbor_support = Preferences::new_normalize(&neighbor_support);

                    // Get this agent's vote
                    let vote =
                        agents[agent].vote(parties, &round2_parties, normalized_neighbor_support);
                    if vote == winner_1 {
                        round2_votes[0] += 1;
                    } else if vote == runner_up {
                        round2_votes[1] += 1;
                    } else {
                        panic!("Agent voted for an eliminated party");
                    }
                    // println!("{vote}");
                }

                // Get the party with the most votes
                // If there is a tie, the party appearing latest in the list wins
                let winner = round2_parties[round2_votes
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|(index, _)| index)
                    .unwrap()];

                let mut votes_per_party_1: HashMap<usize, usize> =
                    HashMap::with_capacity(party_order.len());
                for (i, party_id) in party_order.iter().enumerate() {
                    votes_per_party_1.insert(*party_id, votes[i]);
                }
                let mut votes_per_party_2: HashMap<usize, usize> = HashMap::with_capacity(2);
                for (i, party_id) in round2_parties.iter().enumerate() {
                    votes_per_party_2.insert(*party_id, round2_votes[i]);
                }

                ElectionResult::TwoRound(winner, runner_up, (votes_per_party_1, votes_per_party_2))
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
