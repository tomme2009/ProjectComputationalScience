use crate::party::Parties;
use crate::probability::{Preferences, Probability};
use rand::distr::Distribution;
use rand_distr::Normal;
use std::collections::HashMap;

/*
 * Data type representing relationships between agents
 */
#[derive(Debug)]
struct Relationship {
    _strength: Probability, // Strength of the relationship
}

/*
 * Data type representing agents in the simulation
 */
#[derive(Debug)]
pub struct Agent {
    friends: HashMap<usize, Relationship>, // List of agents this agent has a two-way relationship with
    preferences: Preferences,              // List of standpoints of the agent
    last_vote: Option<usize>,              // Last party this agent voted for
    current_vote: Option<usize>,           // Party this agent votes for in this election
    loyalty: Probability,                  // Loyalty of this agent to its last vote
    susceptibility: Probability,           // Susceptitibility of this agent to peer pressure
}

impl Agent {
    pub fn new(friends: &[(usize, f64)], preferences: Preferences) -> Agent {
        // Convert the raw friends data in Relationships
        let friends = friends
            .iter()
            .map(|(agent, strength)| {
                (
                    *agent,
                    Relationship {
                        _strength: Probability::new(*strength),
                    },
                )
            })
            .collect();

        // Initialize loyalty and susceptibility according to a normal distribution
        let mut rng = rand::rng();
        let normal = Normal::new(0.5, 0.13).unwrap();

        let loyalty = Probability::new(normal.sample(&mut rng));
        let susceptibility = Probability::new(normal.sample(&mut rng));

        Agent {
            friends,
            preferences,
            last_vote: None,
            current_vote: None,
            loyalty,
            susceptibility,
        }
    }

    pub fn get_preferences(&self) -> &Preferences {
        &self.preferences
    }

    // Returns a list of this agent's friends
    // only the indices of the friends are provided
    pub fn get_friends(&self) -> Vec<usize> {
        self.friends.keys().map(|friend| *friend).collect()
    }

    /*
     * Calculates the distance from this agent to each given party
     * and returns a Preferences object with the scores for each party
     * Function requires that the Preferences of the agent and all the parties
     * have the same dimension
     */
    pub fn get_party_preferences_distance(
        &self,
        parties: &Parties,
        party_order: &[usize],
    ) -> Preferences {
        const DISTANCE_MULTIPLIER: f64 = -10.0;
        let party_preferences = party_order
            .iter()
            .map(|party_index| {
                let party = parties.get_party(*party_index).unwrap();
                (DISTANCE_MULTIPLIER * self.preferences.distance(party.get_preferences()).unwrap())
                    .exp()
            })
            .collect::<Vec<f64>>();

        Preferences::new_normalize(&party_preferences)
    }

    /*
     * Calculates the agent's preferences for each given party
     * according to the following rule:
     * exp(peer_pressure * susceptibility * neighbor_support
     *  + external_events * party_attractiveness)
     */
    fn get_party_preferences(
        &self,
        parties: &Parties,
        party_order: &[usize],
        neighbor_support: Preferences,
    ) -> Preferences {
        const PEER_PRESSURE: f64 = 0.3;
        const EXTERNAL_EVENTS: f64 = 0.5;
        let party_preferences = party_order
            .iter()
            .enumerate()
            .map(|(i, party_index)| {
                let party = parties.get_party(*party_index).unwrap();
                (PEER_PRESSURE
                    * self.susceptibility.get_value()
                    * neighbor_support.get_preference(i).get_value()
                    + EXTERNAL_EVENTS * party.get_attractiveness())
                .exp()
            })
            .collect::<Vec<f64>>();

        Preferences::new_normalize(&party_preferences)
    }

    /*
     * Get this agent's vote given a list of parties
     * and the agent's neighbors' support for the those parties
     */
    pub fn vote(
        &mut self,
        parties: &Parties,
        party_order: &[usize],
        neighbor_support: Preferences,
    ) -> usize {
        let party_scores = self.get_party_preferences(parties, party_order, neighbor_support);
        let party_distances = self.get_party_preferences_distance(parties, party_order);

        let party_preferences = party_distances * party_scores;

        // Change vote with chance 1-loyalty
        if rand::random_range(0.0..1.0) > self.loyalty.get_value()
            || self
                .last_vote
                .is_none_or(|vote| !party_order.contains(&vote))
        {
            // Change vote
            let vote_index = party_preferences.choose();
            let vote = party_order[vote_index];
            self.current_vote = Some(vote);
            vote
        } else {
            // Don't change vote
            self.last_vote.unwrap()
        }
    }

    pub fn get_last_vote(&self) -> Option<usize> {
        self.last_vote
    }

    pub fn set_last_vote(&mut self, vote: usize) {
        self.last_vote = Some(vote);
    }

    pub fn unset_last_vote(&mut self) {
        self.last_vote = None;
    }

    pub fn get_current_vote(&self) -> Option<usize> {
        self.current_vote
    }
}
