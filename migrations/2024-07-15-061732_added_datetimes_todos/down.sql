-- This file should undo anything in `up.sql`
-- This file should undo anything in `up.sql`
ALTER TABLE todos 
  DROP COLUMN created_at;

ALTER TABLE todos 
  DROP COLUMN updated_at;