use std::sync::Arc;

use crate::database::prelude::{Resource, ResourceGroup, User};
use crate::database::{resource, resource::Model as ResourceModel, resource_group};
use crate::mappers::DeleteResponse;
use axum::extract::Path;
use axum::{Extension, Json};
use sea_orm::prelude::Uuid;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::packages::db::AppState;
use crate::{
    packages::{
        errors::{AuthenticateError, Error},
        token::TokenUser,
    },
    utils::role_checker::{is_current_realm_admin, is_master_realm_admin},
};

pub async fn get_users(Path(realm_id): Path<Uuid>) -> String {
    format!("Hi from users of {realm_id}")
}

pub async fn get_user(Path((realm_id, user_id)): Path<(Uuid, Uuid)>) -> String {
    println!("This is user Name: {} - {}", &realm_id, &user_id);
    format!("user is - {} - {}", realm_id, user_id)
}

pub async fn delete_user(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<DeleteResponse>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let result = User::delete_by_id(user_id).exec(&state.db).await?;
        Ok(Json(DeleteResponse {
            ok: result.rows_affected == 1,
        }))
    } else {
        Err(Error::Authenticate(AuthenticateError::ActionForbidden))
    }
}

pub async fn get_resources(
    user: TokenUser,
    Extension(state): Extension<Arc<AppState>>,
    Path((realm_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<ResourceModel>>, Error> {
    if is_master_realm_admin(&user) || is_current_realm_admin(&user, &realm_id.to_string()) {
        let resource_groups = ResourceGroup::find()
            .filter(resource_group::Column::RealmId.eq(realm_id))
            .filter(resource_group::Column::UserId.eq(user_id))
            .all(&state.db)
            .await?;

        let mut resource_group_ids = Vec::new();
        for resource_group in resource_groups {
            resource_group_ids.push(resource_group.id);
        }
        let resources = Resource::find()
            .filter(resource::Column::GroupId.is_in(resource_group_ids))
            .all(&state.db)
            .await?;
        Ok(Json(resources))
    } else {
        Err(Error::Authenticate(AuthenticateError::ActionForbidden))
    }
}
