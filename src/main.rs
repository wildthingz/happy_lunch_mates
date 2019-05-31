mod happy_lunch_mates;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let restaurant_names = vec!["Pho", "Argos", "Soosh", "RailTown", "Bao", "BrewHall"];
    let n_groups = 2;

    // let channel_name: &str = "lunch-crew";
    let channel_name: &str = "happy-lunch-mates";

    let message: &str =  "HAIL RUST!!";

    let slack: happy_lunch_mates::slack_driver::SlackDriver = happy_lunch_mates::slack_driver::SlackDriver::new(&channel_name);

    // slack.send_message(message);

    let mut hlm_test: happy_lunch_mates::HappyLunchMates = happy_lunch_mates::HappyLunchMates::new(&slack, &n_groups, &restaurant_names);

    let groups = hlm_test.assign_to_group(); 

    let mut test: String = "Groups are as follows: \n ".to_string();
    // slack.send_message("Groups are as follows: \n");
    for (i, group) in groups.iter().enumerate() {
        test.push_str(&format!("group {0:?} is: {1:?} \n ", i, group.to_vec().join(" and ")));
        
    }
    slack.send_message(&test);

    std::thread::sleep(std::time::Duration::from_millis(5000));

    slack.send_message("another test");
    println!("{:?}", test);
}



