CREATE TABLE IF NOT EXISTS hero (
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