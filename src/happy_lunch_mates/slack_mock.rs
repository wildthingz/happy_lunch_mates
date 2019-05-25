use std::vec::Vec;

#[derive(Debug)]
pub struct SlackChannel<'sch> {
    channel_name : &'sch str,
    pub channel_members: Vec<(&'sch str, &'sch str)>,
}

impl<'sch> SlackChannel<'sch> {
    pub fn new (channel_name: &str) -> SlackChannel {
        SlackChannel {
            channel_name: channel_name,
            channel_members: Vec::new()
        }
    }

    pub fn add_members(&mut self, channel_members: &Vec<(&'sch str, &'sch str)>) {
        self.channel_members = channel_members.to_vec();
    }
}