-- Your SQL goes here
CREATE TABLE IF NOT EXISTS organisations
(
    id character varying(255) COLLATE pg_catalog."default" NOT NULL,
    name character varying(255) COLLATE pg_catalog."default" NOT NULL,
    description character varying(255) COLLATE pg_catalog."default",
    vat_number character varying(35) COLLATE pg_catalog."default",
    pin_number character varying(35) COLLATE pg_catalog."default" NOT NULL,
    created timestamp with time zone NOT NULL,
    updated timestamp with time zone,
    CONSTRAINT organisations_pkey PRIMARY KEY (id)
)