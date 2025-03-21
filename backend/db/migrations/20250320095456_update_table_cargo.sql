-- migrate:up
ALTER TABLE cargo ALTER COLUMN pending SET default false;

-- migrate:down
ALTER TABLE cargo ALTER COLUMN pending SET default true;
