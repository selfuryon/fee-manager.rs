-- Add up migration script here

CREATE TABLE proposers(
  validator     TEXT NOT NULL UNIQUE,
  fee_recipient TEXT NOT NULL,
  gas_limit     INTEGER DEFAULT 0,
  grace         INTEGER DEFAULT 0,
  min_value     REAL    DEFAULT 0,
  reset_relays  BOOLEAN DEFAULT false,
  PRIMARY KEY (validator)
);
