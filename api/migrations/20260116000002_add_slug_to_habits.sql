-- Danger: Truncate data to ensure clean migration for unique constraint
TRUNCATE TABLE habits;

ALTER TABLE habits ADD COLUMN slug TEXT;
-- No update needed since table is empty
ALTER TABLE habits ALTER COLUMN slug SET NOT NULL;
ALTER TABLE habits ADD CONSTRAINT habits_slug_key UNIQUE (slug);
