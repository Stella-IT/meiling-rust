use chrono::NaiveDateTime;

use crate::database::enums::{AuthenticationMethod, LogType};

use super::schema::*;

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "access_token"]
pub struct AccessToken {
    pub id: Vec<u8>,
    pub token: String,
    pub issue_date: NaiveDateTime,
    pub user: Vec<u8>,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "auth_info"]
pub struct AuthenticationInfo {
    pub id: Vec<u8>,
    pub auth_method: AuthenticationMethod,
    pub key: String,
    pub name: String,
    pub user_id: Vec<u8>,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "client"]
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

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "email"]
pub struct Email {
    pub id: Vec<u8>,
    pub address: String,
    pub user: Vec<u8>,
    pub registration_date: Option<NaiveDateTime>,
    pub is_validated: i8,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "group"]
pub struct Group {
    pub id: Vec<u8>,
    pub name: String,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "log"]
pub struct Log {
    pub id: Vec<u8>,
    pub initiator_ip: String,
    pub data: String,
    pub log_type: LogType,
    pub initiator_user: Vec<u8>,
    pub initiator_client: Vec<u8>,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "permission"]
pub struct Permission {
    pub id: Vec<u8>,
    pub name: String,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "permission_group"]
pub struct PermissionGroup {
    pub id: Vec<u8>,
    pub name: String,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "phone_number"]
pub struct PhoneNumber {
    pub id: Vec<u8>,
    pub itu_code: i32,
    pub domestic_number: String,
    pub user: Vec<u8>,
    pub is_validated: i8,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "policy"]
pub struct Policy {
    pub id: Vec<u8>,
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub required: i8,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "refresh_token"]
pub struct RefreshToken {
    pub id: Vec<u8>,
    pub token: String,
    pub issue_date: Option<NaiveDateTime>,
    pub user: Vec<u8>,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "user"]
pub struct User {
    pub id: Vec<u8>,
    pub name: String,
    pub creation_date: NaiveDateTime,
    pub image_url: Option<String>,
    pub gender: Option<String>,
}

#[derive(Insertable, Queryable, Debug, Clone, PartialEq)]
#[table_name = "user_policy_consent"]
pub struct UserPolicyConsent {
    pub policy_id: Vec<u8>,
    pub user_id: Vec<u8>,
    pub consent: i8,
}
