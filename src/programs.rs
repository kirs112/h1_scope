use std::fmt::{Display};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Handlers {
    #[serde(rename = "data")]
   pub data: Vec<Datum>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Datum {

    #[serde(rename = "attributes")]
    pub attributes: Attributes,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Attributes {
    #[serde(rename = "handle")]
    pub handle: String,

    #[serde(rename = "offers_bounties")]
    pub offers_bounties: bool,
}



pub async fn get_programs(client: Client, user:&str, key:&str) -> Vec<String> {
    let mut handle: Vec<String> = vec![];
    for i in 1..50 {
        let url = format!("https://api.hackerone.com/v1/hackers/programs?page%5Bnumber%5D={}&page[size]=100", i);
        let resp =  client.get(url).basic_auth(user,Some(key)).send().await.unwrap();
        let resp_handler  = resp.text().await.unwrap();

        if resp_handler.clone().eq("{\"data\":[],\"links\":{}}") {
            return handle;
        }

        let handlers: Handlers = serde_json::from_str(&resp_handler.clone()).expect("response is not valid JSON");

        for programs_name in handlers.data {
            if programs_name.attributes.offers_bounties {
                handle.push(programs_name.attributes.handle);
            }
            continue
        }
    }

    handle
}


