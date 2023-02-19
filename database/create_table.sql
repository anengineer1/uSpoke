CREATE TABLE messages(
       id serial PRIMARY KEY,
       message_datetime TIMESTAMP with time zone,
       username VARCHAR(255),
       message TEXT
);
