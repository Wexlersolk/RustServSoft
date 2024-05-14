CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS access_table (
    "access_id" int PRIMARY KEY NOT NULL,
    "group_name" varchar NOT NULL
);

CREATE TABLE IF NOT EXISTS user_table (
    "user_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    "login" varchar UNIQUE,
    "password" varchar,
    "email" varchar UNIQUE,
    "access_id" int DEFAULT(1) REFERENCES access_table(access_id),
    "created_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    "updated_at" TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS genre_table (
    "genre_id" SERIAL PRIMARY KEY,
    "genre_name" varchar UNIQUE 
);

CREATE TABLE IF NOT EXISTS book_table (
    "book_id" uuid PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
	"genre_id" int REFERENCES genre_table(genre_id),
    "name" varchar,
    "author" varchar REFERENCES user_table(login),
    "description" varchar,
	"cost" float, 
    "score" float,
    "downloads" int,
    "file_name" varchar,
    "file" bytea,
    "img_name" varchar,
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

CREATE TABLE IF NOT EXISTS admin_table (
    "password_name" VARCHAR(255) PRIMARY KEY NOT NULL,
	"password" varchar
);

CREATE TABLE IF NOT EXISTS bougt_books (
    "user_id" uuid REFERENCES user_table(user_id),
    "book_id" uuid REFERENCES book_table(book_id),
    PRIMARY KEY ("user_id", "book_id")
);

CREATE VIEW book_view AS
    SELECT book_id, name, genre_name, author, cost, score, description, downloads, img_name, created_at, updated_at 
    FROM book_table JOIN genre_table ON book_table.genre_id = genre_table.genre_id;

CREATE VIEW book_files AS
    SELECT book_id, file 
    FROM book_table;
---------------------------------------------------------------------------------------------------------
INSERT INTO genre_table ("genre_name") VALUES 
('Fantasy'),
('Science Fiction'),
('Mystery'),
('Romance'),
('Thriller'),
('Horror'),
('Historical Fiction'),
('Non-fiction'),
('Biography'),
('Poetry'),
('Self-help'),
('Cooking'),
('Travel'),
('Young Adult'),
('Children'),
('Classic');

INSERT INTO access_table ("access_id", "group_name") VALUES 
(1, 'regular'),
(2, 'subscriber'),
(3, 'admin');

INSERT INTO user_table ("login", "password",  "email" ,"access_id") VALUES 
('user1',          '0b14d501a594442a01c6859541bcb3e8164d183d32937b851835442f69d5c94e', 'lox@gmail.com',            1),
('user2',          '6cf615d5bcaac778352a8f1f3360d23f02f34ec182e259897fd6ce485d7870d4', 'donkey@gmail.com',         2),
('user3',          '5906ac361a137e2d286465cd6588ebb5ac3f5ae955001100bc41577c3d751764', 'monkey@gmail.com',         3),
('David Nicholls', '6eab73b49ec4fcd0e49e670dd60ed09f0e8041a12deed307e83d41c9f5c9aa3b', 'David_Nicholls@gmail.com', 1),
('A.F Steadman',   'a68a34f6b75e4a26544fd726e146b51a1ab2eef3287fdbe86a20fe98c5a2eb36', 'A.F_Steadman@gmail.com',   1),
('Jojo Moyes',     'c96a0b2a2c59c2c06d2a1a9c20f0fc4c45d17f6a399e78dc63a617a53b138d6d', 'Jojo_Moyes@gmail.com',     1),
('Peter James',    'fad1b1e3df40b84c98042c6df727c8b952937d91b0b40f474ea0f991d1b33e6e', 'Peter_James@gmail.com',    1),
('Hairy Bikers',    '839fe2a265e6715c7c3d0e8fcdd52f40fc7a3c4d4e0da06c78f0e3c04ff6bb34', 'Hairy_Biker@gmail.com',    1),
('Emily Henry',     '8cf937fddfe76068e2a1e8c2e69eeb802c5a4c2cb82c87db66d20d9864c9e8e8', 'Emily_Henr@gmail.com',     1),
('Shari Lapena',   '26d6d061f2e6ebe0e63b1ffdbed1553d308b20634ea03812458000d2fa824c1d', 'Shari_Lapena@gmail.com',   1),
('Nathan Anthony',  '9d96e437f24085b9710b7f73f4a533dbac5f2e032b15a8639e73c1c4ccf5939f', 'Nathan_Anthon@gmail.com',  1),
('Lee Child',      '1f3a68a6db268e4c00d8f999634a14b4f1be19c2abf27b72e65b9be13d1c29f7', 'Lee_Child@gmail.com',      1),
('Rebecca Yarros', '80c3db0d791a4b9aa0f07db53f8d52ecf78bba3d10e107d2a1f3928a3cc992fb', 'Rebecca_Yarros@gmail.com',  1),
('BBC',            'f517c3e820107a6a9f1d5186a3a4e27d4d48c621843e124e8ad4e9ecba8ab4f7', 'BBC@gmail.com',             1),
('Chris Broad',     'a1eaf83e8b53da8ee27d0811520225175f110581c9852ee2094de3bb96c7f623', 'Chris_Broa@gmail.com',     1),
('Heather Morris', '0f3567dfb3d4877f4d8ee3ef0a472e59ef8889fcda3d39de17f34a2925487730', 'Heather_Morris@gmail.com', 1),
('Julia Donaldson','e5b3dc3a1f6d07aa51f4c001bcaae8f2072f43ff00cb8b48aae02e1c4b0a9a77', 'Julia_Donaldson@gmailcom', 1),
('James Brien',  '3fc12f7c23f30a33d06ff3655820ec1f022708d7642c7002d83707d31ff62d2a', 'James_O`Brien@gmail.com',  1),
('Dilly Court',    'e0d5d2b02856e18ec4d6e935d834cf5845c8fde9a96bcbef5cb876efff4ec5bc', 'Dilly_Court@gmail.com',    1),
('Karin Slaughter','540a44f6417f5e7da1a11d384e0139b6c12cfed8a89f96cf58b31d0148b4ed86', 'Karin_Slaughter@gmailcom', 1),
('Liz Earle',      '60e7bb91e25375c783f89e90b4b481d6b42f053a4292c94da150dd84b6e9d8ef', 'Liz_Earle@gmail.com',      1),
('Dav Pilkey',     'cb17c1907f962d99ae3fd0a6866d4601de203d32a82f5297a3aa2c96e0b856b8', 'Dav_Pilkey@gmail.com',     1),
('Lisa Jewell',    'd78c32cf8d25db7d09ef24016f89cb08e7aa5edebf0ae605c613b44d3b09dcb4', 'Lisa_Jewell@gmai.com',     1),
('Liz Nugent',     'd8964c87db2b82b51c1f59d94a431b5a4dc25f80a1c3dc4a0112a8432dbb2cb1', 'Liz_Nugent@gmail.com',      1),
('Sarah J. Maas',  '0ce34c1e1b3e754fd1a7b0f72e5aeed775ff076b4f1158dc540d8a0f837fb7e2', 'Sarah_J.Maas@gmail.com',   1);



INSERT INTO book_table ("genre_id", "name", "author", "cost", "score", "downloads", "file_name", "img_name")
VALUES
    (1, 'The Galactic Chronicles: Odyssey of the Stars', 'user1',   12.99, 4.6, 150, 'galactic_chronicles.pdf',          'galactic_chronicles.jpg'),
    (2, 'The Sword of Destiny: Realm of Kings',          'user2',   11.99, 4.8, 180, 'sword_of_destiny.pdf',             'sword_of_destiny.jpg'),
    (1, 'Echoes of Eternity: Beyond the Event Horizon',  'user1',   14.99, 4.7, 200, 'echoes_of_eternity.pdf',           'echoes_of_eternity.jpg'),
    (2, 'The Lost Kingdoms: Legends of Eldoria',         'user3',   10.99, 4.5, 170, 'lost_kingdoms.pdf',                'lost_kingdoms.jpg'),
    (9, 'You Are Here',                         'David Nicholls',   14.99, 4.7, 180, 'you_are_here.pdf',                 'you_are_here.jpg'),
    (1, 'Skandar and the Chaos Trials',           'A.F Steadman',   13.99, 4.8, 210, 'skandar_and_the_Chaos_trials.pdf', 'skandar_and_the_chaos_trials.jpg'),
    (3, 'Someone Else`s Shoes',                     'Jojo Moyes',    7.49, 4.5, 120, 'someone_else`s_shoes.pdf',         'someone_elses_shoes.jpg'),
    (10,'Stop Them Dead',                          'Peter James',    4.99, 3.7,  80, 'stop_them_dead.pdf',               'stop_them_dead.jpg'),
    (11,'The Hairy Dieters',                      'Hairy Bikers',   17.47, 4.8, 250, 'the_hairy_dieters.pdf',            'the_hairy_dieters.jpg'),
    (9, 'Funny Story',                             'Emily Henry',   12.99, 4.0, 165, 'funny_story.pdf',                  'funny_story.jpg'),
    (9, 'Everyone Here is Lying',                 'Shari Lapena',    7.49, 4.4, 135, 'everyone_here_is_lying.pdf',       'everyone_here_is_lying.jpg'),
    (11,'Bored of Lunch',                       'Nathan Anthony',   10.00, 3.4,  60, 'bored_of_lunch.pdf',               'bored_of_lunch.jpg'),
    (3, 'The Secret',                                'Lee Child',    7.49, 4.7, 140, 'the_secret.pdf',                   'the_secret.jpg'),
    (2, 'Fourth Wing',                          'Rebecca Yarros',    4.99, 4.3, 280, 'fourth_wing.pdf',                  'fourth_wing.jpg'),
    (13,'BBC Proms',                                       'BBC',    9.19, 4.9, 400, 'bbc_proms.pdf',                    'bbc_proms.jpg'),
    (12,'Abroad in Japan',                         'Chris Broad',    8.24, 4.1, 155, 'abroad_in_japan.pdf',              'abroad_in_japan.jpg'),
    (6, 'The Tattooist of Auschwitz',           'Heather Morris',    7.49, 5.0, 148, 'the_tattooist_of_auschwitz.pdf',   'the_tattooist_of_auschwitz.jpg'),
    (13,'Frog`s Day Out',                      'Julia Donaldson',    7.99, 4.2, 230, 'frog`s_day_out.pdf',               'frogs_day_out.jpg'),
    (14,'How They Broke Britain',                'James Brien',    8.24, 4.7, 330, 'how_they_broke_britain.pdf',       'how_they_broke_britain.jpg'),
    (15,'The Best of Daughters',                   'Dilly Court',    8.27, 4.2, 140, 'the_best_of_daughters.pdf',        'the_best_of_daughters.jpg'),
    (1, 'After That Night',                    'Karin Slaughter',    7.49, 4.5, 250, 'after_that_night.pdf',             'after_that_night.jpg'),
    (10,'A Better Second Half',                      'Liz Earle',   20.24, 4.9,  80, 'a_better_second_half.pdf',         'a_better_second_half.jpg'),
    (14,'Dog Man 12',                               'Dav Pilkey',   12.49, 4.2, 340, 'dog_man_12.pdf',                   'dog_man_12.jpg'),
    (1, 'None of This is True',                    'Lisa Jewell',    9.99, 4.3, 120, 'none_of_this_is_true.pdf',         'none_of_this_is_true.jpg'),
    (10,'Strange Sally Diamond',                    'Liz Nugent',    6.74, 4.7, 325, 'strange_sally_diamond.pdf',        'strange_sally_diamond.jpg'),
    (8, 'A Court of Thorns and Roses',           'Sarah J. Maas',    8.27, 3.8, 170, 'a_court_of_thorns_and_roses.pdf',  'a_court_of_thorns_and_roses.jpg');
