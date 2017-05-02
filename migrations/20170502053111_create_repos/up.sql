-- Your SQL goes here
CREATE TABLE `repos` (
	`user_id`	INTEGER NOT NULL,
	`host`	INTEGER NOT NULL,
	`owner`	TEXT NOT NULL,
	`name`	TEXT NOT NULL,
	PRIMARY KEY(`user_id`,`host`,`owner`,`name`)
)