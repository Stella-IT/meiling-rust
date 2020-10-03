SET @OLD_UNIQUE_CHECKS = @@UNIQUE_CHECKS, UNIQUE_CHECKS = 0;
SET @OLD_FOREIGN_KEY_CHECKS = @@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS = 0;
SET @OLD_SQL_MODE = @@SQL_MODE, SQL_MODE =
        'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

DROP VIEW IF EXISTS `meiling`.`user_emails`;
DROP TABLE IF EXISTS `meiling`.`user_has_scope`;
DROP TABLE IF EXISTS `meiling`.`authorization_code_has_scope`;
DROP TABLE IF EXISTS `meiling`.`group_has_scope`;
DROP TABLE IF EXISTS `meiling`.`refresh_token_has_scope`;
DROP TABLE IF EXISTS `meiling`.`access_token_scope`;
DROP TABLE IF EXISTS `meiling`.`client_scope_optional`;
DROP TABLE IF EXISTS `meiling`.`client_scope_requirements`;
DROP TABLE IF EXISTS `meiling`.`scope`;
DROP TABLE IF EXISTS `meiling`.`authorization_code`;
DROP TABLE IF EXISTS `meiling`.`user_policy_consent`;
DROP TABLE IF EXISTS `meiling`.`meiling_policy`;
DROP TABLE IF EXISTS `meiling`.`auth_info`;
DROP TABLE IF EXISTS `meiling`.`log`;
DROP TABLE IF EXISTS `meiling`.`access_token`;
DROP TABLE IF EXISTS `meiling`.`refresh_token`;
DROP TABLE IF EXISTS `meiling`.`user_accepted_client`;
DROP TABLE IF EXISTS `meiling`.`client`;
DROP TABLE IF EXISTS `meiling`.`phone_number`;
DROP TABLE IF EXISTS `meiling`.`email`;
DROP TABLE IF EXISTS `meiling`.`user_has_group`;
DROP TABLE IF EXISTS `meiling`.`group`;
DROP TABLE IF EXISTS `meiling`.`user`;

SET SQL_MODE = @OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS = @OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS = @OLD_UNIQUE_CHECKS;
