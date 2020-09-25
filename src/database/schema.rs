table! {
    access_token (id, user) {
        id -> Char,
        token -> Char,
        issue_date -> Nullable<Datetime>,
        user -> Char,
    }
}

table! {
    use crate::database::enums::AuthenticationMethodMapping;
    use diesel::sql_types::{Char, Text, Varchar};
    auth_info (id, user_id) {
        id -> Char,
        auth_method -> AuthenticationMethodMapping,
        key -> Text,
        name -> Varchar,
        user_id -> Char,
    }
}

table! {
    client (id, owner) {
        id -> Char,
        name -> Varchar,
        secret -> Varchar,
        author -> Nullable<Varchar>,
        contact -> Nullable<Varchar>,
        image_url -> Nullable<Text>,
        owner -> Char,
        privacy_policy -> Nullable<Text>,
    }
}

table! {
    client_permission_requirement (client, permission_group) {
        client -> Char,
        permission_group -> Char,
    }
}

table! {
    email (id, user) {
        id -> Char,
        address -> Varchar,
        user -> Char,
        registration_date -> Nullable<Datetime>,
        is_validated -> Tinyint,
    }
}

table! {
    group (id) {
        id -> Char,
        name -> Varchar,
    }
}

table! {
    group_has_permission_group (permission_group, group) {
        permission_group -> Char,
        group -> Char,
    }
}

table! {
    use crate::database::enums::LogTypeMapping;
    use diesel::sql_types::{Char, Text, Varchar};
    log (id, initiator_user, initiator_client) {
        id -> Char,
        initiator_ip -> Varchar,
        data -> Text,
        #[sql_name = "type"]
        log_type -> LogTypeMapping,
        initiator_user -> Char,
        initiator_client -> Char,
    }
}

table! {
    permission (id) {
        id -> Char,
        name -> Varchar,
    }
}

table! {
    permission_group (id) {
        id -> Char,
        name -> Varchar,
    }
}

table! {
    permission_group_has_permission (permission, permission_group) {
        permission -> Char,
        permission_group -> Char,
    }
}

table! {
    phone_number (id, user) {
        id -> Char,
        itu_code -> Integer,
        domestic_number -> Varchar,
        user -> Char,
        is_validated -> Tinyint,
    }
}

table! {
    policy (id) {
        id -> Char,
        name -> Varchar,
        description -> Nullable<Text>,
        url -> Nullable<Text>,
        required -> Tinyint,
    }
}

table! {
    refresh_token (id, user) {
        id -> Char,
        token -> Char,
        issue_date -> Nullable<Datetime>,
        user -> Char,
    }
}

table! {
    user (id) {
        id -> Char,
        name -> Varchar,
        creation_date -> Datetime,
        image_url -> Nullable<Text>,
        gender -> Nullable<Varchar>,
    }
}

table! {
    user_accepted_client (client_id, user_id) {
        client_id -> Char,
        user_id -> Char,
    }
}

table! {
    user_has_group (user, group) {
        user -> Char,
        group -> Char,
    }
}

table! {
    user_has_permission_group (permission_group_id, user_id) {
        permission_group_id -> Char,
        user_id -> Char,
    }
}

table! {
    user_policy_consent (policy_id, user_id) {
        policy_id -> Char,
        user_id -> Char,
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
