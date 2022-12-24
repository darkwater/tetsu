CREATE TABLE IF NOT EXISTS anime (
    aid                 INTEGER PRIMARY KEY,
    dateflags           INTEGER,
    year                TEXT,
    atype               TEXT,
    related_aid_list    TEXT,
    related_aid_type    TEXT,
    romaji_name         TEXT,
    kanji_name          TEXT,
    english_name        TEXT,
    short_name_list     TEXT,
    episodes            INTEGER,
    special_ep_count    INTEGER,
    air_date            INTEGER,
    end_date            INTEGER,
    picname             TEXT,
    nsfw                BOOLEAN,
    characterid_list    TEXT,
    specials_count      INTEGER,
    credits_count       INTEGER,
    other_count         INTEGER,
    trailer_count       INTEGER,
    parody_count        INTEGER,
    last_updated        INTEGER
);

CREATE TABLE IF NOT EXISTS episodes (
    eid                 INTEGER PRIMARY KEY,
    aid                 INTEGER,
    length              INTEGER,
    rating              INTEGER,
    votes               INTEGER,
    epno                TEXT,
    eng                 TEXT,
    romaji              TEXT,
    kanji               TEXT,
    aired               INTEGER,
    etype               INTEGER
    last_updated        INTEGER,

    FOREIGN KEY (aid) REFERENCES anime (aid)
);

CREATE TABLE IF NOT EXISTS files (
    fid                 INTEGER PRIMARY KEY,
    aid                 INTEGER,
    eid                 INTEGER,
    gid                 INTEGER,
    state               INTEGER,
    size                INTEGER,
    ed2k                TEXT,
    colour_depth        TEXT,
    quality             TEXT,
    source              TEXT,
    audio_codec_list    TEXT,
    audio_bitrate_list  INTEGER,
    video_codec         TEXT,
    video_bitrate       INTEGER,
    video_resolution    TEXT,
    dub_language        TEXT,
    sub_language        TEXT,
    length_in_seconds   INTEGER,
    description         TEXT,
    aired_date          INTEGER,
    last_updated        INTEGER,

    FOREIGN KEY (aid) REFERENCES anime (aid),
    FOREIGN KEY (eid) REFERENCES episodes (eid),
    FOREIGN KEY (gid) REFERENCES groups (gid)
);

CREATE TABLE IF NOT EXISTS groups (
    gid                 INTEGER PRIMARY KEY,
    rating              INTEGER,
    votes               INTEGER,
    acount              INTEGER,
    fcount              INTEGER,
    name                TEXT,
    short               TEXT,
    irc_channel         TEXT,
    irc_server          TEXT,
    url                 TEXT,
    picname             TEXT,
    foundeddate         INTEGER,
    disbandeddate       INTEGER,
    dateflags           INTEGER,
    lastreleasedate     INTEGER,
    lastactivitydate    INTEGER,
    grouprelations      TEXT,
    last_updated        INTEGER
);

CREATE TABLE IF NOT EXISTS indexed_files (
    path                TEXT PRIMARY KEY,
    filename            TEXT,
    filesize            INTEGER,
    ed2k                TEXT,
    fid                 INTEGER,
    last_updated        INTEGER,

    UNIQUE (filename, filesize) ON CONFLICT REPLACE,
    FOREIGN KEY (fid) REFERENCES files (fid)
);
