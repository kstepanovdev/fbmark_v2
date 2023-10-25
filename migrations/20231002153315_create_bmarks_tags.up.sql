CREATE TABLE bmarks_tags (
    bookmark_id INTEGER REFERENCES bookmarks(id) ON DELETE CASCADE,
    tag_id UUID REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (bookmark_id, tag_id)
);
