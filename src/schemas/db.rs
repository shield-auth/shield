// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_type"))]
    pub struct RoleType;
}

diesel::table! {
    account (provider, provider_account_id) {
        user_id -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        provider -> Text,
        provider_account_id -> Text,
        refresh_token -> Nullable<Text>,
        access_token -> Nullable<Text>,
        expires_at -> Nullable<Int4>,
        token_type -> Nullable<Text>,
        scope -> Nullable<Text>,
        id_token -> Nullable<Text>,
        session_state -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    authenticator (user_id, credential_id) {
        credential_id -> Text,
        user_id -> Text,
        provider_account_id -> Text,
        credential_public_key -> Text,
        counter -> Int4,
        credential_device_type -> Text,
        credential_backed_up -> Bool,
        transports -> Nullable<Text>,
    }
}

diesel::table! {
    password_reset_token (identifier, token) {
        identifier -> Text,
        token -> Text,
        expires -> Timestamp,
    }
}

diesel::table! {
    resource (id) {
        id -> Int4,
        user_id -> Text,
        name -> Text,
        value -> Text,
        is_default -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    session (session_token) {
        session_token -> Text,
        user_id -> Text,
        expires -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    two_factor_confirmation (identifier, token) {
        identifier -> Text,
        token -> Text,
        expires -> Timestamp,
    }
}

diesel::table! {
    two_factor_token (identifier, token) {
        identifier -> Text,
        token -> Text,
        expires -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoleType;

    user (id) {
        id -> Text,
        name -> Nullable<Text>,
        first_name -> Nullable<Text>,
        last_name -> Nullable<Text>,
        email -> Text,
        email_verified -> Nullable<Timestamp>,
        image -> Nullable<Text>,
        is_two_factor_enabled -> Nullable<Bool>,
        password -> Nullable<Text>,
        is_temp_password -> Nullable<Bool>,
        is_active -> Nullable<Bool>,
        app_user_id -> Text,
        role -> RoleType,
        society_id -> Nullable<Text>,
        membership_id -> Nullable<Text>,
        employee_id -> Nullable<Text>,
        is_multiple_membership -> Nullable<Bool>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    verification_token (identifier, token) {
        identifier -> Text,
        token -> Text,
        expires -> Timestamp,
    }
}

diesel::joinable!(account -> user (user_id));
diesel::joinable!(authenticator -> user (user_id));
diesel::joinable!(resource -> user (user_id));
diesel::joinable!(session -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    authenticator,
    password_reset_token,
    resource,
    session,
    two_factor_confirmation,
    two_factor_token,
    user,
    verification_token,
);
