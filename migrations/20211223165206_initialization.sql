CREATE TABLE urls (
    id text PRIMARY KEY,
    url text NOT NULL,
    created text NOT NULL DEFAULT CURRENT_TIMESTAMP
);
