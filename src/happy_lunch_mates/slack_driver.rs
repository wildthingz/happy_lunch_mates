use std::vec::Vec;
extern crate slack_api as slack;

// use dotenv::dotenv;
// use slack::requests::{Error, Client};

#[derive(Debug)]
pub struct SlackDriver<'sd> {
    slack_client: slack::requests::Client,
    slack_token : String,
    channel_name : &'sd str,
    pub members_list : Vec<String>
}

impl<'sd> SlackDriver<'sd> {
    pub fn new(channel_name: &str) -> SlackDriver {
        let token = std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN not set in .env");
        let client: slack::requests::Client = slack::default_client().unwrap();

        let channels_list: Vec<slack::Channel> = SlackDriver::_get_channels_list(&client, &token);
        let users_list: Vec<slack::User> = SlackDriver::_get_users_list(&client, &token);
        let members_list: Vec<String> = SlackDriver::_get_members_from_channel(&channels_list, 
                                                                                &users_list, 
                                                                                channel_name);

        SlackDriver {
            slack_client: client,
            slack_token : token,
            channel_name : channel_name,
            members_list : members_list
        }
    }

    fn _get_members_from_channel(channels_list: &Vec<slack::Channel>, 
                                users_list: &Vec<slack::User>, 
                                channel_name: &str) -> Vec<String>{

        let mut user_id_list_in_channel: Vec<String> = Vec::new();

        let mut user_name_list_in_channel: Vec<String> = Vec::new();

        for channel in channels_list.to_vec() {
            if channel.name.unwrap() == channel_name {
                user_id_list_in_channel = channel.members.unwrap();
                break;
            }
        } 

        for user in users_list.to_vec() {
            if user_id_list_in_channel.contains(&user.id.unwrap()) {
                user_name_list_in_channel.push(user.name.unwrap())
            }
        }
        return user_name_list_in_channel;
    }

    fn _get_channels_list(client: &slack::requests::Client, token: &str) -> Vec<slack::Channel> {
        let mut channels_list: Vec<slack::Channel> = Vec::new();
        let response = slack::channels::list(client, token,
                                &slack::channels::ListRequest::default());

        if let Ok(response) = response {
            if let Some(channels) = response.channels {
                channels_list = channels;    
            }
        } else {
        println!("Response not OK: {:?}", response);
        }
        return channels_list;
    }

    fn _get_users_list(client: &slack::requests::Client, token: &str) -> Vec<slack::User> {
        let mut users_list: Vec<slack::User> = Vec::new();
        let response = slack::users::list(client, token,
                                &slack::users::ListRequest::default());

        if let Ok(response) = response {
            if let Some(users) = response.members {
                users_list = users;    
            }
        } else {
        println!("Response not OK: {:?}", response);
        }
        return users_list;
    }

    pub fn send_message(&self, message: &str) {
        let postmessage = slack::chat::PostMessageRequest{
                                    channel: &self.channel_name, 
                                    text   : message,
                                    parse: None,
                                    link_names: None,
                                    attachments: None,
                                    unfurl_links: None,
                                    unfurl_media: None,
                                    username: "poop".into(),
                                    as_user: false.into(),
                                    icon_url: None,
                                    icon_emoji: None,
                                    thread_ts: None,
                                    reply_broadcast: None};

        let response = slack::chat::post_message(&self.slack_client, &self.slack_token, &postmessage);

        if let Ok(response) = response {
            if let Some(message_sent) = response.message {
                println!("message sent was: {:?}", message_sent)   
            }
        } else {
        println!("Response not OK: {:?}", response);
        }
    }
}
