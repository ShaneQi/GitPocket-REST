CREATE TABLE `hosts` (
	`id`	INTEGER NOT NULL PRIMARY KEY,
	`name`	TEXT NOT NULL,
	`url`	TEXT NOT NULL
);

INSERT INTO `hosts` VALUES (1, 'GitHub', 'https://github.com/')