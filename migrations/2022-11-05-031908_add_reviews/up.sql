-- Your SQL goes here

CREATE TYPE watch_status AS ENUM ('Completed', 'Dropped', 'Watching', 'PlanToWatch');

CREATE TABLE film_reviews (
  id SERIAL NOT NULL PRIMARY KEY,
  tmdb_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  status watch_status NOT NULL,
  text TEXT NOT NULL,
  fun_before BOOLEAN NOT NULL,
  fun_during BOOLEAN NOT NULL,
  fun_after BOOLEAN NOT NULL,
  updated_at TIMESTAMP NOT NULL
);

CREATE TABLE show_reviews (
  id SERIAL NOT NULL PRIMARY KEY,
  tmdb_id INTEGER NOT NULL,
  user_id INTEGER NOT NULL,
  status watch_status NOT NULL,
  text TEXT NOT NULL,
  fun_before BOOLEAN NOT NULL,
  fun_during BOOLEAN NOT NULL,
  fun_after BOOLEAN NOT NULL,
  updated_at TIMESTAMP NOT NULL
);