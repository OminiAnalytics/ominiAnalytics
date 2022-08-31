-- This file should undo anything in `up.sql`
/*
Drop this columns
u_id uuid NOT NULL REFERENCES omini_users (id),
s_id uuid NOT NULL,
created_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL,
mtype varchar NOT NULL,
s_duration integer NOT NULL
 */
ALTER TABLE
  omini_alive_messages DROP COLUMN u_id;

ALTER TABLE
  omini_alive_messages DROP COLUMN s_id;

ALTER TABLE
  omini_alive_messages DROP COLUMN created_at;

ALTER TABLE
  omini_alive_messages DROP COLUMN mtype;

ALTER TABLE
  omini_alive_messages DROP COLUMN s_duration;