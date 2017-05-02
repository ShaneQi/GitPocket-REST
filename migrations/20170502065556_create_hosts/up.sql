CREATE TABLE `hosts` (
	`host_id`	INTEGER NOT NULL PRIMARY KEY,
	`host_name`	TEXT NOT NULL,
	`host_url`	TEXT NOT NULL
);

INSERT INTO `hosts` VALUES (1, 'GitHub', 'https://github.com/')