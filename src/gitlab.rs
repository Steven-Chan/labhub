use crate::api::models::{gitlab, github};
use crate::api::{gitlab_client, github_client};
use crate::config;
use crate::errors::{GitError, RequestErrorResult};
use crate::github as github_helper;

use log::{info};
use regex::Regex;

fn has_gitlab_repo(gitlab_repo_full_name: &str) -> bool {
    let lab_to_hub_lock = config::LAB_TO_HUB.lock().unwrap();
    let lab_to_hub = &*lab_to_hub_lock;
    lab_to_hub.contains_key(gitlab_repo_full_name)
}

fn get_github_repo_name(gitlab_repo_full_name: &str) -> String {
    let lab_to_hub_lock = config::LAB_TO_HUB.lock().unwrap();
    let lab_to_hub = &*lab_to_hub_lock;
    lab_to_hub.get(gitlab_repo_full_name).unwrap().to_string()
}

fn handle_pipeline(pipeline_event: gitlab::PipelineEvent) -> Result<(), RequestErrorResult> {
    let pipeline = pipeline_event.object_attributes.as_ref()?;
    let branch = pipeline.ref_key.as_ref()?;
    let branch_name_re = Regex::new(&format!(r#"^{}(.+)/pr-(\d+)$"#, &config::CONFIG.pr_branch_prefix)).unwrap();
    match branch_name_re.captures(branch) {
        Some(ref caps) if caps.len() == (3 as usize) => {
            info!("Handling branch {}", branch);
            let project = caps.get(1).unwrap().as_str();
            let pr_number = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let client = reqwest::Client::new();
            let github_repo = get_github_repo_name(project);
            let pipeline_number = pipeline.id.as_ref()?;
            let pipeline_url = format!("{}/pipelines/{}", gitlab_client::make_ext_url(&project), pipeline_number);
            let pipeline_status = pipeline.status.as_ref()?;
            let _result = write_pipeline_status_to_pr_comment(&client, &github_repo, pr_number, *pipeline_number, &pipeline_url, &pipeline_status);
            info!("PR number {} from repo {}", pr_number, github_repo)
        }
        _ => info!("Non PR triggered pipeline")
    }
    Ok(())
}

fn write_pipeline_status_to_pr_comment(
    client: &reqwest::Client,
    github_repo: &str,
    pr_number: i64,
    pipeline_number: i64,
    pipeline_url: &str,
    pipeline_status: &str,
) -> Result<(), GitError> {
    let repo_full_name_parts: Vec<String> = github_repo
        .split('/')
        .map(std::string::ToString::to_string)
        .collect();
    if repo_full_name_parts.len() != 2 {
        return Err(GitError {
            message: format!("Invalid repo name {}", github_repo),
        });
    }
    let org = &repo_full_name_parts[0];
    let repo = &repo_full_name_parts[1];

    let status: Option<String> = match pipeline_status {
        "running" => Some(format!("⏳ running")),
        "success" => Some(format!("✅ success")),
        "failed" => Some(format!("❌ failed")),
        "canceled" => Some(format!("🚫 canceled")),
        _ => None
    };

    match status {
        Some(status_str) => {
            let body = format!("Pipeline [#{}]({}) : {}", pipeline_number, pipeline_url, &status_str);
            let last_comment = find_last_comment_by_labhub(client, org, repo, pr_number)?;
            info!("{:?}", last_comment);
            match last_comment {
                Some(last_comment) => {
                    github_client::update_issue_comment(
                        &client,
                        org,
                        repo,
                        last_comment.id.unwrap(),
                        &format!("{}\n{}", last_comment.body.as_ref()?, body),
                    )?
                },
                None => {
                    github_helper::write_comment(
                        &client,
                        &String::from(github_repo),
                        pr_number,
                        &format!("Meow!\n{}", body),
                    )?
                }
            }
        },
        None => {
            info!("Unhandled pipeline status: {}", pipeline_status)
        }
    }
    Ok(())
}

fn find_last_comment_by_labhub(
    client: &reqwest::Client,
    org: &str,
    repo: &str,
    pr_number: i64,
) -> Result<Option<github::IssueCommentComment>, GitError> {
    let comments = github_client::list_issue_comments(
        &client,
        &org,
        &repo,
        pr_number
    )?;
    let last_comment = comments
        .into_iter()
        .filter(|comment| Some(config::CONFIG.github.username.clone()) == comment.user.as_ref().and_then(|u| u.login.clone()))
        .last();
    Ok(last_comment)
}

pub fn handle_event_body(event_type: &str, body: &str) -> Result<String, RequestErrorResult> {
    match event_type {
        "Pipeline Hook" => {
            if config::feature_enabled(&config::Feature::Pipeline) {
                let pipeline: gitlab::PipelineEvent = serde_json::from_str(body)?;
                if has_gitlab_repo(pipeline.project.as_ref()?.path_with_namespace.as_ref()?) {
                    info!("Pipeline ref={}", pipeline.object_attributes.as_ref()?.ref_key.as_ref()?);
                    handle_pipeline(pipeline)?;
                } else {
                    info!("Repo not listed.");
                }
            } else {
                info!("Push feature not enabled. Skipping event.");
            }
            Ok(String::from("Pipeline received"))
        }
        _ => Ok(format!(
            "Unhandled event_type={}, doing nothing 😀",
            event_type,
        )),
    }
}
