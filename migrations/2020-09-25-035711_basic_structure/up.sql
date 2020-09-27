-- MySQL Script generated by MySQL Workbench
-- Fri Sep 25 13:00:27 2020
-- Model: New Model    Version: 1.0
-- MySQL Workbench Forward Engineering
SET @OLD_UNIQUE_CHECKS = @@UNIQUE_CHECKS, UNIQUE_CHECKS = 0;
SET @OLD_FOREIGN_KEY_CHECKS = @@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS = 0;
SET @OLD_SQL_MODE = @@SQL_MODE, SQL_MODE =
        'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';
-- -----------------------------------------------------
-- Schema meiling
-- -----------------------------------------------------
USE `meiling`;
-- -----------------------------------------------------
-- Table `meiling`.`user`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`user`
(
    `id`            VARBINARY(36) NOT NULL DEFAULT UUID(),
    `user_id`       VARCHAR(128)  NOT NULL UNIQUE,
    `name`          VARCHAR(128)  NOT NULL,
    `creation_date` DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `image_url`     TEXT          NULL,
    `gender`        VARCHAR(64)   NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`Group`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`group`
(
    `id`   VARBINARY(36) NOT NULL DEFAULT UUID(),
    `name` VARCHAR(32)   NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`user_has_group`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`user_has_group`
(
    `user`  VARBINARY(36) NOT NULL,
    `group` VARBINARY(36) NOT NULL,
    PRIMARY KEY (`user`, `group`),
    INDEX `fk_user_has_group_group1_idx` (`group` ASC) VISIBLE,
    INDEX `fk_user_has_group_user1_idx` (`user` ASC) VISIBLE,
    CONSTRAINT `fk_user_has_group_user1`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_user_has_group_group1`
        FOREIGN KEY (`group`)
            REFERENCES `meiling`.`group` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`email`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`email`
(
    `id`                VARBINARY(36) NOT NULL DEFAULT UUID(),
    address             VARCHAR(64)   NOT NULL,
    `user`              VARBINARY(36) NOT NULL,
    `registration_date` DATETIME GENERATED ALWAYS AS (CURRENT_TIMESTAMP) VIRTUAL,
    `is_validated`      TINYINT       NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`, `user`),
    UNIQUE INDEX `email_UNIQUE` (address ASC) VISIBLE,
    INDEX `fk_email_user1_idx` (`user` ASC) VISIBLE,
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    CONSTRAINT `fk_email_user1`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`phone_number`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`phone_number`
(
    `id`              VARBINARY(36) NOT NULL DEFAULT UUID(),
    `itu_code`        INT           NOT NULL,
    `domestic_number` VARCHAR(16)   NOT NULL,
    `user`            VARBINARY(36) NOT NULL,
    `is_validated`    TINYINT       NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`, `user`),
    INDEX `fk_phone_number_user1_idx` (`user` ASC) VISIBLE,
    CONSTRAINT `fk_phone_number_user1`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`permission`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`permission`
(
    `id`   VARBINARY(36) NOT NULL DEFAULT UUID(),
    `name` VARCHAR(45)   NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`permission_group`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`permission_group`
(
    `id`   VARBINARY(36) NOT NULL DEFAULT UUID(),
    `name` VARCHAR(45)   NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`permission_group_has_permission`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`permission_group_has_permission`
(
    `permission`       VARBINARY(36) NOT NULL,
    `permission_group` VARBINARY(36) NOT NULL,
    PRIMARY KEY (`permission`, `permission_group`),
    INDEX `fk_permission_has_permission_group_permission_group1_idx` (`permission_group` ASC) VISIBLE,
    INDEX `fk_permission_has_permission_group_permission1_idx` (`permission` ASC) VISIBLE,
    CONSTRAINT `fk_permission_has_permission_group_permission1`
        FOREIGN KEY (`permission`)
            REFERENCES `meiling`.`permission` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_permission_has_permission_group_permission_group1`
        FOREIGN KEY (`permission_group`)
            REFERENCES `meiling`.`permission_group` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`group_has_permission_group`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`group_has_permission_group`
(
    `permission_group` VARBINARY(36) NOT NULL,
    `group`            VARBINARY(36) NOT NULL,
    PRIMARY KEY (`permission_group`, `group`),
    INDEX `fk_permission_group_has_group_group1_idx` (`group` ASC) VISIBLE,
    INDEX `fk_permission_group_has_group_permission_group1_idx` (`permission_group` ASC) VISIBLE,
    CONSTRAINT `fk_permission_group_has_group_permission_group1`
        FOREIGN KEY (`permission_group`)
            REFERENCES `meiling`.`permission_group` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_permission_group_has_group_group1`
        FOREIGN KEY (`group`)
            REFERENCES `meiling`.`group` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`user_has_permission_group`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`user_has_permission_group`
(
    `permission_group_id` VARBINARY(36) NOT NULL,
    `user_id`             VARBINARY(36) NOT NULL,
    PRIMARY KEY (`permission_group_id`, `user_id`),
    INDEX `fk_permission_group_has_user_user1_idx` (`user_id` ASC) VISIBLE,
    INDEX `fk_permission_group_has_user_permission_group1_idx` (`permission_group_id` ASC) VISIBLE,
    CONSTRAINT `fk_permission_group_has_user_permission_group1`
        FOREIGN KEY (`permission_group_id`)
            REFERENCES `meiling`.`permission_group` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_permission_group_has_user_user1`
        FOREIGN KEY (`user_id`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`client`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`client`
(
    `id`             VARBINARY(36) NOT NULL DEFAULT UUID(),
    `name`           VARCHAR(45)   NOT NULL,
    `secret`         VARCHAR(256)  NOT NULL,
    `author`         VARCHAR(32)   NULL,
    `contact`        VARCHAR(64)   NULL,
    `image_url`      TEXT          NULL,
    `owner`          VARBINARY(36) NOT NULL,
    `privacy_policy` TEXT          NULL,
    PRIMARY KEY (`id`, `owner`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    INDEX `fk_client_user1_idx` (`owner` ASC) VISIBLE,
    CONSTRAINT `fk_client_user1`
        FOREIGN KEY (`owner`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`client_permission_requirement`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`client_permission_requirement`
(
    `client`           VARBINARY(36) NOT NULL,
    `permission_group` VARBINARY(36) NOT NULL,
    PRIMARY KEY (`client`, `permission_group`),
    INDEX `fk_client_has_permission_group_permission_group1_idx` (`permission_group` ASC) VISIBLE,
    INDEX `fk_client_has_permission_group_client1_idx` (`client` ASC) VISIBLE,
    CONSTRAINT `fk_client_has_permission_group_client1`
        FOREIGN KEY (`client`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_client_has_permission_group_permission_group1`
        FOREIGN KEY (`permission_group`)
            REFERENCES `meiling`.`permission_group` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`user_accepted_client`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`user_accepted_client`
(
    `client_id` VARBINARY(36) NOT NULL,
    `user_id`   VARBINARY(36) NOT NULL,
    PRIMARY KEY (`client_id`, `user_id`),
    INDEX `fk_client_has_user_user1_idx` (`user_id` ASC) VISIBLE,
    INDEX `fk_client_has_user_client1_idx` (`client_id` ASC) VISIBLE,
    CONSTRAINT `fk_client_has_user_client1`
        FOREIGN KEY (`client_id`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_client_has_user_user1`
        FOREIGN KEY (`user_id`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`refresh_token`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`refresh_token`
(
    `id`       VARBINARY(36) NOT NULL DEFAULT UUID(),
    `token`    CHAR(128)     NOT NULL,
    issue_date DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `user`     VARBINARY(36) NOT NULL,
    PRIMARY KEY (`id`, `user`),
    UNIQUE INDEX `token_UNIQUE` (`token` ASC) VISIBLE,
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    INDEX `fk_refresh_token_user1_idx` (`user` ASC) VISIBLE,
    CONSTRAINT `fk_refresh_token_user1`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`access_token`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`access_token`
(
    `id`       VARBINARY(36) NOT NULL DEFAULT UUID(),
    `token`    CHAR(128)     NOT NULL,
    issue_date DATETIME      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `user`     VARBINARY(36) NOT NULL,
    PRIMARY KEY (`id`, `user`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    UNIQUE INDEX `token_UNIQUE` (`token` ASC) VISIBLE,
    INDEX `fk_access_token_user1_idx` (`user` ASC) VISIBLE,
    CONSTRAINT `fk_access_token_user1`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`log`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`log`
(
    `id`               VARBINARY(36)                                                     NOT NULL DEFAULT UUID(),
    `initiator_ip`     VARCHAR(40)                                                       NOT NULL DEFAULT '0.0.0.0',
    data               TEXT                                                              NOT NULL,
    `type`             ENUM ('job_start', 'job_log', 'job_end', 'user_log', 'audit_log') NOT NULL,
    `initiator_user`   VARBINARY(36)                                                     NULL,
    `initiator_client` VARBINARY(36)                                                     NULL,
    PRIMARY KEY (`id`, `initiator_user`, `initiator_client`),
    INDEX `fk_log_user1_idx` (`initiator_user` ASC) VISIBLE,
    INDEX `fk_log_client1_idx` (`initiator_client` ASC) VISIBLE,
    CONSTRAINT `fk_log_user1`
        FOREIGN KEY (`initiator_user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_log_client1`
        FOREIGN KEY (`initiator_client`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`auth_info`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`auth_info`
(
    `id`          VARBINARY(36)                                    NOT NULL DEFAULT UUID(),
    `auth_method` ENUM ('password', 'pubkey', 'one_time_password') NOT NULL,
    `key`         TEXT                                             NOT NULL,
    `name`        VARCHAR(16)                                      NOT NULL,
    `user_id`     VARBINARY(36)                                    NOT NULL,
    PRIMARY KEY (`id`, `user_id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    INDEX `fk_auth_info_user1_idx` (`user_id` ASC) VISIBLE,
    CONSTRAINT `fk_auth_info_user1`
        FOREIGN KEY (`user_id`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`policy`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`policy`
(
    `id`          VARBINARY(36) NOT NULL DEFAULT UUID(),
    `name`        VARCHAR(64)   NOT NULL,
    `description` TEXT          NULL,
    `url`         TEXT          NULL,
    `required`    TINYINT       NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- Table `meiling`.`user_policy_consent`
-- -----------------------------------------------------

CREATE TABLE `meiling`.`user_policy_consent`
(
    `policy_id` VARBINARY(36) NOT NULL,
    `user_id`   VARBINARY(36) NOT NULL,
    `consent`   TINYINT       NOT NULL DEFAULT 0,
    PRIMARY KEY (`policy_id`, `user_id`),
    INDEX `fk_user_policy_consent_user1_idx` (`user_id` ASC) VISIBLE,
    CONSTRAINT `fk_user_policy_consent_policy1`
        FOREIGN KEY (`policy_id`)
            REFERENCES `meiling`.`policy` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_user_policy_consent_user1`
        FOREIGN KEY (`user_id`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

-- -----------------------------------------------------
-- View `meiling`.`user_emails`
-- -----------------------------------------------------

CREATE VIEW `user_emails` AS
(
SELECT user.id AS user_id, name, creation_date, address
FROM user
         JOIN email ON user.id = email.user);

SET SQL_MODE = @OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS = @OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS = @OLD_UNIQUE_CHECKS;
