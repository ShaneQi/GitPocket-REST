CREATE TABLE `tokens` (
	`token`	INTEGER NOT NULL PRIMARY KEY UNIQUE,
	`user_id`	INTEGER NOT NULL,
	`created_at`	NUMERIC NOT NULL
);