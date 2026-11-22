-- Schema version 1: titles and the ownership records grouped under them.

CREATE TABLE titles (
    id             INTEGER PRIMARY KEY,
    display_name   TEXT NOT NULL,
    normalized_key TEXT NOT NULL UNIQUE
);

CREATE TABLE ownerships (
    id        INTEGER PRIMARY KEY,
    title_id  INTEGER NOT NULL REFERENCES titles (id),
    origin    TEXT NOT NULL CHECK (origin IN ('store', 'manual')),
    source    TEXT NOT NULL,
    source_id TEXT NOT NULL,
    medium    TEXT NOT NULL CHECK (medium IN ('digital', 'physical', 'rom')),
    raw_title TEXT NOT NULL,
    UNIQUE (origin, source, source_id)
);

CREATE INDEX ownerships_title_id ON ownerships (title_id);
