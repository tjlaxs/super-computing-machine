use github_api::new_client;

#[tokio::main]
async fn main() {
    let client = new_client().expect("no client");
    let res = client.repo_languages().await.expect("failing");

    println!("Response:\n{:?}", res);
}
