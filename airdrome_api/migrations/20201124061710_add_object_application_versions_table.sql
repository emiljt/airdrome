-- Add migration script here
CREATE TABLE IF NOT EXISTS `object_application_versions` (
    `id` TINYINT UNSIGNED NOT NULL AUTO_INCREMENT,
    `number` VARCHAR(32),
    `created` DATETIME NOT NULL,
    `commit` VARCHAR(40),
    `zip_hash` VARCHAR(40) NOT NULL,
    `object_id` MEDIUMINT UNSIGNED NOT NULL,

    PRIMARY KEY (`id`)
    CONSTRAINT `object_versions_objects_fk`
        FOREIGN KEY (`object_id`) REFERENCES `object_application_objects` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
