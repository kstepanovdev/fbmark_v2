use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

impl Tag {
    pub fn new(id: i64, name: String) -> Self {
        Self { id, name }
    }

    pub async fn fetch_all(repo: &impl Repo) -> Result<Vec<Self>> {
        repo.fetch_all().await
    }

    pub async fn create(repo: &impl Repo, name: String) -> Result<Self> {
        repo.create(name).await
    }

    pub async fn get(repo: &impl Repo, id: i64) -> Result<Self> {
        repo.get(id).await
    }

    pub async fn get_by_name(repo: &impl Repo, name: String) -> Result<Self> {
        repo.get_by_name(name).await
    }

    pub async fn delete(repo: &impl Repo, id: i64) -> Result<Self> {
        repo.delete(id).await
    }
}

#[async_trait]
pub trait Repo {
    async fn fetch_all(&self) -> Result<Vec<Tag>>;
    async fn create(&self, name: String) -> Result<Tag>;
    async fn get(&self, id: i64) -> Result<Tag>;
    async fn get_by_name(&self, name: String) -> Result<Tag>;
    async fn delete(&self, id: i64) -> Result<Tag>;
}
