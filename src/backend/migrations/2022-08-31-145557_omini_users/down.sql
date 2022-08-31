-- This file should undo anything in `up.sql`
ALTER TABLE
  omini_users DROP COLUMN created_at;

ALTER TABLE
  omini_users DROP COLUMN updated_at;

ALTER TABLE
  omini_users DROP COLUMN device_info;

ALTER TABLE
  omini_users DROP COLUMN country;

ALTER TABLE
  omini_users DROP COLUMN ip;