mod happy_lunch_mates;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let channel_memebers = vec!["Hatef",
                                 "Connor", 
                                 "Nigel", 
                                 "Jono", 
                                 "Jon", 
                                 "Kevin",
                                 "Holly",
                                 "Olivia",
                                 "Geordie",
                                 "Suzanne"];

    let restaurant_names = vec!["Pho", "Argos", "Soosh", "RailTown", "Bao", "BrewHall"];
    let n_groups = 3;

    let channel_name: &str = "lunch-crew";

    // let AUTH_TOKEN: std::result::Result<std::string::String, std::env::VarError> = std::env::var("AUTH_TOKEN");

    // println!("{:?}", AUTH_TOKEN);


    let mut slack: happy_lunch_mates::slack_driver::SlackDriver = happy_lunch_mates::slack_driver::SlackDriver::new();

    println!("{:?}", slack._get_users_list());

    // println!("channel members: {:?}", slack.channel_members);

    // let mut hlm_test: happy_lunch_mates::HappyLunchMates = happy_lunch_mates::HappyLunchMates::new(&slack, &n_groups, &restaurant_names);

    // hlm_test.assign_to_group();


}



