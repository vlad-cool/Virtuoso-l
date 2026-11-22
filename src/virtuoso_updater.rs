const UPDATE_URL: &'static str =
    "https://api.github.com/repos/vlad-cool/Virtuoso-l/releases/latest";

#[derive(Clone, Debug, serde::Deserialize)]
struct Asset {
    url: String,
    name: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Releases {
    name: String,
    tag_name: String,
    assets: Vec<Asset>,
}

pub enum UpdateStatus {
    Idle,
    Fetching,
    Waiting,
    Updating,
    Error(String),
}

pub struct VirtuosoUpdater {
    status: UpdateStatus,
}

impl VirtuosoUpdater {
    pub fn new() -> Self {
        Self {
            status: UpdateStatus::Idle,
        }
    }

    pub fn fetch_update(&mut self) {}
}
