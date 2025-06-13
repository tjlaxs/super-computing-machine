use github_api::new_client;

#[tokio::main]
async fn main() {
    let client = new_client().expect("no client");
    let res = client.user_repos("tjlaxs").await.expect("failing");

    println!("Response:\n{:?}", res);
}
