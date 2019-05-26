use std::vec::Vec;

#[derive(Debug)]
pub struct SlackDriver<'sch> {
    channel_name : &'sch str,
    pub channel_members: Vec<(&'sch str, &'sch str)>,
}

impl<'sch> SlackDriver<'sch> {
    pub fn new (channel_name: &str) -> SlackDriver {
        SlackDriver {
            channel_name: channel_name,
            channel_members: Vec::new()
        }
    }

    pub fn get_members(&mut self, channel_members: &Vec<(&'sch str, &'sch str)>) {
        self.channel_members = channel_members.to_vec();
    }
}