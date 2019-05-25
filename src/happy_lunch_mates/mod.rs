pub mod slack_mock;

#[derive(Debug)]
pub struct HappyLunchMates<'hlm> {
    slack_channel: &'hlm slack_mock::SlackChannel<'hlm>,
    n_groups: u8,
    restaurant_names: Vec<&'hlm str>
}

impl<'hlm> HappyLunchMates<'hlm> {
    pub fn new (slack_channel: &'hlm slack_mock::SlackChannel, n_groups: &u8, restaurant_names: &Vec<&'hlm str>) -> HappyLunchMates<'hlm> {
        HappyLunchMates {
            slack_channel: slack_channel,
            n_groups: *n_groups,
            restaurant_names: restaurant_names.to_vec()
        }
    }
}