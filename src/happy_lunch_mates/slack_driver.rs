use std::vec::Vec;
extern crate slack_api as slack;

use dotenv::dotenv;
use slack::requests::{Error, Client};

#[derive(Debug)]
pub struct SlackDriver<'sd> {
    slack_client: &'sd slack::requests::Client,
    slack_token : &'sd str,
    channel_name : &'sd str,
    members_list : &'sd Vec<String>
}

impl<'sd> SlackDriver<'sd> {
    pub fn new(channel_name: &str) -> SlackDriver {
        let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set in .env");
        let client: slack::requests::Client = slack::default_client().unwrap();

        SlackDriver {
            slack_client: &client,
            slack_token : &token,
            channel_name : channel_name,
            members_list : &Vec::new()
        }
    }

    pub fn get_members_from_channel(&mut self) -> Vec<String>{

        let channels_list: Vec<slack::Channel> = self._get_channels_list();
        let users_list:    Vec<slack::User>    = self._get_users_list();

        let mut user_ID_list_in_channel: Vec<String> = Vec::new();

        let mut user_name_list_in_channel: &'sd Vec<String> = &Vec::new();

        for channel in channels_list {
            if channel.name.unwrap() == self.channel_name {
                user_ID_list_in_channel = channel.members.unwrap();
                break;
            }
        } 

        for user in users_list {
            if user_ID_list_in_channel.contains(&user.id.unwrap()) {
                user_name_list_in_channel.push(user.name.unwrap())
            }
        }
        self.members_list = user_name_list_in_channel;
        return user_name_list_in_channel.to_vec();
    }

    fn _get_channels_list(&self) -> Vec<slack::Channel> {
        let mut channels_list: Vec<slack::Channel> = Vec::new();
        let response = slack::channels::list(self.slack_client,
                                self.slack_token,
                                &slack::channels::ListRequest::default());

        if let Ok(response) = response {
            if let Some(channels) = response.channels {
                channels_list = channels;    
            }
        } else {
        println!("{:?}", response);
        }
        return channels_list;
    }

    fn _get_users_list(&self) -> Vec<slack::User> {
        let mut users_list: Vec<slack::User> = Vec::new();
        let response = slack::users::list(self.slack_client,
                                self.slack_token,
                                &slack::users::ListRequest::default());

        if let Ok(response) = response {
            if let Some(users) = response.members {
                users_list = users;    
            }
        } else {
        println!("{:?}", response);
        }
        return users_list;
    }
}
