extern crate rand;
use rand::seq::SliceRandom;

pub mod slack_driver;

#[derive(Debug)]
pub struct HappyLunchMates<'hlm> {
    slack_client: slack_driver::SlackDriver,
    n_groups: u8,
    restaurant_names: Vec<&'hlm str>
}

// impl<'hlm> HappyLunchMates<'hlm> {
//     pub fn new (slack_channel: &'hlm slack_driver::SlackDriver, n_groups: &u8, restaurant_names: &Vec<&'hlm str>) -> HappyLunchMates<'hlm> {
//         let mut number_group: u8 = *n_groups;
//         if  (slack_channel.members_list.len() as f32 / number_group as f32) < 1.3 {
//             let new_n_group: f32 = slack_channel.members_list.len() as f32 / 1.3;
//             println!("WARNING: number of groups, {}, is too high. It is autoimatically set to {:?}", n_groups, new_n_group as u8);
//             number_group = new_n_group as u8;
//         }
//         HappyLunchMates {
//             slack_client: slack_channel,
//             n_groups: number_group,
//             restaurant_names: restaurant_names.to_vec()
//         }
//     }

//     pub fn assign_to_group(&mut self){
//         // assigmenet based on random numbers
//         let n_group_members: u8 = self.slack_client.members_list.len() as u8;
//         let mut group_numbers: Vec<u8> = vec![0; self.n_groups as usize];
//         let ratio = n_group_members as f32 / self.n_groups as f32;

//         let mut sum_members: u8 = 0;

//         for i in 0..self.n_groups {
//             if ratio > ratio as u8 as f32 {
//                 if i % 2 == 0 {
//                     group_numbers[i as usize] = ratio as u8 + 1;
//                 } else {
//                     group_numbers[i as usize] = ratio as u8
//                 }
//             } else {
//                 group_numbers[i as usize] = ratio as u8
//             }
            
//             if i == self.n_groups - 1 {
//                 group_numbers[i as usize] = n_group_members - sum_members;
//             }

//             sum_members += group_numbers[i as usize];
//         }

//         let mut channel_members: Vec<&str> = self.slack_client.members_list.clone();
//         let mut groups: Vec<_> = vec![];

//         for number in group_numbers.iter() {
//             let group = channel_members
//                 .choose_multiple(&mut rand::thread_rng(), *number as usize)
//                 .cloned()
//                 .collect::<Vec<&str>>();

//             channel_members.retain(|&x| !group.contains(&x));
//             groups.push(group);
//         }

//         println!("groups are as follows: {:?}", groups);
//     }
// }