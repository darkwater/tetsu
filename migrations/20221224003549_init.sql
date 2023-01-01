CREATE TABLE IF NOT EXISTS anime (
    aid     INTEGER NOT NULL PRIMARY KEY,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS episodes (
    eid     INTEGER NOT NULL PRIMARY KEY,
    aid     INTEGER NOT NULL,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
    fid     INTEGER NOT NULL PRIMARY KEY,
    aid     INTEGER NOT NULL,
    eid     INTEGER NOT NULL,
    gid     INTEGER NOT NULL,
    size    INTEGER NOT NULL,
    ed2k    TEXT NOT NULL,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS groups (
    gid     INTEGER NOT NULL PRIMARY KEY,
    json    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS indexed_files (
    path                TEXT NOT NULL PRIMARY KEY,
    filename            TEXT NOT NULL,
    filesize            INTEGER NOT NULL,
    ed2k                TEXT NOT NULL,
    fid                 INTEGER,
    first_seen          INTEGER NOT NULL,
    last_updated        INTEGER NOT NULL,

    UNIQUE (filename, filesize) ON CONFLICT REPLACE
);

CREATE TABLE IF NOT EXISTS settings (
    key                 TEXT NOT NULL PRIMARY KEY,
    value               TEXT NOT NULL
);
