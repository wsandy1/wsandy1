use serde::{Deserialize, Serialize};
use std::io::Write;
use dotenv::dotenv;
use std::env;
use chrono::Local;


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

// for fetching projects
#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct ProjectResponse {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    name: String,
    username: String,
    url: String,
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

async fn fetch_projects() -> Vec<Project> {
    let client = reqwest::Client::new();
    let res = client.get(format!("{}projects?select=*", env::var("BASE_URL").unwrap()))
                    .header("apikey", env::var("API_KEY").unwrap())
                    .header("Authorization", format!("Bearer {}", env::var("API_KEY").unwrap()))
                    .send()
                    .await
                    .unwrap();
    
    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<ProjectResponse>().await {
                Ok(parsed) => return parsed.projects,
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

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    stargazers_count: u64,
    forks: u64,
}

async fn fetch_repo_details(name: &String, username: &String) -> Repo {
    let client = reqwest::Client::new();
    let res = client.get(format!("https://api.github.com/repos/{}/{}", username, name))
                    .header(reqwest::header::USER_AGENT, "Rust")
                    .send()
                    .await
                    .unwrap();
    
    match res.status() {
        reqwest::StatusCode::OK => {
            match res.json::<Repo>().await {
                Ok(parsed) => return parsed,
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

    let mut file = std::fs::File::create("README.md").unwrap();

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
    write!(file, "\n### Current Projects\n").unwrap();

    let projects = fetch_projects().await;
    write!(file, "|📖 Projects|⭐ Stars|🍴 Forks|\n|-|-|-|\n").unwrap();
    if projects.is_empty() {
        write!(file, "|*It's deserted here… 😔*|🏜️|🌃|").unwrap();
    } else {
        for project in projects.iter() {
            let repo_details = fetch_repo_details(&project.name, &project.username).await;
            write!(file, "|**[{}]({})**|{}|{}|", project.name, project.url, repo_details.stargazers_count, repo_details.forks).unwrap();
        }
    }
    write!(file, "\n### Stats\n![Will's GitHub stats](https://github-readme-stats.vercel.app/api?username=anuraghazra&theme=github_dark&show_icons=true)\n<figure><embed src=\"https://wakatime.com/share/@a345175e-628e-4692-8abd-b349e72e294a/7ac23843-3870-4034-9d1e-d59527adf038.svg\"></embed></figure>\n").unwrap();
    write!(file, "-----\nThis *README* file is regenerated **every 6 hours**.\n*Should I have written it in Rust? No*\n\nLast Update: {}\n\n![Source Code](https://github.com/wsandy1/wsandy1)", Local::now().format("%A, %e %B, %H:%M GMT%z")).unwrap()
}