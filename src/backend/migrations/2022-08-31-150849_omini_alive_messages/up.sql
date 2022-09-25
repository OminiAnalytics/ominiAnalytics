-- Your SQL goes here
/*
Add this columns to omini_alive_messages
u_id uuid NOT NULL REFERENCES omini_users (id),
s_id uuid NOT NULL,
created_at bigint DEFAULT EXTRACT(EPOCH FROM NOW()) NOT NULL,
mtype varchar NOT NULL,
s_duration integer NOT NULL
 */
ALTER TABLE omini_alive_messages
  ADD u_id uuid NOT NULL REFERENCES omini_users (id);

ALTER TABLE omini_alive_messages
  ADD s_id uuid NOT NULL;

ALTER TABLE omini_alive_messages
  ADD created_at bigint DEFAULT extract(epoch FROM now()) NOT NULL;

ALTER TABLE omini_alive_messages
  ADD mtype varchar NOT NULL;

ALTER TABLE omini_alive_messages
  ADD s_duration integer NOT NULL;

