-- Object application

CREATE TABLE IF NOT EXISTS `object_application_languages` (
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL,
    `name` TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS `object_application_targets` (
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL,
    `name` TEXT UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS `object_application_repositories` (
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL
);

CREATE TABLE IF NOT EXISTS `object_application_objects` (
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL,
    `name` TEXT UNIQUE NOT NULL,
    `description` TEXT,
    `repository_id` TEXT,

    FOREIGN KEY (`repository_id`) REFERENCES `object_application_repositories` (`id`)
        ON UPDATE CASCADE
        ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS `object_application_object_languages` (
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL,
    `object_id` TEXT NOT NULL,
    `language_id` TEXT NOT NULL,

    FOREIGN KEY (`object_id`) REFERENCES `object_application_objects` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY (`language_id`) REFERENCES `object_application_languages` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX `object_language_index`
ON `object_application_object_languages` (`object_id`, `language_id`);

CREATE TABLE IF NOT EXISTS `object_application_object_targets` (
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL,
    `object_id` TEXT NOT NULL,
    `target_id` TEXT NOT NULL,

    FOREIGN KEY (`object_id`) REFERENCES `object_application_objects` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY (`target_id`) REFERENCES `object_application_targets` (`id`)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE UNIQUE INDEX `object_target_index`
ON `object_application_object_targets` (`object_id`, `target_id`);

CREATE TABLE IF NOT EXISTS `object_application_versions` (                                          
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL,                                                          
    `number` TEXT,                                                                                  
    `created_timestamp` TEXT NOT NULL,                                                              
    `commit` TEXT,                                                                                  
    `zip_hash` TEXT NOT NULL,                                                                       
    `object_id` TEXT NOT NULL,                                                                      

    FOREIGN KEY (`object_id`) REFERENCES `object_application_objects` (`id`)                        
        ON UPDATE CASCADE                                                                               
        ON DELETE CASCADE                                                                               
);

-- Repository application

CREATE TABLE IF NOT EXISTS `repository_application_repositories` (
    `id` TEXT PRIMARY KEY UNIQUE NOT NULL,
    `url` TEXT UNIQUE NOT NULL,
    `path` TEXT UNIQUE NOT NULL
);

