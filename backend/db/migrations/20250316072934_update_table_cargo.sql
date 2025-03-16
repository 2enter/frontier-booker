-- migrate:up
ALTER TABLE cargo
	ADD COLUMN name VARCHAR(255),
	ADD COLUMN description VARCHAR(255);

-- migrate:down
ALTER TABLE cargo
	DROP COLUMN name,
	DROP COLUMN description;
