-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.
DROP FUNCTION IF EXISTS diesel_manage_updated_at (_tbl regclass);

DROP FUNCTION IF EXISTS diesel_set_updated_at ();

DROP TABLE IF EXISTS omini_alive_messages;

DROP TABLE IF EXISTS omini_calls;

-- omini_users should always be the last dropped table
DROP TABLE IF EXISTS omini_users;

