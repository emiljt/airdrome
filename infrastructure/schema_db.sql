USE `airdrome_test`

CREATE TABLE IF NOT EXISTS `languages` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL UNIQUE,

    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `targets` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL UNIQUE,

    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `repositories` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,

    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `objects` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `guid` VARCHAR(32) NOT NULL UNIQUE,
    `name` VARCHAR(100) NOT NULL UNIQUE,
    `description` VARCHAR(2500),
    `repository_id` MEDIUMINT UNSIGNED,

    PRIMARY KEY (`id`),
    CONSTRAINT `repositories_fk`
        FOREIGN KEY (`repository_id`) REFERENCES `repositories` (`id`)
        ON UPDATE CASCADE
        ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS `object_languages` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `object_id` MEDIUMINT UNSIGNED NOT NULL,
    `language_id` TINYINT UNSIGNED NOT NULL,

    PRIMARY KEY (`id`),
    CONSTRAINT `object_languages_objects_fk`
        FOREIGN KEY (`object_id`) REFERENCES `objects` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT `object_languages_languages_fk`
        FOREIGN KEY (`language_id`) REFERENCES `languages` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `object_targets` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `object_id` MEDIUMINT UNSIGNED NOT NULL,
    `target_id` TINYINT UNSIGNED NOT NULL,

    PRIMARY KEY (`id`),
    CONSTRAINT `object_targets_objects_fk`
        FOREIGN KEY (`object_id`) REFERENCES `objects` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT `object_targets_targets_fk`
        FOREIGN KEY (`target_id`) REFERENCES `targets` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
