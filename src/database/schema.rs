table! {
    access_token (id, user) {
        id -> Binary,
        token -> Char,
        issue_date -> Datetime,
        user -> Binary,
    }
}

table! {
    use crate::database::enums::AuthenticationMethodMapping;
    use diesel::sql_types::{Binary, Text, Varchar};
    auth_info (id, user_id) {
        id -> Binary,
        auth_method -> AuthenticationMethodMapping,
        key -> Text,
        name -> Varchar,
        user_id -> Binary,
    }
}

table! {
    client (id, owner) {
        id -> Binary,
        name -> Varchar,
        secret -> Varchar,
        author -> Nullable<Varchar>,
        contact -> Nullable<Varchar>,
        image_url -> Nullable<Text>,
        owner -> Binary,
        privacy_policy -> Nullable<Text>,
    }
}

table! {
    client_permission_requirement (client, permission_group) {
        client -> Binary,
        permission_group -> Binary,
    }
}

table! {
    email (id, user) {
        id -> Binary,
        address -> Varchar,
        user -> Binary,
        registration_date -> Nullable<Datetime>,
        is_validated -> Tinyint,
    }
}

table! {
    group (id) {
        id -> Binary,
        name -> Varchar,
    }
}

table! {
    group_has_permission_group (permission_group, group) {
        permission_group -> Binary,
        group -> Binary,
    }
}

table! {
    use crate::database::enums::LogTypeMapping;
    use diesel::sql_types::{Binary, Text, Varchar};
    log (id, initiator_user, initiator_client) {
        id -> Binary,
        initiator_ip -> Varchar,
        data -> Text,
        #[sql_name = "type"]
        log_type -> LogTypeMapping,
        initiator_user -> Binary,
        initiator_client -> Binary,
    }
}

table! {
    permission (id) {
        id -> Binary,
        name -> Varchar,
    }
}

table! {
    permission_group (id) {
        id -> Binary,
        name -> Varchar,
    }
}

table! {
    permission_group_has_permission (permission, permission_group) {
        permission -> Binary,
        permission_group -> Binary,
    }
}

table! {
    phone_number (id, user) {
        id -> Binary,
        itu_code -> Integer,
        domestic_number -> Varchar,
        user -> Binary,
        is_validated -> Tinyint,
    }
}

table! {
    policy (id) {
        id -> Binary,
        name -> Varchar,
        description -> Nullable<Text>,
        url -> Nullable<Text>,
        required -> Tinyint,
    }
}

table! {
    refresh_token (id, user) {
        id -> Binary,
        token -> Char,
        issue_date -> Datetime,
        user -> Binary,
    }
}

table! {
    user (id) {
        id -> Binary,
        name -> Varchar,
        creation_date -> Datetime,
        image_url -> Nullable<Text>,
        gender -> Nullable<Varchar>,
    }
}

table! {
    user_accepted_client (client_id, user_id) {
        client_id -> Binary,
        user_id -> Binary,
    }
}

table! {
    user_has_group (user, group) {
        user -> Binary,
        group -> Binary,
    }
}

table! {
    user_has_permission_group (permission_group_id, user_id) {
        permission_group_id -> Binary,
        user_id -> Binary,
    }
}

table! {
    user_policy_consent (policy_id, user_id) {
        policy_id -> Binary,
        user_id -> Binary,
        consent -> Tinyint,
    }
}

joinable!(access_token -> user (user));
joinable!(auth_info -> user (user_id));
joinable!(client -> user (owner));
joinable!(client_permission_requirement -> permission_group (permission_group));
joinable!(email -> user (user));
joinable!(group_has_permission_group -> group (group));
joinable!(group_has_permission_group -> permission_group (permission_group));
joinable!(log -> user (initiator_user));
joinable!(permission_group_has_permission -> permission (permission));
joinable!(permission_group_has_permission -> permission_group (permission_group));
joinable!(phone_number -> user (user));
joinable!(refresh_token -> user (user));
joinable!(user_accepted_client -> user (user_id));
joinable!(user_has_group -> group (group));
joinable!(user_has_group -> user (user));
joinable!(user_has_permission_group -> permission_group (permission_group_id));
joinable!(user_has_permission_group -> user (user_id));
joinable!(user_policy_consent -> policy (policy_id));
joinable!(user_policy_consent -> user (user_id));

allow_tables_to_appear_in_same_query!(
    access_token,
    auth_info,
    client,
    client_permission_requirement,
    email,
    group,
    group_has_permission_group,
    log,
    permission,
    permission_group,
    permission_group_has_permission,
    phone_number,
    policy,
    refresh_token,
    user,
    user_accepted_client,
    user_has_group,
    user_has_permission_group,
    user_policy_consent,
);
