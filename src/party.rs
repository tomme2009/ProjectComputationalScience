use crate::probability::Preferences;

/*
 * Data type representing parties that agents can vote for
 */
#[derive(Debug)]
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
}
