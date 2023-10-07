use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use url::Url;

use super::tags::Tag;

// That struct only need to be able to operate with the bookmarks and thier tags from the database
// since query_as cannot be used due to sqlite limitations (no ARRAY_AGG function)
pub struct RawBookmark {
    pub id: i64,
    pub title: Option<String>,
    pub url: BookmarkUrl,
    pub tags: Option<String>,
}

#[derive(Debug)]
pub struct NewBookmarkParams {
    pub title: Option<String>,
    // TODO: change to just Url if possible
    pub url: Url,
    pub tags: Vec<String>,
}

impl NewBookmarkParams {
    pub fn new(title: Option<String>, url: Url, tags: Vec<String>) -> Self {
        Self { title, url, tags }
    }
}

// TODO: change to private if possible
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: i64,
    pub title: Option<String>,
    // TODO: change to Url if possible
    pub url: BookmarkUrl,
    pub tags: Option<Vec<Tag>>,
}

// TODO: get rid of this struct completely
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookmarkUrl {
    pub inner: Url,
}

impl From<BookmarkUrl> for String {
    fn from(value: BookmarkUrl) -> Self {
        value.inner.to_string()
    }
}

impl From<String> for BookmarkUrl {
    fn from(value: String) -> Self {
        Self {
            inner: Url::parse(&value).expect("Wasn't able to convert String into URL"),
        }
    }
}

impl Bookmark {
    pub fn new(id: i64, title: Option<String>, url: String, tags: Option<Vec<Tag>>) -> Self {
        Self {
            id,
            title,
            url: url.into(),
            tags,
        }
    }

    pub async fn fetch_all(repo: &impl Repo, tags_filter: Vec<Tag>) -> Result<Vec<Self>> {
        repo.fetch_all(tags_filter).await
    }

    pub async fn create(
        repo: &impl Repo,
        title: Option<String>,
        url: Url,
        tags: Option<Vec<Tag>>,
    ) -> Result<Self> {
        repo.create(title, url, tags).await
    }

    pub async fn batch_create(
        repo: &impl Repo,
        params: Vec<NewBookmarkParams>,
    ) -> Result<Vec<Self>> {
        repo.batch_create(params).await
    }

    pub async fn delete(repo: &impl Repo, id: i64) -> Result<()> {
        repo.delete(id).await
    }
}

#[async_trait]
pub trait Repo {
    async fn fetch_all(&self, tags_filter: Vec<Tag>) -> Result<Vec<Bookmark>>;
    async fn create(
        &self,
        title: Option<String>,
        url: Url,
        tags: Option<Vec<Tag>>,
    ) -> Result<Bookmark>;
    async fn batch_create(&self, params: Vec<NewBookmarkParams>) -> Result<Vec<Bookmark>>;
    async fn delete(&self, id: i64) -> Result<()>;
}
