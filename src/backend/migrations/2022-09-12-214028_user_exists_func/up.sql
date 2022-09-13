-- Your SQL goes here
CREATE OR REPLACE FUNCTION USER_EXISTS (IPa varchar(45), DEVICE jsonb, COUNTRYa varchar)
    RETURNS varchar
    AS $$
DECLARE
    RESULT varchar;
BEGIN
    SELECT
        id
    FROM
        OMINI_USERS
    WHERE
        ip = IPa
        AND device_info = DEVICE
        AND country = COUNTRYa INTO RESULT;
    IF RESULT IS NULL THEN
        RETURN 'false';
    ELSE
        RETURN RESULT;
    END IF;
END;
$$
LANGUAGE plpgsql;

