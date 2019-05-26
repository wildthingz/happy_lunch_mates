mod happy_lunch_mates;

fn main() {
    let channel_memebers = vec![("Hatef", "Khadivi"),
                                 ("Connor", "Shannon"), 
                                 ("Nigel", "Myers"), 
                                 ("Jono", "Chapple"), 
                                 ("Jon", "Moore"), 
                                 ("Kevin", "Wu"),
                                 ("Holly", "Peck"),
                                 ("Olivia", "Norton"),
                                 ("Geordie", "Rose"),
                                 ("Suzanne", "Gildert")];

    let restaurant_names = vec!["Pho", "Argos", "Soosh", "RailTown", "Bao", "BrewHall"];
    let n_groups = 3;

    let channel_name: &str = "lunch-crew";

    let mut slack: happy_lunch_mates::slack_driver::SlackDriver = happy_lunch_mates::slack_driver::SlackDriver::new(&channel_name);

    slack.get_members(&channel_memebers);

    println!("channel members: {:?}", slack.channel_members);

    let mut hlm_test: happy_lunch_mates::HappyLunchMates = happy_lunch_mates::HappyLunchMates::new(&slack, &n_groups, &restaurant_names);

    hlm_test.assign_to_group();
}



