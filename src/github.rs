use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeObject {
    path: String,
    mode: String,
    #[serde(rename = "type")]
    type_: String,
    size: Option<usize>,
    sha: String,
    url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeResponse {
    sha: String,
    url: String,
    tree: Vec<TreeObject>,
}

impl TreeResponse {
    pub fn files(&self) -> Vec<String> {
        self.tree
            .iter()
            .filter(|x| x.type_ == "blob")
            .map(|x| x.path.clone())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubFile {
    #[serde(rename = "type")]
    type_: String,
    encoding: String,
    size: usize,
    name: String,
    path: String,
    content: String,
    sha: String,
    url: String,
    git_url: String,
    html_url: String,
    download_url: String,
    _links: HashMap<String, String>,
}

pub struct GithubClient {
    client: Client,
    api_url: String,
    repo: String,
    has_token: bool,
}

impl GithubClient {
    pub fn new(repo: String, api_token: Option<String>) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "hid-io".parse().unwrap());
        headers.insert(ACCEPT, "application/vnd.github.v3+json".parse().unwrap());
        if let Some(api_token) = &api_token {
            headers.insert(
                AUTHORIZATION,
                format!("token {}", api_token).parse().unwrap(),
            );
        }

        let client = reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        GithubClient {
            client,
            repo,
            api_url: "https://api.github.com".to_string(),
            has_token: api_token.is_some(),
        }
    }

    // If this fails there is no use in trying to parse the response
    fn _check_response(&self, response: &Response) {
        let headers = response.headers();
        if headers.get("X-RateLimit-Remaining").unwrap() == "0" {
            println!(
                "Rate limit resets at unix time: {}",
                headers.get("X-RateLimit-Reset").unwrap().to_str().unwrap()
            );
            if !self.has_token {
                println!("NOTE: Setting GITHUB_API_TOKEN will help avoid this");
            }
            panic!("RATE LIMIT EXCEEDED");
        }
    }

    pub fn get_file_info(
        &self,
        path: &str,
        reftag: &str,
    ) -> Result<GithubFile, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get(&format!(
                "{}/repos/{}/contents/{}",
                self.api_url, self.repo, path
            ))
            .query(&[("ref", reftag)])
            .send()?;
        self._check_response(&resp);
        let file = resp.json::<GithubFile>()?;
        Ok(file)
    }

    pub fn get_file_raw(
        &self,
        path: &str,
        reftag: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get(&format!(
                "{}/repos/{}/contents/{}",
                self.api_url, self.repo, path
            ))
            .header(ACCEPT, "application/vnd.github.VERSION.raw+json")
            .query(&[("ref", reftag)])
            .send()?;
        self._check_response(&resp);
        Ok(resp.text()?)
    }

    pub fn list_files(&self, reftag: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let resp = self
            .client
            .get(&format!(
                "{}/repos/{}/git/trees/{}",
                self.api_url, self.repo, reftag
            ))
            .query(&[("recursive", "1")])
            .send()?;
        self._check_response(&resp);
        let tree = resp.json::<TreeResponse>()?;
        Ok(tree.files())
    }
}
