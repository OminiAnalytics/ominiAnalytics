-- Your SQL goes here
/* add this columns to omini_calls
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
  omini_calls
ADD
  u_id uuid NOT NULL REFERENCES omini_users (id);

ALTER TABLE
  omini_calls
ADD
  ptitle varchar(200) NOT NULL;

ALTER TABLE
  omini_calls
ADD
  purl varchar(200) NOT NULL;

ALTER TABLE
  omini_calls
ADD
  pmdata jsonb NOT NULL;

ALTER TABLE
  omini_calls
ADD
  created_at bigint DEFAULT EXTRACT(
    EPOCH
    FROM
      NOW()
  ) NOT NULL;

ALTER TABLE
  omini_calls
ADD
  ip varchar(45) NOT NULL;