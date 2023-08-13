CREATE TABLE IF NOT EXISTS rates_history (
    id VARCHAR(255) NOT NULL,
    datetime TIMESTAMP NOT NULL,
    bid float8 NOT NULL,
    ask float8 NOT NULL,
	open float8 NOT NULL,
    close float8 NOT NULL,
    price float8 NOT null,
    --
    CONSTRAINT pk_rates_history PRIMARY KEY (datetime, id)
);
