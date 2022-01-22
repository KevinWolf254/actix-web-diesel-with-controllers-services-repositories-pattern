-- Your SQL goes here
CREATE TABLE IF NOT EXISTS organisations
(
    id SERIAL NOT NULL,
    name character varying(100) NOT NULL,
    description character varying(100),
    vat_number character varying(35),
    pin_number character varying(35) NOT NULL,
    created timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated timestamp with time zone,
    CONSTRAINT organisations_pkey PRIMARY KEY (id)
)