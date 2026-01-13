use std::collections::HashMap;

use crate::probability::Preferences;

/*
 * Data type representing parties that agents can vote for
 */
#[derive(Debug, Clone)]
pub struct Party {
    name: String,             // Name of the party
    preferences: Preferences, // Standpoints of the party
    attractiveness: f64,      // Attractiveness of the party
}

impl Party {
    pub fn new(name: String, preferences: Preferences) -> Party {
        Party {
            name,
            preferences,
            attractiveness: 1.0,
        }
    }

    pub fn get_preferences(&self) -> &Preferences {
        &self.preferences
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_attractiveness(&self) -> f64 {
        self.attractiveness
    }

    pub fn set_attractiveness(&mut self, value: f64) {
        self.attractiveness = value;
    }
}

pub struct Parties {
    parties: HashMap<usize, Party>,
}

impl Parties {
    pub fn new(num_parties: usize, names: &[&str], preferences: &[&[f64]]) -> Parties {
        let mut parties = HashMap::with_capacity(num_parties);

        assert_eq!(num_parties, names.len());
        assert_eq!(num_parties, preferences.len());

        for i in 0..num_parties {
            parties.insert(
                i,
                Party::new(String::from(names[i]), Preferences::new(preferences[i])),
            );
        }

        Parties { parties }
    }

    pub fn new_random(num_parties: usize, num_preferences: usize) -> Parties {
        let mut parties = HashMap::with_capacity(num_parties);

        for i in 0..num_parties {
            // Name is just the party's index
            let name = String::from(char::from_digit(i as u32, 10).unwrap());

            // Randomly initialize this party's preferences
            let values: Vec<f64> = (0..num_preferences)
                .map(|_| rand::random_range(0.0..=1.0))
                .collect();
            parties.insert(i, Party::new(name, Preferences::new(&values)));
        }

        Parties { parties }
    }

    pub fn new_with_names(num_parties: usize, num_preferences: usize, names: &[&str]) -> Parties {
        let mut parties = HashMap::with_capacity(num_parties);

        assert_eq!(num_parties, names.len());

        for i in 0..num_parties {
            // Randomly initialize this party's preferences
            let values: Vec<f64> = (0..num_preferences)
                .map(|_| rand::random_range(0.0..=1.0))
                .collect();
            parties.insert(
                i,
                Party::new(String::from(names[i]), Preferences::new(&values)),
            );
        }

        Parties { parties }
    }

    pub fn get_party(&self, party: usize) -> Option<&Party> {
        self.parties.get(&party)
    }

    pub fn get_parties(&self) -> Vec<usize> {
        let mut party_order: Vec<usize> = self.parties.keys().map(|value| *value).collect();
        party_order.sort();
        party_order
    }

    pub fn len(&self) -> usize {
        self.parties.len()
    }
}
