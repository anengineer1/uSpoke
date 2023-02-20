CREATE TABLE messages(
       id serial PRIMARY KEY NOT NULL,
       created_at_utc TIMESTAMP with time zone NOT NULL,
       username VARCHAR(255) NOT NULL,
       message TEXT NOT NULL
);
