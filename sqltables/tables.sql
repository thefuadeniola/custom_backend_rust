USE `blog_server`;

CREATE TABLE `blog_server`.`blogs` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `title` VARCHAR(256) NOT NULL,
    `body` VARCHAR(512) NULL,
    -- status can be "New" or "Completed"
    `reads_count` INT NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);

CREATE TABLE `blog_server`.`users` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `username` VARCHAR(256) NOT NULL,
    `email` TEXT NULL,
    `password_hash` VARCHAR(256) NOT NULL,
    PRIMARY KEY (`id`)
);