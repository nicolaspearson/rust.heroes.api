-- The database is created by the docker-compose.yml
-- entrypoint using the credentials specified in the env varaibles
SET search_path TO hero;

CREATE SCHEMA IF NOT EXISTS hero;

BEGIN;

CREATE TABLE IF NOT EXISTS hero.hero (
	id SERIAL NOT NULL,
	"name" character varying(255) NOT NULL,
	"identity" character varying(255) NOT NULL,
	hometown character varying(255) NOT NULL,
	age INT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	deleted_at TIMESTAMP WITH TIME ZONE DEFAULT NULL,
	PRIMARY KEY (id)
);

COMMIT;