use clap::Parser;
use reqwest;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::Command;
const URL_DEFAULT: &str = "https://api.github.com/repos/";

#[derive(Debug, Serialize, Deserialize)]
struct Owner {
    login: String,
    avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Organization {
    login: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoInfo {
    name: String,
    description: String,
    stargazers_count: i32,
    created_at: String,
    html_url: String,
    open_issues_count: i32,
    has_pages: bool,
    owner: Owner,
    forks_count: i32,
    language: String,

    #[serde(default = "default_or_none")]
    organization: Organization,
}

fn default_or_none() -> Organization {
    Organization {
        login: "NONE".to_string(),
    }
}

#[derive(Parser, Debug)]

struct Args {
    #[arg(short, long)]
    user_name: String,
    #[arg(short, long)]
    repo_name: String,
    #[arg(short, long)]
    output: String,
}

async fn get_repo_info(user: &str, repo: &str) -> RepoInfo {
    let repo_url = format!("{URL_DEFAULT}{user}/{repo}");
    let client = reqwest::Client::new();
    let response: RepoInfo = client
        .get(repo_url)
        .header("User-Agent", user)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    response
}

async fn downdload_user_avatar(url: &str) -> &str {
    let file_path = "./image/avatar.png";
    let mut file = std::fs::File::create(file_path).unwrap();
    let response = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    file.write_all(&response)
        .expect("ERROR: Could not download user avatar");
    file_path
}

fn create_frame_1(rounded_path: &str, user_name: &str, repo_desc: &str) {
    let bash_script_path = "bash/frame1.sh";
    let output_path = "./image/frame1.png";
    Command::new("bash")
        .arg(bash_script_path)
        .arg("kobold.jpg")
        .arg(rounded_path)
        .arg(user_name)
        .arg(format!("{}", repo_desc))
        .arg(output_path)
        .output()
        .expect("Could not run frame1.sh script");
}

fn create_frame_2(
    repo_name: &str,
    repo_desc: &str,
    stars_count: i32,
    issues_count: i32,
    has_page: bool,
) {
    let bash_script_path = "bash/frame2.sh";
    let output_path = "./image/frame2.png";
    let confirm_or_cancel = if has_page {
        "./image/confirm.png"
    } else {
        "./image/cancel.png"
    };
    // above 1000 the text will overlap the star
    let max_number: i32 = 1000;
    Command::new("bash")
        .arg(bash_script_path)
        .arg(repo_name)
        .arg(repo_desc)
        .arg(if stars_count < max_number {
            format!("{stars_count}")
        } else {
            "1k+".to_string()
        })
        .arg(if issues_count < max_number {
            format!("{issues_count}")
        } else {
            "1k+".to_string()
        })
        .arg(output_path)
        .arg(confirm_or_cancel)
        .output()
        .expect("Could not run frame2.sh script");
}

fn create_frame_3(
    repo_name: &str,
    repo_desc: &str,
    language: &str,
    created_at: &str,
    organization: &str,
) {
    let bash_script_path = "bash/frame3.sh";
    let output_path = "./image/frame3.png";

    Command::new("bash")
        .arg(bash_script_path)
        .arg(repo_name)
        .arg(repo_desc)
        .arg(language)
        .arg(created_at)
        .arg(organization)
        .arg(output_path)
        .output()
        .expect("Could not run frame3.sh script");
}

fn create_avatar_rounded(file_path: &str) -> &str {
    let avatar_rounded_path = "./image/avatar_rounded.png";
    let bash_script_path = "bash/rounded.sh";
    Command::new("bash")
        .arg(bash_script_path)
        .arg(file_path)
        .arg(avatar_rounded_path)
        .output()
        .expect("Could not run rounded.sh script");
    avatar_rounded_path
}

fn create_video(output: &str) {
    let bash_script_path = "bash/video.sh";
    Command::new("bash")
            .arg(bash_script_path)
            .arg(output)
            .output()
            .expect("Could not run video.sh script");
}


#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.user_name != "" && args.repo_name != "" && args.output != "" {
        let user_data: RepoInfo = get_repo_info(&args.user_name, &args.repo_name).await;
        let avatar_path = downdload_user_avatar(&user_data.owner.avatar_url).await;
        let rounded_path = create_avatar_rounded(avatar_path);
        let repo_name = format!("{}", user_data.name);
        create_frame_1(
            &rounded_path,
            &user_data.owner.login,
            &user_data.description,
        );
        create_frame_2(
            &repo_name,
            &user_data.description,
            user_data.stargazers_count,
            user_data.open_issues_count,
            user_data.has_pages,
        );

        create_frame_3(
            &repo_name,
            &user_data.description,
            &user_data.language,
            &user_data.created_at,
            &user_data.organization.login,
        );
       
        create_video(&args.output);

    } else {
        println!("Please provide the correct arguments");
    }
}
