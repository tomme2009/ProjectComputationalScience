use crate::probability::Preferences;

/*
 * Data type representing parties that agents can vote for
 */
#[derive(Debug)]
pub struct Party {
    name: String,             // Name of the party
    preferences: Preferences, // Standpoints of the party
}

impl Party {
    pub fn new(name: String, preferences: Preferences) -> Party {
        Party { name, preferences }
    }

    pub fn get_preferences(&self) -> &Preferences {
        &self.preferences
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
