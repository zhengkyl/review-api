-- Your SQL goes here
ALTER TABLE reviews ADD COLUMN season INTEGER NOT NULL DEFAULT -1;

UPDATE reviews SET season = 1 WHERE category = 'Show';

ALTER TABLE reviews DROP CONSTRAINT reviews_pkey;

ALTER TABLE reviews ADD CONSTRAINT reviews_pkey PRIMARY KEY (user_id, tmdb_id, category, season);

