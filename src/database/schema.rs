table! {
    use diesel::sql_types::*;

    access_token (id, user_id, client_id) {
        id -> Binary,
        token -> Char,
        issue_date -> Nullable<Datetime>,
        #[sql_name="user"]
        user_id -> Binary,
        client_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;

    access_token_scope (access_token_id, scope_scope_id) {
        access_token_id -> Binary,
        scope_scope_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;

    authorization_code (id, user, client_id) {
        id -> Binary,
        code -> Char,
        issue_date -> Nullable<Datetime>,
        user -> Binary,
        client_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;

    authorization_code_has_scope (authorization_code_id, scope_scope_id) {
        authorization_code_id -> Binary,
        scope_scope_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    auth_info (id, user_id) {
        id -> Binary,
        auth_method -> AuthenticationMethodMapping,
        key -> Text,
        name -> Varchar,
        user_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;

    client (id, owner) {
        id -> Binary,
        name -> Varchar,
        secret -> Varchar,
        author -> Nullable<Varchar>,
        contact -> Nullable<Varchar>,
        image_url -> Nullable<Text>,
        owner -> Binary,
        privacy_policy -> Nullable<Text>,
        terms_of_service -> Nullable<Text>,
    }
}

table! {
    use diesel::sql_types::*;

    client_scope_optional (client_id, scope_id) {
        client_id -> Binary,
        scope_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;

    client_scope_requirements (client_id, scope_id) {
        client_id -> Binary,
        scope_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;

    email (id, user_id) {
        id -> Binary,
        #[sql_name="email"]
        address -> Varchar,
        #[sql_name="user"]
        user_id -> Binary,
        registration_date -> Nullable<Datetime>,
        is_validated -> Tinyint,
    }
}

table! {
    use diesel::sql_types::*;

    group (id) {
        id -> Binary,
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;

    group_has_scope (group_id, scope_scope_id) {
        group_id -> Binary,
        scope_scope_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    log (id, initiator_user, initiator_client) {
        id -> Binary,
        initiator_ip -> Varchar,
        #[sql_name="log"]
        body -> Varchar,
        log_type -> LogTypeMapping,
        initiator_user -> Binary,
        initiator_client -> Binary,
    }
}

table! {
    use diesel::sql_types::*;

    meiling_policy (id) {
        id -> Binary,
        name -> Varchar,
        description -> Nullable<Text>,
        url -> Nullable<Text>,
        required -> Tinyint,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    phone_number (id, user_id) {
        id -> Binary,
        itu_code -> Integer,
        domestic_number -> Varchar,
        #[sql_name="user"]
        user_id -> Binary,
        is_validated -> Tinyint,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    refresh_token (id, user, client_id) {
        id -> Binary,
        token -> Char,
        issue_date -> Nullable<Datetime>,
        user -> Binary,
        client_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    refresh_token_has_scope (refresh_token_id, scope_id) {
        refresh_token_id -> Binary,
        scope_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    scope (scope_id) {
        scope_id -> Binary,
        description -> Nullable<Text>,
        #[sql_name="scope"]
        name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    user (id) {
        id -> Binary,
        name -> Varchar,
        user_id -> Varchar,
        creation_date -> Datetime,
        image_url -> Nullable<Text>,
        gender -> Nullable<Varchar>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    user_accepted_client (client_id, user_id) {
        client_id -> Binary,
        user_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    user_has_group (user_id, group_id) {
        #[sql_name="user"]
        user_id -> Binary,
        #[sql_name="group"]
        group_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    user_has_scope (user_id, scope_scope_id) {
        user_id -> Binary,
        scope_scope_id -> Binary,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::enums::*;

    user_policy_consent (policy_id, user_id) {
        policy_id -> Binary,
        user_id -> Binary,
        consent -> Tinyint,
    }
}

joinable!(access_token -> user (user_id));
joinable!(access_token_scope -> scope (scope_scope_id));
joinable!(auth_info -> user (user_id));
joinable!(authorization_code -> user (user));
joinable!(authorization_code_has_scope -> scope (scope_scope_id));
joinable!(client -> user (owner));
joinable!(client_scope_optional -> scope (scope_id));
joinable!(client_scope_requirements -> scope (scope_id));
joinable!(email -> user (user_id));
joinable!(group_has_scope -> group (group_id));
joinable!(group_has_scope -> scope (scope_scope_id));
joinable!(log -> user (initiator_user));
joinable!(phone_number -> user (user_id));
joinable!(refresh_token -> user (user));
joinable!(refresh_token_has_scope -> scope (scope_id));
joinable!(user_accepted_client -> user (user_id));
joinable!(user_has_group -> group (group_id));
joinable!(user_has_group -> user (user_id));
joinable!(user_has_scope -> scope (scope_scope_id));
joinable!(user_has_scope -> user (user_id));
joinable!(user_policy_consent -> meiling_policy (policy_id));
joinable!(user_policy_consent -> user (user_id));

allow_tables_to_appear_in_same_query!(
    access_token,
    access_token_scope,
    authorization_code,
    authorization_code_has_scope,
    auth_info,
    client,
    client_scope_optional,
    client_scope_requirements,
    email,
    group,
    group_has_scope,
    log,
    meiling_policy,
    phone_number,
    refresh_token,
    user_has_group,
    user_has_scope,
    user_policy_consent,
);
