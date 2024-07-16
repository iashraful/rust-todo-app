-- This file should undo anything in `up.sql`
ALTER TABLE labels 
  DROP COLUMN created_at;

ALTER TABLE labels 
  DROP COLUMN updated_at;