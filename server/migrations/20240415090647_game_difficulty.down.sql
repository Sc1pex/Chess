-- Add down migration script here
ALTER TABLE games
DROP COLUMN difficulty;
