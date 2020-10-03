SET @OLD_UNIQUE_CHECKS = @@UNIQUE_CHECKS, UNIQUE_CHECKS = 0;
SET @OLD_FOREIGN_KEY_CHECKS = @@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS = 0;
SET @OLD_SQL_MODE = @@SQL_MODE, SQL_MODE =
        'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema meiling
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema meiling
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `meiling` DEFAULT CHARACTER SET utf8;
USE `meiling`;

-- -----------------------------------------------------
-- Table `meiling`.`user`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`user`;

CREATE TABLE IF NOT EXISTS `meiling`.`user`
(
    `id`            BINARY(36)  NOT NULL DEFAULT UUID(),
    `name`          VARCHAR(45) NOT NULL,
    `user_id`       VARCHAR(45) NOT NULL,
    `creation_date` DATETIME    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `image_url`     TEXT        NULL,
    `gender`        VARCHAR(12) NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`group`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`group`;

CREATE TABLE IF NOT EXISTS `meiling`.`group`
(
    `id`   BINARY(36)  NOT NULL DEFAULT UUID(),
    `name` VARCHAR(32) NOT NULL,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    UNIQUE INDEX `name_UNIQUE` (`name` ASC) VISIBLE
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`user_has_group`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`user_has_group`;

CREATE TABLE IF NOT EXISTS `meiling`.`user_has_group`
(
    `user`  BINARY(36) NOT NULL,
    `group` BINARY(36) NOT NULL,
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
DROP TABLE IF EXISTS `meiling`.`email`;

CREATE TABLE IF NOT EXISTS `meiling`.`email`
(
    `id`                BINARY(36)  NOT NULL DEFAULT UUID(),
    `email`             VARCHAR(64) NOT NULL,
    `user`              BINARY(36)  NOT NULL,
    `registration_date` DATETIME GENERATED ALWAYS AS (CURRENT_TIMESTAMP) VIRTUAL,
    `is_validated`      TINYINT     NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`, `user`),
    UNIQUE INDEX `email_UNIQUE` (`email` ASC) VISIBLE,
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
DROP TABLE IF EXISTS `meiling`.`phone_number`;

CREATE TABLE IF NOT EXISTS `meiling`.`phone_number`
(
    `id`              BINARY(36)  NOT NULL DEFAULT UUID(),
    `itu_code`        INT         NOT NULL,
    `domestic_number` VARCHAR(16) NOT NULL,
    `user`            BINARY(36)  NOT NULL,
    `is_validated`    TINYINT     NOT NULL DEFAULT 0,
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
-- Table `meiling`.`client`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`client`;

CREATE TABLE IF NOT EXISTS `meiling`.`client`
(
    `id`               BINARY(36)   NOT NULL DEFAULT UUID(),
    `name`             VARCHAR(45)  NOT NULL,
    `secret`           VARCHAR(256) NOT NULL,
    `author`           VARCHAR(32)  NULL,
    `contact`          VARCHAR(64)  NULL,
    `image_url`        TEXT         NULL,
    `owner`            BINARY(36)   NOT NULL,
    `privacy_policy`   TEXT         NULL,
    `terms_of_service` TEXT         NULL,
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
-- Table `meiling`.`user_accepted_client`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`user_accepted_client`;

CREATE TABLE IF NOT EXISTS `meiling`.`user_accepted_client`
(
    `client_id` BINARY(36) NOT NULL,
    `user_id`   BINARY(36) NOT NULL,
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
DROP TABLE IF EXISTS `meiling`.`refresh_token`;

CREATE TABLE IF NOT EXISTS `meiling`.`refresh_token`
(
    `id`         BINARY(36) NOT NULL DEFAULT UUID(),
    `token`      CHAR(128)  NOT NULL,
    `issue_date` DATETIME GENERATED ALWAYS AS (CURRENT_TIMESTAMP) VIRTUAL,
    `user`       BINARY(36) NOT NULL,
    `client_id`  BINARY(36) NOT NULL,
    PRIMARY KEY (`id`, `user`, `client_id`),
    UNIQUE INDEX `token_UNIQUE` (`token` ASC) VISIBLE,
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    INDEX `fk_refresh_token_user1_idx` (`user` ASC) VISIBLE,
    INDEX `fk_refresh_token_client1_idx` (`client_id` ASC) VISIBLE,
    CONSTRAINT `fk_refresh_token_user1`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_refresh_token_client1`
        FOREIGN KEY (`client_id`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`access_token`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`access_token`;

CREATE TABLE IF NOT EXISTS `meiling`.`access_token`
(
    `id`         BINARY(36) NOT NULL DEFAULT UUID(),
    `token`      CHAR(128)  NOT NULL,
    `issue_date` DATETIME GENERATED ALWAYS AS (CURRENT_TIMESTAMP) VIRTUAL,
    `user`       BINARY(36) NOT NULL,
    `client_id`  BINARY(36) NOT NULL,
    PRIMARY KEY (`id`, `user`, `client_id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    UNIQUE INDEX `token_UNIQUE` (`token` ASC) VISIBLE,
    INDEX `fk_access_token_user1_idx` (`user` ASC) VISIBLE,
    INDEX `fk_access_token_client1_idx` (`client_id` ASC) VISIBLE,
    CONSTRAINT `fk_access_token_user1`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_access_token_client1`
        FOREIGN KEY (`client_id`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`log`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`log`;

CREATE TABLE IF NOT EXISTS `meiling`.`log`
(
    `id`               BINARY(36)                                                                                 NOT NULL DEFAULT UUID(),
    `initiator_ip`     VARCHAR(40)                                                                                NOT NULL DEFAULT '0.0.0.0',
    `log`              VARCHAR(256)                                                                               NOT NULL,
    `log_type`         ENUM ('job_start', 'job_log', 'job_end', 'user_log', 'audit_log','debug_log','client_log') NOT NULL,
    `initiator_user`   BINARY(36)                                                                                 NULL,
    `initiator_client` BINARY(36)                                                                                 NULL,
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
DROP TABLE IF EXISTS `meiling`.`auth_info`;

CREATE TABLE IF NOT EXISTS `meiling`.`auth_info`
(
    `id`          BINARY(36)                                               NOT NULL DEFAULT UUID(),
    `auth_method` ENUM ('password', 'pubkey', 'one_time_password','fido2') NOT NULL,
    `key`         TEXT                                                     NOT NULL,
    `name`        VARCHAR(16)                                              NOT NULL,
    `user_id`     BINARY(36)                                               NOT NULL,
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
-- Table `meiling`.`meiling_policy`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`meiling_policy`;

CREATE TABLE IF NOT EXISTS `meiling`.`meiling_policy`
(
    `id`          BINARY(36)  NOT NULL DEFAULT UUID(),
    `name`        VARCHAR(64) NOT NULL,
    `description` TEXT        NULL,
    `url`         TEXT        NULL,
    `required`    TINYINT     NOT NULL DEFAULT 0,
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`user_policy_consent`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`user_policy_consent`;

CREATE TABLE IF NOT EXISTS `meiling`.`user_policy_consent`
(
    `policy_id` BINARY(36) NOT NULL,
    `user_id`   BINARY(36) NOT NULL,
    `consent`   TINYINT    NOT NULL DEFAULT 0,
    PRIMARY KEY (`policy_id`, `user_id`),
    INDEX `fk_user_policy_consent_user1_idx` (`user_id` ASC) VISIBLE,
    CONSTRAINT `fk_user_policy_consent_policy1`
        FOREIGN KEY (`policy_id`)
            REFERENCES `meiling`.`meiling_policy` (`id`)
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
-- Table `meiling`.`authorization_code`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`authorization_code`;

CREATE TABLE IF NOT EXISTS `meiling`.`authorization_code`
(
    `id`         BINARY(36) NOT NULL DEFAULT UUID(),
    `code`       CHAR(128)  NOT NULL,
    `issue_date` DATETIME GENERATED ALWAYS AS (CURRENT_TIMESTAMP) VIRTUAL,
    `user`       BINARY(36) NOT NULL,
    `client_id`  BINARY(36) NOT NULL,
    PRIMARY KEY (`id`, `user`, `client_id`),
    UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE,
    UNIQUE INDEX `token_UNIQUE` (`code` ASC) VISIBLE,
    INDEX `fk_access_token_user1_idx` (`user` ASC) VISIBLE,
    INDEX `fk_access_token_client1_idx` (`client_id` ASC) VISIBLE,
    CONSTRAINT `fk_access_token_user10`
        FOREIGN KEY (`user`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_access_token_client10`
        FOREIGN KEY (`client_id`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`scope`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`scope`;

CREATE TABLE IF NOT EXISTS `meiling`.`scope`
(
    `scope_id`    BINARY(36)   NOT NULL DEFAULT UUID(),
    `description` TEXT         NULL,
    `scope`       VARCHAR(255) NOT NULL,
    PRIMARY KEY (`scope_id`),
    UNIQUE INDEX `scope_id_UNIQUE` (`scope_id` ASC) VISIBLE,
    UNIQUE INDEX `scope_UNIQUE` (`scope` ASC) VISIBLE
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`client_scope_requirements`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`client_scope_requirements`;

CREATE TABLE IF NOT EXISTS `meiling`.`client_scope_requirements`
(
    `client_id` BINARY(36) NOT NULL,
    `scope_id`  BINARY(36) NOT NULL,
    PRIMARY KEY (`client_id`, `scope_id`),
    INDEX `fk_client_has_scope_scope1_idx` (`scope_id` ASC) VISIBLE,
    INDEX `fk_client_has_scope_client1_idx` (`client_id` ASC) VISIBLE,
    CONSTRAINT `fk_client_has_scope_client1`
        FOREIGN KEY (`client_id`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_client_has_scope_scope1`
        FOREIGN KEY (`scope_id`)
            REFERENCES `meiling`.`scope` (`scope_id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`client_scope_optional`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`client_scope_optional`;

CREATE TABLE IF NOT EXISTS `meiling`.`client_scope_optional`
(
    `client_id` BINARY(36) NOT NULL,
    `scope_id`  BINARY(36) NOT NULL,
    PRIMARY KEY (`client_id`, `scope_id`),
    INDEX `fk_client_has_scope_scope1_idx` (`scope_id` ASC) VISIBLE,
    INDEX `fk_client_has_scope_client1_idx` (`client_id` ASC) VISIBLE,
    CONSTRAINT `fk_client_has_scope_client10`
        FOREIGN KEY (`client_id`)
            REFERENCES `meiling`.`client` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_client_has_scope_scope10`
        FOREIGN KEY (`scope_id`)
            REFERENCES `meiling`.`scope` (`scope_id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`access_token_scope`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`access_token_scope`;

CREATE TABLE IF NOT EXISTS `meiling`.`access_token_scope`
(
    `access_token_id` BINARY(36) NOT NULL,
    `scope_scope_id`  BINARY(36) NOT NULL,
    PRIMARY KEY (`access_token_id`, `scope_scope_id`),
    INDEX `fk_access_token_has_scope_scope1_idx` (`scope_scope_id` ASC) VISIBLE,
    INDEX `fk_access_token_has_scope_access_token1_idx` (`access_token_id` ASC) VISIBLE,
    CONSTRAINT `fk_access_token_has_scope_access_token1`
        FOREIGN KEY (`access_token_id`)
            REFERENCES `meiling`.`access_token` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_access_token_has_scope_scope1`
        FOREIGN KEY (`scope_scope_id`)
            REFERENCES `meiling`.`scope` (`scope_id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`refresh_token_has_scope`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`refresh_token_has_scope`;

CREATE TABLE IF NOT EXISTS `meiling`.`refresh_token_has_scope`
(
    refresh_token_id BINARY(36) NOT NULL,
    scope_id         BINARY(36) NOT NULL,
    PRIMARY KEY (refresh_token_id, scope_id),
    INDEX `fk_refresh_token_has_scope_scope1_idx` (scope_id ASC) VISIBLE,
    INDEX `fk_refresh_token_has_scope_refresh_token1_idx` (refresh_token_id ASC) VISIBLE,
    CONSTRAINT `fk_refresh_token_has_scope_refresh_token1`
        FOREIGN KEY (refresh_token_id)
            REFERENCES `meiling`.`refresh_token` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_refresh_token_has_scope_scope1`
        FOREIGN KEY (scope_id)
            REFERENCES `meiling`.`scope` (`scope_id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`group_has_scope`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`group_has_scope`;

CREATE TABLE IF NOT EXISTS `meiling`.`group_has_scope`
(
    `group_id`       BINARY(36) NOT NULL,
    `scope_scope_id` BINARY(36) NOT NULL,
    PRIMARY KEY (`group_id`, `scope_scope_id`),
    INDEX `fk_group_has_scope_scope1_idx` (`scope_scope_id` ASC) VISIBLE,
    INDEX `fk_group_has_scope_group1_idx` (`group_id` ASC) VISIBLE,
    CONSTRAINT `fk_group_has_scope_group1`
        FOREIGN KEY (`group_id`)
            REFERENCES `meiling`.`group` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_group_has_scope_scope1`
        FOREIGN KEY (`scope_scope_id`)
            REFERENCES `meiling`.`scope` (`scope_id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`authorization_code_has_scope`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`authorization_code_has_scope`;

CREATE TABLE IF NOT EXISTS `meiling`.`authorization_code_has_scope`
(
    `authorization_code_id` BINARY(36) NOT NULL,
    `scope_scope_id`        BINARY(36) NOT NULL,
    PRIMARY KEY (`authorization_code_id`, `scope_scope_id`),
    INDEX `fk_authorization_code_has_scope_scope1_idx` (`scope_scope_id` ASC) VISIBLE,
    INDEX `fk_authorization_code_has_scope_authorization_code1_idx` (`authorization_code_id` ASC) VISIBLE,
    CONSTRAINT `fk_authorization_code_has_scope_authorization_code1`
        FOREIGN KEY (`authorization_code_id`)
            REFERENCES `meiling`.`authorization_code` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_authorization_code_has_scope_scope1`
        FOREIGN KEY (`scope_scope_id`)
            REFERENCES `meiling`.`scope` (`scope_id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;


-- -----------------------------------------------------
-- Table `meiling`.`user_has_scope`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`user_has_scope`;

CREATE TABLE IF NOT EXISTS `meiling`.`user_has_scope`
(
    `user_id`        BINARY(36) NOT NULL,
    `scope_scope_id` BINARY(36) NOT NULL,
    PRIMARY KEY (`user_id`, `scope_scope_id`),
    INDEX `fk_user_has_scope_scope1_idx` (`scope_scope_id` ASC) VISIBLE,
    INDEX `fk_user_has_scope_user1_idx` (`user_id` ASC) VISIBLE,
    CONSTRAINT `fk_user_has_scope_user1`
        FOREIGN KEY (`user_id`)
            REFERENCES `meiling`.`user` (`id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION,
    CONSTRAINT `fk_user_has_scope_scope1`
        FOREIGN KEY (`scope_scope_id`)
            REFERENCES `meiling`.`scope` (`scope_id`)
            ON DELETE NO ACTION
            ON UPDATE NO ACTION
)
    ENGINE = InnoDB;

USE `meiling`;

-- -----------------------------------------------------
-- Placeholder table for view `meiling`.`user_emails`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `meiling`.`user_emails`
(
    `id` INT
);

-- -----------------------------------------------------
-- View `meiling`.`user_emails`
-- -----------------------------------------------------
DROP TABLE IF EXISTS `meiling`.`user_emails`;
DROP VIEW IF EXISTS `meiling`.`user_emails`;
USE `meiling`;
CREATE OR REPLACE VIEW `user_emails` AS
(
SELECT user.id AS user_id, name, creation_date, email
FROM user
         JOIN email ON user.id = email.user);

SET SQL_MODE = @OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS = @OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS = @OLD_UNIQUE_CHECKS;
