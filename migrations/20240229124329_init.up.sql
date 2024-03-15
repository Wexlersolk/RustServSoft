CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS access_table (
    "access_id" int PRIMARY KEY NOT NULL ,
    "group_name" varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS user_table (
    "user_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "login" varchar NOT NULL,
    "password" varchar NOT NULL,
    "access_id" int NOT NULL REFERENCES access_table(access_id),
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);


INSERT INTO access_table ("access_id", "group_name") VALUES 
(1, 'regular'),
(2, 'subscriber'),
(3, 'admin');

INSERT INTO user_table ("login", "password", "access_id") VALUES 
('user1', 'password1', 1),
('user2', 'password2', 2),
('user3', 'password3', 3);

