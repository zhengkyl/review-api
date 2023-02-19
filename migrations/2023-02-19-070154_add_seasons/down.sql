-- This file should undo anything in `up.sql`

ALTER TABLE reviews DROP CONSTRAINT reviews_pkey;

ALTER TABLE reviews DROP COLUMN season;

ALTER TABLE reviews ADD CONSTRAINT reviews_pkey PRIMARY KEY (user_id, tmdb_id, category);