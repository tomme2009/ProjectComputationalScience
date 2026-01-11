use election_simulator::agent::Agent;
use election_simulator::network::WattsStrogatz;
use election_simulator::party::Party;
use election_simulator::probability::{Preferences, Probability};

#[derive(PartialEq, Eq)]
enum VotingSystem {
    FPTP, // First Past The Post
}

#[derive(Debug)]
enum ElectionResult {
    FPTP(usize, Vec<usize>), // (winner, votes per party)
}

fn hold_election(agents: &[Agent], parties: &[Party], system: VotingSystem) -> ElectionResult {
    match system {
        VotingSystem::FPTP => {
            let mut votes: Vec<usize> = vec![0; parties.len()];

            // Get each agent's vote
            for agent in agents {
                let vote = agent
                    .get_party_preferences(parties)
                    .get_vote(Probability::new(rand::random_range(0.0..=1.0)));
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

            ElectionResult::FPTP(winner, votes)
        }
    }
}

fn initialize_agents(num_agents: usize, num_preferences: usize) -> Vec<Agent> {
    let mut agents = Vec::new();

    for _i in 0..num_agents {
        // Randomly initialize this agent's preferences
        let values: Vec<f64> = (0..num_preferences)
            .map(|_| rand::random_range(0.0..=1.0))
            .collect();
        agents.push(Agent::new(&[], Preferences::new(&values)));
    }

    agents
}

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

fn main() {
    const NUM_PREFERENCES: usize = 1;
    let network = WattsStrogatz::new(1000, 24, 0.7, NUM_PREFERENCES).unwrap();
    let agents = network.get_agents();
    // let agents = initialize_agents(1000, NUM_PREFERENCES);
    // let parties = initialize_parties(2, NUM_PREFERENCES);
    let parties = initialize_parties_with_names(2, NUM_PREFERENCES, &["Left", "Right"]);

    let election_result = hold_election(agents, &parties, VotingSystem::FPTP);
    match election_result {
        ElectionResult::FPTP(winner, votes) => {
            println!("The winner is {}", parties[winner].get_name());
            for (i, num_votes) in votes.iter().enumerate() {
                println!("Party {} got {} votes", parties[i].get_name(), num_votes);
            }
        }
    }
}
