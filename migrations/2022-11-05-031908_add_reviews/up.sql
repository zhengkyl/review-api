-- Your SQL goes here

CREATE TYPE watch_status AS ENUM ('Completed', 'Dropped', 'Watching', 'PlanToWatch');
CREATE TYPE media_category AS ENUM ('Film', 'Show');

CREATE TABLE reviews (
  user_id INTEGER NOT NULL,
  tmdb_id INTEGER NOT NULL,
  category media_category NOT NULL,
  status watch_status NOT NULL,
  text TEXT NOT NULL,
  fun_before BOOLEAN NOT NULL,
  fun_during BOOLEAN NOT NULL,
  fun_after BOOLEAN NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  PRIMARY KEY (user_id, tmdb_id, category)
);
