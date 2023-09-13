use crate::client::constants::{LOCALHOST_URL, LOCALHOST_WS};
use client_api::Client;

mod client;
mod collab;
mod gotrue;
mod realtime;

pub fn client_api_client() -> Client {
  Client::from(reqwest::Client::new(), LOCALHOST_URL, LOCALHOST_WS)
}
