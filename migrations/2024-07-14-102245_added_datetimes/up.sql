-- Your SQL goes here
ALTER TABLE labels 
  ADD COLUMN created_at TIMESTAMP NOT NULL DEFAULT NOW();
ALTER TABLE labels 
  ADD COLUMN updated_at TIMESTAMP NOT NULL DEFAULT NOW();