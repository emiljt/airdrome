USE `airdrome`

/*
Object application
*/

CREATE TABLE IF NOT EXISTS `object_application_languages` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL UNIQUE,

    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `object_application_targets` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(100) NOT NULL UNIQUE,

    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `object_application_repositories` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,

    PRIMARY KEY (`id`)
);

CREATE TABLE IF NOT EXISTS `object_application_objects` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `guid` VARCHAR(32) NOT NULL UNIQUE,
    `name` VARCHAR(100) NOT NULL UNIQUE,
    `description` VARCHAR(2500),
    `repository_id` MEDIUMINT UNSIGNED,

    PRIMARY KEY (`id`),
    FULLTEXT INDEX `description_index` (`description`),
    CONSTRAINT `repositories_fk`
        FOREIGN KEY (`repository_id`) REFERENCES `object_application_repositories` (`id`)
        ON UPDATE CASCADE
        ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS `object_application_object_languages` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `object_id` MEDIUMINT UNSIGNED NOT NULL,
    `language_id` TINYINT UNSIGNED NOT NULL,

    PRIMARY KEY (`id`),
    UNIQUE INDEX (`object_id`, `language_id`),
    CONSTRAINT `object_languages_objects_fk`
        FOREIGN KEY (`object_id`) REFERENCES `object_application_objects` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT `object_languages_languages_fk`
        FOREIGN KEY (`language_id`) REFERENCES `object_application_languages` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS `object_application_object_targets` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `object_id` MEDIUMINT UNSIGNED NOT NULL,
    `target_id` TINYINT UNSIGNED NOT NULL,

    PRIMARY KEY (`id`),
    UNIQUE INDEX (`object_id`, `target_id`),
    CONSTRAINT `object_targets_objects_fk`
        FOREIGN KEY (`object_id`) REFERENCES `object_application_objects` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    CONSTRAINT `object_targets_targets_fk`
        FOREIGN KEY (`target_id`) REFERENCES `object_application_targets` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

/*
Repository application
*/

CREATE TABLE IF NOT EXISTS `repository_application_repositories` (
    `id` MEDIUMINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `guid` VARCHAR(32) NOT NULL UNIQUE,
    `url` VARCHAR(100) NOT NULL UNIQUE,
    `path` VARCHAR(2500) NOT NULL UNIQUE,

    PRIMARY KEY (`id`)
);

