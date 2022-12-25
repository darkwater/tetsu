CREATE TABLE IF NOT EXISTS anime (
    aid     INTEGER PRIMARY KEY,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS episodes (
    eid     INTEGER PRIMARY KEY,
    aid     INTEGER NOT NULL,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
    fid     INTEGER PRIMARY KEY,
    aid     INTEGER NOT NULL,
    eid     INTEGER NOT NULL,
    gid     INTEGER NOT NULL,
    size    INTEGER NOT NULL,
    ed2k    TEXT NOT NULL,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
    gid     INTEGER PRIMARY KEY,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS indexed_files (
    path                TEXT PRIMARY KEY,
    filename            TEXT NOT NULL,
    filesize            INTEGER NOT NULL,
    ed2k                TEXT NOT NULL,
    fid                 INTEGER,
    first_seen          INTEGER NOT NULL,
    last_updated        INTEGER NOT NULL,

    UNIQUE (filename, filesize) ON CONFLICT REPLACE
);

CREATE TABLE IF NOT EXISTS settings (
    key                 TEXT PRIMARY KEY,
    value               TEXT NOT NULL
);
