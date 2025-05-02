-- Create the table for a generic key-value store.

CREATE TABLE key_values
(
    key   TEXT PRIMARY KEY NOT NULL,
    value BLOB             NULL
) STRICT, WITHOUT ROWID;
