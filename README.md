# Happy Lunch Mates

_Slack App for organizing lunch groups_

## Introduction

The lunch crew at Sanctuary has gotten large enough to warrant lunch groups.
This is the repo for a Slack App that makes lunch groups and rotates members periodically, also allowing group members to vote on where to eat.

## Design specification

- Reads channel members
- Reads name of the restaurants
- Reads number of desired groups
- Assigns memebers to groups
- Suggests a restaurant for each group
- Waits for response, if 2/3 of the responded members agree with the choice proceed
- Non-responders and abstainers are assumed to be absent from lunch.
- If more than 1/3 of the responders disagree with the choice a voting session is going to be held. 
- The chosen restaurant will be the vote of the simple majority (51%)
- A firebase system needs to be set up to keep track of the restaurants and groups
- The members are assigned to groups such that each day 36% of the group members are new.

## Requirements

- [`rust`](https://www.rust-lang.org/)

## Installation and setup

You'll need an API Auth token to make requests via Slack-API.
Get the token from a present contributor and add the token to a `.env` file.

Alternatively, use command `$ export AUTH_TOKEN=<your-token-here>`.

To compile, run `$ cargo build`

## Running

Run `$ cargo run`

## Resources

- slack rust api: https://github.com/slack-rs/slack-rs-api
- firebase rust : https://github.com/davidrhyswhite/rust-firebase

## Future Work

- Use supervised machine learning based on the gathered data to predict each group prefered restaurant.
