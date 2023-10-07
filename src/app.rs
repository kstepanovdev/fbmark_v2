use std::fmt::Debug;

use crate::{
    adapters::{
        sqlite::Repo,
        tagpacker::{self},
    },
    models::{bookmarks::Bookmark, tags::Tag},
};
use anyhow::Result;
use crossterm::event::{Event, KeyEvent};
use ratatui::widgets::ListState;
use simsearch::{SearchOptions, SimSearch};
use tui_input::{backend::crossterm::EventHandler, Input};
use url::Url;

pub struct App {
    pub bookmarks: Vec<Bookmark>,
    pub bookmarks_items: StatefulList<Bookmark>,
    pub tags: Vec<Tag>,
    pub tags_items: StatefulList<Tag>,
    pub mode: Mode,
    pub tags_filter: Vec<Tag>,
    pub render_help: bool,
    should_quit: bool,
    repo: Repo,
}

impl App {
    pub fn is_should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub async fn new() -> Result<Self> {
        let repo = Repo::new().await?;
        let (bookmarks, bookmarks_items, tags, tags_items) =
            Self::refresh_state(&repo, vec![]).await?;

        Ok(Self {
            bookmarks,
            bookmarks_items,
            tags,
            tags_items,
            tags_filter: Vec::new(),
            mode: Mode::default(),
            should_quit: false,
            render_help: false,
            repo,
        })
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match &mut self.mode {
            Mode::Search(_params) => Mode::Scrolling,
            Mode::Create(ref mut params) => Mode::Create(params.clone()),
            Mode::Scrolling => Mode::Search(SearchingParams::default()),
        }
    }

    pub fn toggle_creation_mode(&mut self) {
        if let Mode::Create(_params) = &self.mode {
            self.mode = Mode::Scrolling;
        } else {
            let params = CreationParams {
                tags_items: StatefulList::with_items(self.tags.clone()),
                ..Default::default()
            };
            self.mode = Mode::Create(params);
        }
    }

    pub async fn reset(&mut self) -> Result<()> {
        let (_, bookmarks_items, _, tags_items, ..) =
            Self::refresh_state(&self.repo, Vec::new()).await?;
        self.bookmarks_items = bookmarks_items;
        self.tags_items = tags_items;
        self.tags_filter = Vec::new();

        Ok(())
    }

    pub fn change_active_window(&mut self) {
        // TODO: implement state machine here?
        match self.mode {
            Mode::Search(ref mut params) => match params.active_window {
                ActiveWindow::Link => params.active_window = ActiveWindow::Title,
                ActiveWindow::Title => params.active_window = ActiveWindow::Tags,
                ActiveWindow::Tags => params.active_window = ActiveWindow::Link,
            },
            Mode::Create(ref mut params) => match params.active_window {
                ActiveWindow::Link => params.active_window = ActiveWindow::Title,
                ActiveWindow::Title => params.active_window = ActiveWindow::Tags,
                ActiveWindow::Tags => params.active_window = ActiveWindow::Link,
            },
            Mode::Scrolling => {}
        }
    }

    pub async fn sync_bmarks(&mut self) -> Result<()> {
        let tagpacker_links = tagpacker::get_links().await?;
        self.bookmarks = Bookmark::batch_create(&self.repo, tagpacker_links).await?;
        self.bookmarks_items = StatefulList::with_items(self.bookmarks.clone());
        self.tags = Tag::fetch_all(&self.repo).await?;
        self.tags_items = StatefulList::with_items(self.tags.clone());

        Ok(())
    }

    pub async fn resolve_enter(&mut self) -> Result<()> {
        match &mut self.mode {
            Mode::Search(_) => {
                if let Some(tag_index) = self.tags_items.state.selected() {
                    // TODO: add deselection for a tag if it was already selected
                    let selected_tag = self.tags_items.items[tag_index].clone();
                    self.tags_filter.push(selected_tag);

                    let (bookmarks, items, ..) =
                        Self::refresh_state(&self.repo, self.tags_filter.clone()).await?;
                    self.bookmarks = bookmarks;
                    self.bookmarks_items = items;
                }

                Ok(())
            }
            Mode::Create(params) => {
                match params.active_window {
                    ActiveWindow::Link | ActiveWindow::Title => {
                        let title = Some(params.title.to_string());
                        let url = Url::parse(&params.link.to_string())?;
                        // TODO: add tags
                        let _bookmark = Bookmark::create(
                            &self.repo,
                            title,
                            url,
                            Some(params.selected_tags.clone()),
                        )
                        .await?;
                        let (bookmarks, bookmarks_items, tags, tags_items) =
                            Self::refresh_state(&self.repo, self.tags_filter.clone()).await?;

                        self.bookmarks = bookmarks;
                        self.bookmarks_items = bookmarks_items;
                        self.tags = tags;
                        self.tags_items = tags_items;
                        self.mode = Mode::Scrolling;
                        Ok(())
                    }
                    ActiveWindow::Tags => {
                        // TODO: add deselection for a tag if it was already selected
                        if let Some(tag_index) = params.tags_items.state.selected() {
                            let selected_tag = params.tags_items.items[tag_index].clone();
                            params.selected_tags.push(selected_tag);
                        }
                        Ok(())
                    }
                }
            }
            Mode::Scrolling => {
                if let Some(url_index) = self.bookmarks_items.state.selected() {
                    let url: String = self.bookmarks_items.items[url_index].url.clone().into();
                    open::that(url)?;
                }
                Ok(())
            }
        }
    }

    pub async fn on_delete(&mut self) -> Result<()> {
        match &self.mode {
            Mode::Search(_) | Mode::Create(_) => Ok(()),
            Mode::Scrolling => {
                if let Some(index) = self.bookmarks_items.state.selected() {
                    let id = self.bookmarks_items.items[index].id;
                    Bookmark::delete(&self.repo, id).await?;

                    let (bookmarks, items, ..) =
                        Self::refresh_state(&self.repo, self.tags_filter.clone()).await?;
                    self.bookmarks = bookmarks;
                    self.bookmarks_items = items;
                }
                Ok(())
            }
        }
    }

    pub async fn refresh_state(
        repo: &Repo,
        filter_tags: Vec<Tag>,
    ) -> Result<(
        Vec<Bookmark>,
        StatefulList<Bookmark>,
        Vec<Tag>,
        StatefulList<Tag>,
    )> {
        let bookmarks = Bookmark::fetch_all(repo, filter_tags).await?;
        let bookmark_items = StatefulList::with_items(bookmarks.clone());
        let tags = Tag::fetch_all(repo).await?;
        let tags_items = StatefulList::with_items(tags.clone());

        Ok((bookmarks, bookmark_items, tags, tags_items))
    }

    pub fn add_char(&mut self, key_event: KeyEvent) -> Result<()> {
        match self.mode {
            Mode::Search(ref mut params) => match params.active_window {
                // TODO: remove code duplication
                ActiveWindow::Link => {
                    params.link.handle_event(&Event::Key(key_event));

                    let new_state = search(self.bookmarks.clone(), params.link.to_string(), false)?;
                    self.bookmarks_items.unselect();
                    self.bookmarks_items.items = new_state;
                }
                ActiveWindow::Title => {
                    params.title.handle_event(&Event::Key(key_event));

                    let new_state = search(self.bookmarks.clone(), params.title.to_string(), true)?;
                    self.bookmarks_items.unselect();
                    self.bookmarks_items.items = new_state;
                }
                ActiveWindow::Tags => {}
            },
            Mode::Create(ref mut params) => match params.active_window {
                ActiveWindow::Link => {
                    params.link.handle_event(&Event::Key(key_event));
                }
                ActiveWindow::Title => {
                    params.title.handle_event(&Event::Key(key_event));
                }
                ActiveWindow::Tags => {}
            },
            Mode::Scrolling => {}
        }
        Ok(())
    }

    pub fn toggle_help_render(&mut self) {
        self.render_help = !self.render_help;
    }
}

fn search<T: AsRef<str>>(
    bookmarks: Vec<Bookmark>,
    search_string: T,
    for_titles: bool,
) -> Result<Vec<Bookmark>> {
    if search_string.as_ref().is_empty() {
        return Ok(bookmarks);
    }

    let options = SearchOptions::new().stop_words(vec!["/".to_string(), r"\\".to_string()]);
    let mut engine: SimSearch<u32> = SimSearch::new_with(options);

    for bookmark in bookmarks.iter().filter(|b| b.title.is_some()) {
        if for_titles {
            let title = bookmark.title.clone().unwrap_or(String::new());
            engine.insert(bookmark.id.try_into()?, &title);
        } else {
            let url: String = bookmark.url.clone().into();
            engine.insert(bookmark.id.try_into()?, &url);
        }
    }

    let sorted_links = engine
        .search(search_string.as_ref())
        .iter()
        .filter_map(|id| bookmarks.iter().find(|b| b.id == i64::from(*id)).cloned())
        .take(15)
        .collect();

    Ok(sorted_links)
}

#[derive(Debug, Default)]
pub enum Mode {
    Search(SearchingParams),
    Create(CreationParams),
    #[default]
    Scrolling,
}

#[derive(Debug, Default, Clone)]
pub struct CreationParams {
    pub active_window: ActiveWindow,
    pub title: Input,
    pub link: Input,
    pub tags_items: StatefulList<Tag>,
    pub selected_tags: Vec<Tag>,
}

#[derive(Debug, Default)]
pub struct SearchingParams {
    pub active_window: ActiveWindow,
    pub title: Input,
    pub link: Input,
    pub tags: Option<Vec<Tag>>,
}

impl Clone for StatefulList<Tag> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            items: self.items.clone(),
        }
    }
}

impl Default for StatefulList<Tag> {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: Vec::default(),
        }
    }
}

impl Debug for StatefulList<Tag> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatefulList")
            .field("state", &self.state)
            .field("items", &self.items)
            .finish()
    }
}

#[derive(Debug, Default, Clone)]
pub enum ActiveWindow {
    #[default]
    Title,
    Link,
    Tags,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        if self.items.is_empty() {
            return;
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
