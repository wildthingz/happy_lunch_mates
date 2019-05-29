mod happy_lunch_mates;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let restaurant_names = vec!["Pho", "Argos", "Soosh", "RailTown", "Bao", "BrewHall"];
    let n_groups = 5;

    let channel_name: &str = "lunch-crew";

    let slack: happy_lunch_mates::slack_driver::SlackDriver = happy_lunch_mates::slack_driver::SlackDriver::new(&channel_name);

    let mut hlm_test: happy_lunch_mates::HappyLunchMates = happy_lunch_mates::HappyLunchMates::new(&slack, &n_groups, &restaurant_names);

    hlm_test.assign_to_group(); 

}



