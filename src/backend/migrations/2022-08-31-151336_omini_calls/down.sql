-- This file should undo anything in `up.sql`
/* delete this columns to omini_calls
u_id uuid NOT NULL REFERENCES omini_users (id),
ptitle varchar(200) NOT NULL,
purl varchar(200) NOT NULL,
pmdata jsonb NOT NULL,
created_at bigint DEFAULT EXTRACT(
EPOCH
FROM
NOW()
) NOT NULL,
ip varchar(45) NOT NULL
 */
ALTER TABLE
  omini_calls DROP COLUMN u_id;

ALTER TABLE
  omini_calls DROP COLUMN ptitle;

ALTER TABLE
  omini_calls DROP COLUMN purl;

ALTER TABLE
  omini_calls DROP COLUMN pmdata;

ALTER TABLE
  omini_calls DROP COLUMN created_at;

ALTER TABLE
  omini_calls DROP COLUMN ip;