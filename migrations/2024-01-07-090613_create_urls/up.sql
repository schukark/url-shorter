CREATE TABLE urls (
    id SERIAL PRIMARY KEY,
    short_url TEXT NOT NULL,
    long_url TEXT NOT NULL
)