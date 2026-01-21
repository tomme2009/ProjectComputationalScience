use election_simulator::{
    network::{ElectionResult, Network, VotingSystem, WattsStrogatz},
    party::Parties,
    probability::Probability,
};

fn main() {
    const NUM_PREFERENCES: usize = 1;
    let mut network = WattsStrogatz::new(1000, 24, 0.05, NUM_PREFERENCES).unwrap();

    let parties = Parties::new(
        5,
        &["Extreme-Left", "Left", "Middle", "Right", "Extreme-Right"],
        &[&[0.1], &[0.3], &[0.5], &[0.7], &[0.9]],
    );

    network.initialize_previous_votes(&parties, Probability::new(0.05));

    for _election_num in 0..10 {
        // let election_result = hold_election(network.get_agents_mut(), &parties, VotingSystem::FPTP);
        // if election_num == 5 {
        //     parties[4].set_attractiveness(0.1);
        //     println!("{} had their attractiveness lowered", parties[4].get_name());
        // }
        let election_result = network.hold_election(&parties, VotingSystem::TwoRound);
        match election_result {
            ElectionResult::FPTP(winner, votes) => {
                println!(
                    "The winner is {}",
                    parties.get_party(winner).unwrap().get_name()
                );
                for (party_id, num_votes) in votes.iter() {
                    println!(
                        "Party {} got {} votes",
                        parties.get_party(*party_id).unwrap().get_name(),
                        num_votes
                    );
                }
            }
            ElectionResult::TwoRound(winner, runner_up, (votes_1, votes_2)) => {
                println!(
                    "The winner is {}",
                    parties.get_party(winner).unwrap().get_name()
                );
                // println!("Winner: {}, Runner-up: {}", winner, runner_up);
                for (party_id, num_votes) in votes_1.iter() {
                    if *party_id == winner {
                        println!(
                            "Party {} got {} votes in round 1, {} votes in round 2",
                            parties.get_party(*party_id).unwrap().get_name(),
                            num_votes,
                            votes_2.get(party_id).unwrap()
                        );
                    } else if *party_id == runner_up {
                        println!(
                            "Party {} got {} votes in round 1, {} votes in round 2",
                            parties.get_party(*party_id).unwrap().get_name(),
                            num_votes,
                            votes_2.get(party_id).unwrap()
                        );
                    } else {
                        println!(
                            "Party {} got {} votes in round 1",
                            parties.get_party(*party_id).unwrap().get_name(),
                            num_votes
                        );
                    }
                }
            }
        }
        println!();
    }
}
