use chrono::NaiveDateTime;

use crate::database::database_enums::{AuthenticationMethod, LogType};

use super::schema::*;

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(User)]
#[table_name = "access_token"]
pub struct AccessToken {
    pub id: Vec<u8>,
    pub token: String,
    pub issue_date: NaiveDateTime,
    pub user_id: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "access_token"]
pub struct NewAccessToken {
    pub token: String,
    pub issue_date: NaiveDateTime,
    pub user_id: Vec<u8>,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct AuthenticationInfo {
    pub id: Vec<u8>,
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "auth_info"]
pub struct NewAuthenticationInfo {
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: Vec<u8>,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct Client {
    pub id: Vec<u8>,
    pub name: String,
    pub secret: String,
    pub author: Option<String>,
    pub contact: Option<String>,
    pub image_url: Option<String>,
    pub owner: Vec<u8>,
    pub privacy_policy: Option<String>,
}

#[derive(Insertable)]
#[table_name = "client"]
pub struct NewClient {
    pub name: String,
    pub secret: String,
    pub author: Option<String>,
    pub contact: Option<String>,
    pub image_url: Option<String>,
    pub owner: Vec<u8>,
    pub privacy_policy: Option<String>,
}

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(Client)]
#[belongs_to(PermissionGroup)]
#[table_name = "client_permission_requirement"]
pub struct ClientPermissionRequirement {
    pub client_id: Vec<u8>,
    pub permission_group_id: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "client_permission_requirement"]
pub struct NewClientPermissionRequirement {
    pub client_id: Vec<u8>,
    pub permission_group_id: Vec<u8>,
}

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(User)]
#[table_name = "email"]
pub struct Email {
    pub id: Vec<u8>,
    pub address: String,
    pub user_id: Vec<u8>,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: i8,
}

#[derive(Insertable)]
#[table_name = "email"]
pub struct NewEmail {
    pub address: String,
    pub user_id: Vec<u8>,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: i8,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct Group {
    pub id: Vec<u8>,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "group"]
pub struct NewGroup {
    pub name: String,
}

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(Group)]
#[belongs_to(PermissionGroup)]
#[table_name = "group_has_permission_group"]
pub struct GroupHasPermissionGroup {
    pub group_id: Vec<u8>,
    pub permission_group_id: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "group_has_permission_group"]
pub struct NewGroupHasPermissionGroup {
    pub group_id: Vec<u8>,
    pub permission_group_id: Vec<u8>,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct Log {
    pub id: Vec<u8>,
    pub initiator_ip: String,
    pub data: String,
    pub log_type: LogType,
    pub initiator_user: Vec<u8>,
    pub initiator_client: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "log"]
pub struct NewLog {
    pub initiator_ip: String,
    pub data: String,
    pub log_type: LogType,
    pub initiator_user: Vec<u8>,
    pub initiator_client: Vec<u8>,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct Permission {
    pub id: Vec<u8>,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "permission"]
pub struct NewPermission {
    pub name: String,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct PermissionGroup {
    pub id: Vec<u8>,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "permission_group"]
pub struct NewPermissionGroup {
    pub name: String,
}

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(Permission)]
#[belongs_to(PermissionGroup)]
#[table_name = "permission_group_has_permission"]
pub struct PermissionGroupHasPermission {
    pub permission_id: Vec<u8>,
    pub permission_group_id: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "permission_group_has_permission"]
pub struct NewPermissionGroupHasPermission {
    pub permission_id: Vec<u8>,
    pub permission_group_id: Vec<u8>,
}

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(User)]
#[table_name = "phone_number"]
pub struct PhoneNumber {
    pub id: Vec<u8>,
    pub itu_code: i32,
    pub domestic_number: String,
    pub user_id: Vec<u8>,
    pub is_validated: i8,
}

#[derive(Insertable)]
#[table_name = "phone_number"]
pub struct NewPhoneNumber {
    pub itu_code: i32,
    pub domestic_number: String,
    pub user_id: Vec<u8>,
    pub is_validated: i8,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct Policy {
    pub id: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub required: i8,
}

#[derive(Insertable)]
#[table_name = "policy"]
pub struct NewPolicy {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub required: i8,
}

#[derive(Queryable, Debug, Clone, PartialEq)]
pub struct RefreshToken {
    pub id: Vec<u8>,
    pub token: String,
    pub issue_date: Option<NaiveDateTime>,
    pub user: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "refresh_token"]
pub struct NewRefreshToken {
    pub token: String,
    pub issue_date: Option<NaiveDateTime>,
    pub user: Vec<u8>,
}

#[derive(Identifiable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "user"]
pub struct User {
    pub id: Vec<u8>,
    pub name: String,
    pub user_id: String,
    pub creation_date: NaiveDateTime,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "user"]
pub struct NewUser {
    pub name: String,
    pub user_id: String,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(Client)]
#[belongs_to(User)]
#[table_name = "user_accepted_client"]
pub struct UserAcceptedClient {
    pub client_id: Vec<u8>,
    pub user_id: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "user_accepted_client"]
pub struct NewUserAcceptedClient {
    pub client_id: Vec<u8>,
    pub user_id: Vec<u8>,
}

#[derive(Associations, Queryable, Debug, Clone, PartialEq)]
#[belongs_to(User)]
#[belongs_to(Policy)]
#[table_name = "user_policy_consent"]
pub struct UserPolicyConsent {
    pub policy_id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub consent: i8,
}

#[derive(Insertable)]
#[table_name = "user_policy_consent"]
pub struct NewUserPolicyConsent {
    pub policy_id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub consent: i8,
}

#[derive(Associations, Queryable)]
#[belongs_to(User)]
#[belongs_to(Group)]
#[table_name = "user_has_group"]
pub struct UserHasGroup {
    pub user_id: Vec<u8>,
    pub group_id: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "user_has_group"]
pub struct NewUserHasGroup {
    pub user_id: Vec<u8>,
    pub group_id: Vec<u8>,
}
