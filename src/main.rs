use election_simulator::network::{ElectionResult, Network, VotingSystem, WattsStrogatz};
use election_simulator::party::Party;
use election_simulator::probability::{Preferences, Probability};

fn initialize_parties(num_parties: usize, num_preferences: usize) -> Vec<Party> {
    let mut parties = Vec::new();

    for i in 0..num_parties {
        // Name is just the party's index
        let name = String::from(char::from_digit(i as u32, 10).unwrap());

        // Randomly initialize this party's preferences
        let values: Vec<f64> = (0..num_preferences)
            .map(|_| rand::random_range(0.0..=1.0))
            .collect();
        parties.push(Party::new(name, Preferences::new(&values)));
    }

    parties
}

fn initialize_parties_with_names(
    num_parties: usize,
    num_preferences: usize,
    names: &[&str],
) -> Vec<Party> {
    let mut parties = Vec::new();

    assert_eq!(num_parties, names.len());

    for i in 0..num_parties {
        // Randomly initialize this party's preferences
        let values: Vec<f64> = (0..num_preferences)
            .map(|_| rand::random_range(0.0..=1.0))
            .collect();
        parties.push(Party::new(
            String::from(names[i]),
            Preferences::new(&values),
        ));
    }

    parties
}

fn initialize_parties_with_preferences(
    num_parties: usize,
    names: &[&str],
    preferences: &[&[f64]],
) -> Vec<Party> {
    let mut parties = Vec::new();

    assert_eq!(num_parties, names.len());
    assert_eq!(num_parties, preferences.len());

    for i in 0..num_parties {
        parties.push(Party::new(
            String::from(names[i]),
            Preferences::new(preferences[i]),
        ));
    }

    parties
}

fn main() {
    const NUM_PREFERENCES: usize = 1;
    let mut network = WattsStrogatz::new(1000, 24, 0.05, NUM_PREFERENCES).unwrap();

    let mut parties = initialize_parties_with_preferences(
        5,
        &["Extreme-Left", "Left", "Middle", "Right", "Extreme-Right"],
        &[&[0.1], &[0.3], &[0.5], &[0.7], &[0.9]],
    );

    network.initialize_previous_votes(&parties, Probability::new(0.05));

    for election_num in 0..100 {
        // let election_result = hold_election(network.get_agents_mut(), &parties, VotingSystem::FPTP);
        if election_num == 5 {
            parties[4].set_attractiveness(0.1);
            println!("{} had their attractiveness lowered", parties[4].get_name());
        }
        let election_result = network.hold_election(&parties, VotingSystem::FPTP);
        match election_result {
            ElectionResult::FPTP(winner, votes) => {
                println!("The winner is {}", parties[winner].get_name());
                for (i, num_votes) in votes.iter().enumerate() {
                    println!("Party {} got {} votes", parties[i].get_name(), num_votes);
                }
            }
        }
        println!();
    }
}
