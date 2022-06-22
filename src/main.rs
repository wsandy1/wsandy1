use serde::{Deserialize, Serialize};
use std::io::Write;
use dotenv::dotenv;
use std::env;


// for fetching technologies
#[derive(Serialize, Deserialize, Debug)]
struct Technology {
    name: String,
    badge: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct TechResponse {
    technologies: Vec<Technology>,
}

// for fetching things im learning
#[derive(Serialize, Deserialize, Debug)]
struct Learn {
    name: String,
    badge: String,
    reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct LearnResponse {
    items: Vec<Learn>,
}

async fn fetch_tech() -> Vec<Technology> {
    let client = reqwest::Client::new();
    let res = client.get(format!("{}technologies?select=*", env::var("BASE_URL").unwrap()))
                    .header("apikey", env::var("API_KEY").unwrap())
                    .header("Authorization", format!("Bearer {}", env::var("API_KEY").unwrap()))
                    .send()
                    .await
                    .unwrap();
    
    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<TechResponse>().await {
                Ok(parsed) => return parsed.technologies,
                Err(e) => panic!("Error deserializing response: {:?}", e),
            };
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized");
        },
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}

async fn fetch_learn() -> Vec<Learn> {
    let client = reqwest::Client::new();
    let res = client.get(format!("{}learn?select=*", env::var("BASE_URL").unwrap()))
                    .header("apikey", env::var("API_KEY").unwrap())
                    .header("Authorization", format!("Bearer {}", env::var("API_KEY").unwrap()))
                    .send()
                    .await
                    .unwrap();
    
    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<LearnResponse>().await {
                Ok(parsed) => return parsed.items,
                Err(e) => panic!("Error deserializing response: {:?}", e),
            };
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Unauthorized");
        },
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    };
}

#[tokio::main]
async fn main() {
    // for local testing
    dotenv().ok();

    let mut file = std::fs::File::create("readme.md").unwrap();

    let greeting = "Hey there!👋";
    let intro = "I'm **Will**, a web developer and code tinkerer living in **🇬🇧 London, UK**. I've been coding since age 8, and since then I've learnt a lot but achieved remarkably little with it!";

    write!(file, "# {}\n{}\n### Technologies I use\n", greeting, intro).unwrap();

    let technologies = fetch_tech().await;
    for tech in technologies.iter() {
        write!(file, "![{}]({}) ", tech.name, tech.badge).unwrap();
    }
    write!(file, "\n### Things I'm learning\n").unwrap();

    let learn = fetch_learn().await;
    for item in learn.iter() {
        write!(file, "![{}]({}) ", item.name, item.badge).unwrap();
    }

}