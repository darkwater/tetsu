CREATE TABLE IF NOT EXISTS animebytes_groups (
    id    INTEGER NOT NULL PRIMARY KEY,
    data  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS animebytes_torrents (
    torrent_id    INTEGER NOT NULL PRIMARY KEY,
    group_id      INTEGER NOT NULL
);
