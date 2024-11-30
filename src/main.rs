use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Define owner and repo
    let owner = "rust-lang-nursery";
    let repo = "rust-cookbook";

    // Construct the API URL dynamically
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = owner,
        repo = repo
    );

    println!("Requesting data from: {}", request_url);

    // Create the client
    let client = reqwest::Client::new();

    // Send the GET request
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "basic rust api client") // Ensure your user-agent is valid
        .send()
        .await?;

    // Ensure the response is OK (status code 2xx)
    if !response.status().is_success() {
        println!("Error: Received an unsuccessful response: {}", response.status());
        return Ok(());
    }

    // Deserialize the response into a Vec of Stargazer structs
    let stargazers: Vec<Stargazer> = response.json().await?;

    // Print the first 20 stargazers
    for stargazer in stargazers.iter().take(20) {
        println!("ID: {}\nLogin: {}\n", stargazer.id, stargazer.login);
    }

    Ok(())
}

// Define a struct to match the GitHub API response
#[derive(Deserialize, Debug)]
struct Stargazer {
    id: u32,
    login: String,
}
