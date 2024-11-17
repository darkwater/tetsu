CREATE TABLE IF NOT EXISTS watch_progress (
    aid              INTEGER NOT NULL PRIMARY KEY,
    last_eid         INTEGER NOT NULL,
    episode_progress REAL NOT NULL,
    anime_progress   REAL NOT NULL,
    last_updated     INTEGER NOT NULL
);
