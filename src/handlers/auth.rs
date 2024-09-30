use chrono::Utc;
use entity::{client, resource, resource_group, session, user};
use sea_orm::{ActiveModelTrait, Set};
use std::sync::Arc;

use crate::{
    mappers::auth::{CreateUserRequest, IntrospectRequest, IntrospectResponse, LogoutRequest, LogoutResponse},
    middleware::session_info_extractor::SessionInfo,
    packages::{
        db::AppState,
        errors::{AuthenticateError, Error},
        settings::SETTINGS,
        token::{create, decode, TokenUser},
    },
    services::user::insert_user,
    utils::role_checker::{is_current_realm_admin, is_master_realm_admin},
};
use axum::{extract::Path, Extension, Json};
use sea_orm::{prelude::Uuid, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    user: user::Model,
    realm_id: Uuid,
    client_id: Uuid,
}

pub async fn login(
    Extension(state): Extension<Arc<AppState>>,
    Extension(session_info): Extension<Arc<SessionInfo>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<Credentials>,
) -> Result<Json<LoginResponse>, Error> {
    debug!("🚀 Login request received! {:#?}", session_info);
    let user_with_resource_groups = user::Entity::find()
        .filter(user::Column::Email.eq(payload.email))
        .find_also_related(resource_group::Entity)
        .filter(resource_group::Column::RealmId.eq(realm_id))
        .filter(resource_group::Column::ClientId.eq(client_id))
        .one(&state.db)
        .await?;

    if user_with_resource_groups.is_none() {
        debug!("No matching data found");
        return Err(Error::not_found());
    }

    let (user, resource_groups) = user_with_resource_groups.unwrap();
    if !user.verify_password(&payload.password) {
        debug!("Wrong password");
        return Err(Error::Authenticate(AuthenticateError::WrongCredentials));
    }
    if user.locked_at.is_some() {
        debug!("User is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    if resource_groups.is_none() {
        debug!("No matching resource group found");
        return Err(Error::not_found());
    }

    let resource_groups = resource_groups.unwrap();
    if resource_groups.locked_at.is_some() {
        debug!("Resource group is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    // Fetch client separately
    let client = client::Entity::find_by_id(client_id).one(&state.db).await?.ok_or_else(|| {
        debug!("No client found");
        Error::not_found()
    })?;

    if client.locked_at.is_some() {
        debug!("Client is locked");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    let sessions = session::Entity::find()
        .filter(session::Column::ClientId.eq(client.id))
        .filter(session::Column::UserId.eq(user.id))
        .filter(session::Column::Expires.gt(chrono::Utc::now()))
        .count(&state.db)
        .await?;

    if sessions >= client.max_concurrent_sessions.unwrap() as u64 {
        debug!("Client has reached max concurrent sessions");
        return Err(Error::Authenticate(AuthenticateError::MaxConcurrentSessions));
    }

    // Fetch resources
    let resources = resource::Entity::find()
        .filter(resource::Column::GroupId.eq(resource_groups.id))
        .filter(resource::Column::LockedAt.is_null())
        .all(&state.db)
        .await?;

    if resources.is_empty() {
        debug!("No resources found");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    let session = session::ActiveModel {
        user_id: Set(user.id),
        client_id: Set(client.id),
        ip_address: Set(session_info.ip_address.to_string()),
        user_agent: Set(Some(session_info.user_agent.to_string())),
        browser: Set(Some(session_info.browser.to_string())),
        browser_version: Set(Some(session_info.browser_version.to_string())),
        operating_system: Set(Some(session_info.operating_system.to_string())),
        device_type: Set(Some(session_info.device_type.to_string())),
        country_code: Set(session_info.country_code.to_string()),
        expires: Set((Utc::now() + chrono::Duration::seconds(client.session_lifetime as i64)).into()),
        ..Default::default()
    };
    let session = session.insert(&state.db).await?;

    let access_token = create(
        user.clone(),
        client,
        resource_groups,
        resources,
        session,
        &SETTINGS.read().secrets.signing_key,
    )
    .unwrap();

    Ok(Json(LoginResponse {
        access_token,
        user,
        realm_id,
        client_id,
    }))
}

pub async fn register(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<user::Model>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let user = insert_user(&state.db, realm_id, client_id, payload).await?;
        Ok(Json(user))
    } else {
        return Err(Error::Authenticate(AuthenticateError::ActionForbidden));
    }
}

pub async fn logout_current_session(user: TokenUser, Extension(state): Extension<Arc<AppState>>) -> Result<Json<LogoutResponse>, Error> {
    let result = session::Entity::delete_by_id(user.sid).exec(&state.db).await?;
    Ok(Json(LogoutResponse {
        ok: result.rows_affected == 1,
        user_id: user.sub,
        session_id: user.sid,
    }))
}

pub async fn logout(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, _)): Path<(Uuid, Uuid)>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        match payload.access_token {
            Some(access_token) => {
                let sid = decode(&access_token, &SETTINGS.read().secrets.signing_key)
                    .map_err(|_| AuthenticateError::InvalidToken)?
                    .claims
                    .sid;
                let result = session::Entity::delete_by_id(sid).exec(&state.db).await?;
                Ok(Json(LogoutResponse {
                    ok: result.rows_affected == 1,
                    user_id: user.sub,
                    session_id: user.sid,
                }))
            }
            None => match payload.refresh_token {
                Some(refresh_token) => {
                    let sid = decode(&refresh_token, &SETTINGS.read().secrets.signing_key)
                        .map_err(|_| AuthenticateError::InvalidToken)?
                        .claims
                        .sid;
                    let result = session::Entity::delete_by_id(sid).exec(&state.db).await?;
                    Ok(Json(LogoutResponse {
                        ok: result.rows_affected == 1,
                        user_id: user.sub,
                        session_id: user.sid,
                    }))
                }
                None => Err(Error::Authenticate(AuthenticateError::NoResource)),
            },
        }
    } else {
        Err(Error::Authenticate(AuthenticateError::ActionForbidden))
    }
}

pub async fn logout_my_all_sessions(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((_, client_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<LogoutResponse>, Error> {
    let result = session::Entity::delete_many()
        .filter(session::Column::ClientId.eq(client_id))
        .filter(session::Column::UserId.eq(user.sub))
        .exec(&state.db)
        .await?;
    Ok(Json(LogoutResponse {
        ok: result.rows_affected > 0,
        user_id: user.sub,
        session_id: user.sid,
    }))
}

pub async fn logout_all(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<LogoutRequest>,
) -> Result<Json<LogoutResponse>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        match payload.access_token {
            Some(access_token) => {
                let sub = decode(&access_token, &SETTINGS.read().secrets.signing_key)
                    .map_err(|_| AuthenticateError::InvalidToken)?
                    .claims
                    .sub;
                let result = session::Entity::delete_many()
                    .filter(session::Column::ClientId.eq(client_id))
                    .filter(session::Column::UserId.eq(sub))
                    .exec(&state.db)
                    .await?;
                Ok(Json(LogoutResponse {
                    ok: result.rows_affected > 0,
                    user_id: user.sub,
                    session_id: user.sid,
                }))
            }
            None => match payload.refresh_token {
                Some(refresh_token) => {
                    let sub = decode(&refresh_token, &SETTINGS.read().secrets.signing_key)
                        .map_err(|_| AuthenticateError::InvalidToken)?
                        .claims
                        .sub;
                    let result = session::Entity::delete_many()
                        .filter(session::Column::ClientId.eq(client_id))
                        .filter(session::Column::UserId.eq(sub))
                        .exec(&state.db)
                        .await?;
                    Ok(Json(LogoutResponse {
                        ok: result.rows_affected > 0,
                        user_id: user.sub,
                        session_id: user.sid,
                    }))
                }
                None => Err(Error::Authenticate(AuthenticateError::NoResource)),
            },
        }
    } else {
        Err(Error::Authenticate(AuthenticateError::ActionForbidden))
    }
}

pub async fn introspect(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, client_id)): Path<(Uuid, Uuid)>,
    Json(payload): Json<IntrospectRequest>,
) -> Result<Json<IntrospectResponse>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let token_data = decode(&payload.access_token, &SETTINGS.read().secrets.signing_key).expect("Failed to decode token");

        if token_data.claims.resource.is_none() || token_data.claims.resource.is_some() && token_data.claims.resource.unwrap().client_id != client_id
        {
            return Err(Error::Authenticate(AuthenticateError::NoResource));
        }

        let session = session::Entity::find_by_id(token_data.claims.sid).one(&state.db).await?;
        match session {
            Some(session) => {
                let user = user::Entity::find_by_id(session.user_id)
                    .filter(user::Column::LockedAt.is_null())
                    .one(&state.db)
                    .await?;

                match user {
                    Some(user) => {
                        let client = client::Entity::find_by_id(session.client_id)
                            .filter(client::Column::LockedAt.is_null())
                            .one(&state.db)
                            .await?;

                        match client {
                            Some(client) => {
                                let resource_group = resource_group::Entity::find()
                                    .filter(resource_group::Column::RealmId.eq(realm_id))
                                    .filter(resource_group::Column::ClientId.eq(client.id))
                                    .filter(resource_group::Column::UserId.eq(user.id))
                                    .filter(resource_group::Column::LockedAt.is_null())
                                    .one(&state.db)
                                    .await?;

                                match resource_group {
                                    Some(resource_group) => {
                                        let resources = resource::Entity::find()
                                            .filter(resource::Column::GroupId.eq(resource_group.id))
                                            .filter(resource::Column::LockedAt.is_null())
                                            .all(&state.db)
                                            .await?;
                                        Ok(Json(IntrospectResponse {
                                            active: true,
                                            client_id: client.id,
                                            first_name: user.first_name.to_string(),
                                            last_name: Some(user.last_name.unwrap_or("".to_string())),
                                            sub: user.id,
                                            token_type: "bearer".to_string(),
                                            exp: token_data.claims.exp,
                                            iat: token_data.claims.iat,
                                            iss: SETTINGS.read().server.host.clone(),
                                            client_name: client.name,
                                            resource_group: resource_group.name,
                                            resources: resources.iter().map(|r| r.name.clone()).collect::<Vec<String>>(),
                                        }))
                                    }
                                    None => Err(Error::Authenticate(AuthenticateError::NoResource))?,
                                }
                            }
                            None => Err(Error::Authenticate(AuthenticateError::NoResource)),
                        }
                    }
                    None => Err(Error::Authenticate(AuthenticateError::NoResource)),
                }
            }
            None => Err(Error::Authenticate(AuthenticateError::NoResource)),
        }
    } else {
        Err(Error::Authenticate(AuthenticateError::ActionForbidden))
    }
}
