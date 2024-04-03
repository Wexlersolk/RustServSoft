CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS access_table (
    "access_id" int PRIMARY KEY NOT NULL,
    "group_name" varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS user_table (
    "user_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "login" varchar,
    "password" varchar,
    "access_id" int DEFAULT(1) REFERENCES access_table(access_id),
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS book_table (
    "book_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "name" varchar ,
    "author" uuid NOT NULL REFERENCES user_table(user_id),
    "score" float,
    "cost" float, 
    "file_name" varchar,
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS comments_table (
    "comment_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "comment_text" TEXT NOT NULL,
    "comment_author" VARCHAR NOT NULL,
    "commented_book" varchar NOT NULL,
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()

);

INSERT INTO access_table ("access_id", "group_name") VALUES 
(1, 'regular'),
(2, 'subscriber'),
(3, 'admin');

INSERT INTO user_table ("login", "password", "access_id") VALUES 
('user1', '0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e', 1),
('user2', '6cf615d5bcaac778352a8f1f3360d23f02f34ec182e259897fd6ce485d7870d4', 2),
('user3', '5906ac361a137e2d286465cd6588ebb5ac3f5ae955001100bc41577c3d751764', 3);

