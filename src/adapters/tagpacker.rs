use anyhow::Result;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer};
use url::Url;

use crate::{models::bookmarks::NewBookmarkParams, settings::Settings};

#[derive(Deserialize, Debug)]
pub struct Link {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "sourceUrl")]
    pub source_url: Url,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "thumbnailSource")]
    pub thumbnail_source: Option<String>,
    #[serde(rename = "thumbnailSourceUrl")]
    pub thumbnail_source_url: Option<String>,
    #[serde(rename = "thumbnailId")]
    pub thumbnail_id: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    pub url: String,
    #[serde(rename = "createdAt", deserialize_with = "deserialize_created_at")]
    pub created_at: DateTime<Utc>,
    pub tags: Vec<Tag>,
}

fn deserialize_created_at<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str = String::deserialize(deserializer)?;

    let date =
        NaiveDate::parse_from_str(&date_str, "%b %d, %y").map_err(serde::de::Error::custom)?;

    let naive_datetime = &date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| serde::de::Error::custom("Wasn't able to convert to NaiveDateTime"))?;

    let datetime = Utc
        .from_local_datetime(naive_datetime)
        .earliest()
        .ok_or_else(|| serde::de::Error::custom("Invalid date and time"))?;

    Ok(datetime)
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub url: String,
    pub pack: Option<Pack>,
}

#[derive(Deserialize, Debug)]
pub struct Pack {
    pub id: String,
    pub name: String,
    pub color: u16,
}

pub async fn get_links() -> Result<Vec<NewBookmarkParams>> {
    let settings = Settings::get_configuration()?;

    let links = reqwest::get(format!(
        "https://tagpacker.com/api/users/{}/links",
        settings.tagpacker.user_id
    ))
    .await?
    .json::<Vec<Link>>()
    .await?;

    Ok(links
        .into_iter()
        .map(|link| {
            NewBookmarkParams::new(
                Some(link.title),
                link.source_url,
                link.tags.into_iter().map(|tag| tag.name).collect(),
            )
        })
        .collect::<Vec<NewBookmarkParams>>())
}
