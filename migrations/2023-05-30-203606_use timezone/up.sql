-- Your SQL goes here
SET timezone = 'UTC';

ALTER TABLE reviews ALTER created_at TYPE timestamptz;
ALTER TABLE reviews ALTER updated_at TYPE timestamptz;

ALTER TABLE users ALTER created_at TYPE timestamptz;
ALTER TABLE users ALTER updated_at TYPE timestamptz;