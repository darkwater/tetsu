CREATE TABLE IF NOT EXISTS platform_links (
    id                  INTEGER NOT NULL PRIMARY KEY,
    animebytes_id       INTEGER NOT NULL DEFAULT 0,
    anidb_id            INTEGER NOT NULL DEFAULT 0,
    ann_id              INTEGER NOT NULL DEFAULT 0,
    anilist_id          INTEGER NOT NULL DEFAULT 0,
    mal_id              INTEGER NOT NULL DEFAULT 0,

    UNIQUE (animebytes_id, ann_id, anilist_id, mal_id) ON CONFLICT ABORT
);
