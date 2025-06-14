use github_api::new_client;
use std::{thread, time};

#[tokio::main]
async fn main() {
    let client = new_client().expect("no client");
    let repos = client.user_repos("tjlaxs").await.expect("failing");
    for r in repos.repos {
        thread::sleep(time::Duration::from_millis(10));
        println!("{}", r.name);
    }
}
