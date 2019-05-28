use std::vec::Vec;
extern crate slack_api as slack;

use dotenv::dotenv;
use slack::requests::{Error, Client};

#[derive(Debug)]
pub struct SlackDriver {
    slack_client: slack::requests::Client,
    slack_token : std::string::String,

    
}

impl SlackDriver {
    pub fn new() -> SlackDriver {
        let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set in .env");
        let client: slack::requests::Client = slack::default_client().unwrap();

        SlackDriver {
            slack_client: client,
            slack_token : token,
        }
    }


    // pub fn get_members(&mut self, channel_members: &Vec<&'sd str>) {
    //     self.channel_members = channel_members.to_vec();
    // }
    pub fn get_members(&self, channel_name: &str) {

        let response = slack::users::list(&self.slack_client,
                                        &self.slack_token,
                                        &slack::users::ListRequest::default());

        let response = slack::channels::list(&self.slack_client,
                                        &self.slack_token,
                                        &slack::channels::ListRequest::default());

        if let Ok(response) = response {
            // println!("{:?}", response.channels);
            if let Some(channels) = response.channels {
                println!("Got {} channels", channels.len());

                for channel in channels {
                    if &channel.name.unwrap() == channel_name {
                        println!("{:?}", channel.members);
                    }
                }
            }
            // if let Some(users) = response.members {
            //     println!("Got {} users", users.len());
            //     for user in users {
            //         println!("{:?}", user.real_name);
            //     }
            // }
        } else {
            println!("{:?}", response);
        }
    }

    pub fn _get_channels_list(&self) -> Vec<slack::Channel> {
        let mut channels_list: Vec<slack::Channel> = Vec::new();
        let response = slack::channels::list(&self.slack_client,
                                &self.slack_token,
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

    pub fn _get_users_list(&self) -> Vec<slack::User> {
        let mut users_list: Vec<slack::User> = Vec::new();
        let response = slack::users::list(&self.slack_client,
                                &self.slack_token,
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
