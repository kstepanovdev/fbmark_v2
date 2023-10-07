use std::{env, path::Path};

use anyhow::Result;
use async_trait::async_trait;
use bookmarks::Repo as BookmarkRepo;
use sqlx::{migrate::MigrateDatabase, query, query_as, sqlite::SqlitePoolOptions, SqlitePool};
use tags::Repo as TagsRepo;

use crate::models::{
    bookmarks::{self, Bookmark, NewBookmarkParams, RawBookmark},
    tags::{self, Tag},
};

#[derive(Debug, Clone)]
pub struct Repo {
    pub pool: SqlitePool,
}

impl Repo {
    pub async fn new() -> Result<Self> {
        // Create the database
        let db_url = dotenvy::var("DATABASE_URL")?;

        if !sqlx::Sqlite::database_exists(&db_url).await? {
            sqlx::Sqlite::create_database(&db_url).await?;
        }

        // Connect to the database
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        // Migrate the database
        let migrations = if env::var("RUST_ENV") == Ok("production".to_string()) {
            // Productions migrations dir
            std::env::current_exe()?.join("./migrations")
        } else {
            // Development migrations dir
            let crate_dir = std::env::var("CARGO_MANIFEST_DIR")?;
            Path::new(&crate_dir).join("./migrations")
        };

        sqlx::migrate::Migrator::new(migrations)
            .await?
            .run(&pool)
            .await?;

        Ok(Repo { pool })
    }
}

#[async_trait]
impl BookmarkRepo for Repo {
    async fn fetch_all(&self, tags_filter: Vec<Tag>) -> Result<Vec<Bookmark>> {
        // TODO: this is dreadful code duplication, though seems like it cannot be solved w/o heavy refactoring
        // due to sqlx that behaves kinda strange in terms of Vec<Record> type conversions
        // !!! DO NOT USE SQLX IF YOU NEED "WHERE ... IN" request !!!

        let raw_bookmarks = query_as!(
            RawBookmark,
            r#"
                SELECT b.id, b.title, b.url, group_concat(t.id || ',' || t.name) tags
                FROM bookmarks b
                LEFT JOIN bmarks_tags bt ON bt.bookmark_id = b.id
                LEFT JOIN tags t ON t.id = bt.tag_id
                GROUP BY b.id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let raw_bookmarks = if tags_filter.is_empty() {
            raw_bookmarks
        } else {
            // filter bookmarks in order to left only that we're interested in
            raw_bookmarks
                .into_iter()
                .filter(|record| {
                    if let Some(tags) = &record.tags {
                        let tags: Vec<(i64, &str)> = tags
                            .split(',')
                            .map(str::trim)
                            .collect::<Vec<&str>>()
                            .windows(2)
                            .filter_map(|pair| {
                                if let [number_str, text] = pair {
                                    if let Ok(number) = number_str.parse::<i64>() {
                                        Some((number, *text))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect();

                        let bmark_tags_ids: Vec<i64> = tags.iter().map(|tag| tag.0).collect();

                        let required_tag_ids =
                            tags_filter.iter().map(|tag| tag.id).collect::<Vec<i64>>();

                        for tag_id in bmark_tags_ids {
                            if required_tag_ids.contains(&tag_id) {
                                return true;
                            }
                        }
                        false
                    } else {
                        false
                    }
                })
                .collect::<Vec<RawBookmark>>()
        };

        let bookmarks: Vec<Bookmark> = raw_bookmarks
            .into_iter()
            .map(|record| {
                let tags = record.tags.as_ref().map(|tags| {
                    tags.split(',')
                        .collect::<Vec<&str>>()
                        .chunks(2)
                        .map(|chunk| {
                            if chunk.len() > 1 {
                                let id = chunk[0].parse::<i64>();
                                let name = chunk[1].to_string();
                                match id {
                                    Ok(parsed_id) => Tag::new(parsed_id, name),
                                    Err(_) => Tag::default(),
                                }
                            } else {
                                Tag::default()
                            }
                        })
                        .collect::<Vec<Tag>>()
                });

                Bookmark::new(record.id, record.title, record.url.into(), tags)
            })
            .collect();

        Ok(bookmarks)
    }

    async fn batch_create(&self, bmarks_params: Vec<NewBookmarkParams>) -> Result<Vec<Bookmark>> {
        // TODO: implement batch transactions
        // TODO: try to refactor that
        let mut tx = self.pool.begin().await?;

        let mut bookmarks = vec![];
        for bmark_params in bmarks_params {
            let url: String = bmark_params.url.into();
            let raw_bookmark = query!(
                r#"
                INSERT INTO bookmarks
                (title, url)
                VALUES ($1, $2)
                RETURNING id, title, url
                "#,
                bmark_params.title,
                url,
            )
            .fetch_one(&mut *tx)
            .await?;

            let mut tags = vec![];
            // tags part
            for tag_name in bmark_params.tags {
                let tag = if let Ok(tag) = query_as!(
                    Tag,
                    r#"
                    SELECT id, name
                    FROM tags
                    WHERE name = $1
                    "#,
                    tag_name
                )
                .fetch_one(&mut *tx)
                .await
                {
                    tag
                } else {
                    query_as!(
                        Tag,
                        r#"
                        INSERT INTO tags
                        (name)
                        VALUES ($1)
                        RETURNING id, name
                        "#,
                        tag_name
                    )
                    .fetch_one(&mut *tx)
                    .await?
                };

                query!(
                    r#"
                    INSERT INTO bmarks_tags
                    (bookmark_id, tag_id)
                    VALUES ($1, $2)
                    "#,
                    raw_bookmark.id,
                    tag.id,
                )
                .execute(&mut *tx)
                .await?;

                tags.push(tag);
            }

            let bmark = Bookmark::new(
                raw_bookmark.id,
                raw_bookmark.title,
                raw_bookmark.url,
                Some(tags),
            );
            bookmarks.push(bmark);
        }
        tx.commit().await?;

        Ok(bookmarks)
    }

    async fn create(
        &self,
        title: Option<String>,
        url: url::Url,
        tags: Option<Vec<Tag>>,
    ) -> Result<Bookmark> {
        let mut tx = self.pool.begin().await?;
        let url = url.to_string();

        let raw_bookmark = query!(
            r#"
            INSERT INTO bookmarks
            (title, url)
            VALUES ($1, $2)
            RETURNING id, title, url
            "#,
            title,
            url,
        )
        .fetch_one(&mut *tx)
        .await?;

        if let Some(tags) = &tags {
            for tag in tags {
                query!(
                    r#"
                    INSERT INTO bmarks_tags
                    (bookmark_id, tag_id)
                    VALUES ($1, $2)
                    "#,
                    raw_bookmark.id,
                    tag.id,
                )
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;

        Ok(Bookmark::new(
            raw_bookmark.id,
            raw_bookmark.title,
            raw_bookmark.url,
            tags,
        ))
    }

    async fn delete(&self, id: i64) -> Result<()> {
        query!(
            r#"
            DELETE FROM bookmarks
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl TagsRepo for Repo {
    async fn fetch_all(&self) -> Result<Vec<Tag>> {
        let tags = query_as!(
            Tag,
            r#"
            SELECT id, name
            FROM tags
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tags)
    }

    async fn create(&self, name: String) -> Result<Tag> {
        let tag = query_as!(
            Tag,
            r#"
            INSERT INTO tags
            (name)
            VALUES ($1)
            RETURNING id, name
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }

    async fn get(&self, id: i64) -> Result<Tag> {
        let tags = query_as!(
            Tag,
            r#"
            SELECT id, name
            FROM tags
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(tags)
    }

    async fn get_by_name(&self, name: String) -> Result<Tag> {
        let tag = query_as!(
            Tag,
            r#"
            SELECT id, name
            FROM tags
            WHERE name = $1
            "#,
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }

    async fn delete(&self, id: i64) -> Result<Tag> {
        let tag = query_as!(
            Tag,
            r#"
            DELETE FROM tags
            WHERE id = $1
            RETURNING id, name
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }
}
