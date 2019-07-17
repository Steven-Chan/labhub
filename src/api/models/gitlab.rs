// This file is auto-generated, do not edit.

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
    pub id: Option<i64>,
    pub status: Option<String>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub sha: Option<String>,
    pub web_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobEvent {
    pub object_kind: Option<String>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub tag: Option<bool>,
    pub before_sha: Option<String>,
    pub sha: Option<String>,
    pub job_id: Option<i64>,
    pub job_name: Option<String>,
    pub job_stage: Option<String>,
    pub job_status: Option<String>,
    pub job_started_at: Option<String>,
    pub job_finished_at: Option<String>,
    pub job_duration: Option<String>,
    pub job_allow_failure: Option<bool>,
    pub job_failure_reason: Option<String>,
    pub project_id: Option<i64>,
    pub project_name: Option<String>,
    pub user: Option<JobEventUser>,
    pub commit: Option<JobEventCommit>,
    pub repository: Option<JobEventRepository>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobEventUser {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobEventCommit {
    pub id: Option<i64>,
    pub sha: Option<String>,
    pub message: Option<String>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub status: Option<String>,
    pub duration: Option<String>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JobEventRepository {
    pub name: Option<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub git_ssh_url: Option<String>,
    pub git_http_url: Option<String>,
    pub visibility_level: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEvent {
    pub object_kind: Option<String>,
    pub object_attributes: Option<PipelineEventObjectAttributes>,
    pub user: Option<PipelineEventUser>,
    pub project: Option<PipelineEventProject>,
    pub commit: Option<PipelineEventCommit>,
    pub builds: Option<Vec<PipelineEventBuildsItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventObjectAttributes {
    pub id: Option<i64>,
    #[serde(rename = "ref")]
    pub ref_key: Option<String>,
    pub tag: Option<bool>,
    pub sha: Option<String>,
    pub before_sha: Option<String>,
    pub status: Option<String>,
    pub stages: Option<Vec<String>>,
    pub created_at: Option<serde_json::value::Value>,
    pub finished_at: Option<String>,
    pub duration: Option<i64>,
    pub variables: Option<Vec<PipelineEventObjectAttributeVariablesItem>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventObjectAttributeVariablesItem {
    pub key: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventProject {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub web_url: Option<String>,
    pub avatar_url: Option<String>,
    pub git_ssh_url: Option<String>,
    pub git_http_url: Option<String>,
    pub namespace: Option<String>,
    pub visibility_level: Option<i64>,
    pub path_with_namespace: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventCommit {
    pub id: Option<String>,
    pub message: Option<String>,
    pub timestamp: Option<String>,
    pub url: Option<String>,
    pub author: Option<PipelineEventCommitAuthor>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventCommitAuthor {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventBuildsItem {
    pub id: Option<i64>,
    pub stage: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<serde_json::value::Value>,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub when: Option<String>,
    pub manual: Option<bool>,
    pub user: Option<PipelineEventBuildsItemUser>,
    pub runner: Option<PipelineEventBuildsItemRunner>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventBuildsItemUser {
    pub name: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PipelineEventBuildsItemRunner {
    pub id: Option<i64>,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub is_shared: Option<bool>,
}
