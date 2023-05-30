-- This file should undo anything in `up.sql`
ALTER TABLE reviews ALTER created_at TYPE timestamp;
ALTER TABLE reviews ALTER updated_at TYPE timestamp;

ALTER TABLE users ALTER created_at TYPE timestamp;
ALTER TABLE users ALTER updated_at TYPE timestamp;