mod happy_lunch_mates;

fn main() {
    let channel_memebers = vec![("Hatef", "Khadivi"),
                                 ("Connor", "Shannon"), 
                                 ("Nigel", "Myers"), 
                                 ("Jono", "Chapple"), 
                                 ("Jon", "Moore"), 
                                 ("Kevin", "Wu")];

    let restaurant_names = vec!["Pho", "Argos", "Soosh", "RailTown", "Bao", "BrewHall"];
    let n_groups = 2;

    let channel_name: &str = "lunch-crew";

    println!("channel members: {:?}", channel_memebers);

    let mut slack: happy_lunch_mates::slack_mock::SlackChannel = happy_lunch_mates::slack_mock::SlackChannel::new(&channel_name);

    println!("channel members: {:?}", slack.channel_members);

    slack.add_members(&channel_memebers);

    println!("channel members: {:?}", slack.channel_members);

    let mut hlm_test: happy_lunch_mates::HappyLunchMates = happy_lunch_mates::HappyLunchMates::new(&slack, &n_groups, &restaurant_names);
}



