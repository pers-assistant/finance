-- Create Subscriptions Table
CREATE TABLE transaction(
    id uuid NOT NULL ,
    PRIMARY KEY (id),
    title TEXT NOT NULL,
    amount INTEGER NOT NULL ,
    type_operation TEXT NOT NULL
)