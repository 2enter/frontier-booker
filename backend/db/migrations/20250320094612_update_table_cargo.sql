-- migrate:up
ALTER TABLE cargo ADD COLUMN pending BOOLEAN default true;

-- migrate:down
ALTER TABLE cargo DROP COLUMN pending;
