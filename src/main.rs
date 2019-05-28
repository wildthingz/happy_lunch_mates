mod happy_lunch_mates;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let restaurant_names = vec!["Pho", "Argos", "Soosh", "RailTown", "Bao", "BrewHall"];
    let n_groups = 3;

    let channel_name: &str = "lunch-crew";

    // let AUTH_TOKEN: std::result::Result<std::string::String, std::env::VarError> = std::env::var("AUTH_TOKEN");

    // println!("{:?}", AUTH_TOKEN);


    let mut slack: happy_lunch_mates::slack_driver::SlackDriver = happy_lunch_mates::slack_driver::SlackDriver::new(&channel_name);

    let channel_memebers = slack.get_members_from_channel();

    // let mut hlm_test: happy_lunch_mates::HappyLunchMates = happy_lunch_mates::HappyLunchMates::new(&slack, &n_groups, &restaurant_names);

    // hlm_test.assign_to_group(); 

}



