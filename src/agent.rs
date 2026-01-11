use crate::party::Party;
use crate::probability::{Preferences, Probability};
use std::collections::HashMap;

/*
 * Data type representing relationships between agents
 */
#[derive(Debug)]
struct Relationship {
    // agent: usize,          // Index of the agent on the other side of the relationship
    strength: Probability, // Strength of the relationship
}

/*
 * Data type representing agents in the simulation
 */
#[derive(Debug)]
pub struct Agent {
    friends: HashMap<usize, Relationship>, // List of agents this agent has a two-way relationship with
    preferences: Preferences,              // List of standpoints of the agent
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
                        strength: Probability::new(*strength),
                    },
                )
            })
            .collect();

        Agent {
            friends,
            preferences,
        }
    }

    pub fn get_preferences(&self) -> &Preferences {
        &self.preferences
    }

    /*
     * Calculates the distance from this agent to each given party
     * and returns a Preferences object with the normalized distances
     * Function requires that the Preferences of the agent and all the parties
     * have the same dimension
     */
    pub fn get_party_preferences(&self, parties: &[Party]) -> Preferences {
        let party_preferences = parties
            .iter()
            .map(|party| self.preferences.distance(party.get_preferences()).unwrap())
            .collect::<Vec<f64>>();

        Preferences::new_normalize(&party_preferences).unwrap()
    }
}
