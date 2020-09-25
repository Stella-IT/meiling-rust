SET @OLD_UNIQUE_CHECKS = @@UNIQUE_CHECKS, UNIQUE_CHECKS = 0;
SET @OLD_FOREIGN_KEY_CHECKS = @@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS = 0;
SET @OLD_SQL_MODE = @@SQL_MODE, SQL_MODE =
        'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

use `meiling`;

DROP VIEW `meiling`.`user_emails`;
DROP TABLE `meiling`.`user_policy_consent`;
DROP TABLE `meiling`.`policy`;
DROP TABLE `meiling`.`auth_info`;
DROP TABLE `meiling`.`log`;
DROP TABLE `meiling`.`access_token`;
DROP TABLE `meiling`.`refresh_token`;
DROP TABLE `meiling`.`user_accepted_client`;
DROP TABLE `meiling`.`client_permission_requirement`;
DROP TABLE `meiling`.`client`;
DROP TABLE `meiling`.`user_has_permission_group`;
DROP TABLE `meiling`.`group_has_permission_group`;
DROP TABLE `meiling`.`permission_group_has_permission`;
DROP TABLE `meiling`.`permission_group`;
DROP TABLE `meiling`.`permission`;
DROP TABLE `meiling`.`phone_number`;
DROP TABLE `meiling`.`email`;
DROP TABLE `meiling`.`user_has_group`;
DROP TABLE `meiling`.`group`;
DROP TABLE `meiling`.`user`;

SET SQL_MODE = @OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS = @OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS = @OLD_UNIQUE_CHECKS;
