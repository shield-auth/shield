// @generated automatically by Diesel CLI.

diesel::table! {
    account (provider, provider_account_id) {
        user_id -> Int4,
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
        user_id -> Int4,
        provider_account_id -> Text,
        credential_public_key -> Text,
        counter -> Int4,
        credential_device_type -> Text,
        credential_backed_up -> Bool,
        transports -> Nullable<Text>,
    }
}

diesel::table! {
    client (id) {
        id -> Int4,
        name -> Text,
        two_factor_enabled_at -> Nullable<Timestamp>,
        locked_at -> Nullable<Timestamp>,
        realm_id -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
    realm (id) {
        id -> Int4,
        name -> Text,
        slug -> Text,
        locked_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    resource (id) {
        id -> Int4,
        group_id -> Nullable<Int4>,
        name -> Text,
        value -> Text,
        description -> Nullable<Text>,
        locked_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    resource_group (id) {
        id -> Int4,
        realm_id -> Nullable<Int4>,
        client_id -> Nullable<Int4>,
        user_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        is_default -> Nullable<Bool>,
        locked_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    session (session_token) {
        session_token -> Text,
        user_id -> Int4,
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
    user (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Nullable<Text>,
        email -> Text,
        email_verified_at -> Nullable<Timestamp>,
        image -> Nullable<Text>,
        two_factor_enabled_at -> Nullable<Timestamp>,
        password_hash -> Nullable<Text>,
        is_temp_password -> Nullable<Bool>,
        locked_at -> Nullable<Timestamp>,
        realm_id -> Nullable<Int4>,
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
diesel::joinable!(client -> realm (realm_id));
diesel::joinable!(resource -> resource_group (group_id));
diesel::joinable!(resource_group -> client (client_id));
diesel::joinable!(resource_group -> realm (realm_id));
diesel::joinable!(resource_group -> user (user_id));
diesel::joinable!(session -> user (user_id));
diesel::joinable!(user -> realm (realm_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    authenticator,
    client,
    password_reset_token,
    realm,
    resource,
    resource_group,
    session,
    two_factor_confirmation,
    two_factor_token,
    user,
    verification_token,
);
