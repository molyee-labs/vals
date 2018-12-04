CREATE SCHEMA IF NOT EXISTS vals;
GRANT ALL SCHEMA vals TO vals;

CREATE TYPE vals.account AS (
        id BIGINT,
        email VARCHAR(42),
        phone VARCHAR(22),
        hash VARCHAR(255),
        salt VARCHAR(255)
    );
CREATE SEQUENCE vals.account_id_seq START 1 NO CYCLE;
CREATE TABLE IF NOT EXISTS vals.accounts OF vals.account (
        PRIMARY KEY (id) USING INDEX TABLESPACE vals_index_space,
        UNIQUE (email) USING INDEX TABLESPACE vals_index_space,
        UNIQUE (phone) USING INDEX TABLESPACE vals_index_space,
        id WITH OPTIONS DEFAULT nextval('vals.account_id_seq'),
        hash WITH OPTIONS NOT NULL,
        salt WITH OPTIONS NOT NULL
    )
    TABLESPACE vals_db_space;
ALTER SEQUENCE vals.account_id_seq OWNED BY vals.accounts.id;

CREATE TYPE vals.address AS (
        id UUID,
        owner BIGINT,
        balance BIGINT
    );
CREATE TABLE IF NOT EXISTS vals.addresses OF vals.address (
        PRIMARY KEY (id) INCLUDE (owner) USING INDEX TABLESPACE vals_index_space,
        FOREIGN KEY (owner) REFERENCES vals.accounts,
        owner WITH OPTIONS NOT NULL,
        balance WITH OPTIONS NOT NULL DEFAULT 0
    )
    TABLESPACE vals_db_space;