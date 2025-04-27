USE `blog_server`;

CREATE TABLE `blog_server`.`blogs` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `title` VARCHAR(256) NOT NULL,
    `body` VARCHAR(512) NULL,
    -- status can be "New" or "Completed"
    `reads` INT NOT NULL,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);