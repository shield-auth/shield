use crate::packages::{api_token::RefreshTokenClaims, errors::Error};
use entity::{client, refresh_token};
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseTransaction, EntityTrait, Set};

pub async fn handle_refresh_token(
    txn: &DatabaseTransaction,
    refresh_token: &refresh_token::Model,
    client: &client::Model,
) -> Result<RefreshTokenClaims, Error> {
    let refresh_token_model = if refresh_token.re_used_count >= client.refresh_token_reuse_limit {
        refresh_token::Entity::delete_by_id(refresh_token.id).exec(txn).await?;
        let model = refresh_token::ActiveModel {
            id: Set(Uuid::now_v7()),
            user_id: Set(refresh_token.user_id),
            client_id: Set(Some(client.id)),
            realm_id: Set(client.realm_id),
            re_used_count: Set(0),
            locked_at: Set(None),
            ..Default::default()
        };
        model.insert(txn).await?
    } else {
        let model = refresh_token::ActiveModel {
            id: Set(refresh_token.id),
            user_id: Set(refresh_token.user_id),
            client_id: Set(refresh_token.client_id),
            realm_id: Set(refresh_token.realm_id),
            re_used_count: Set(refresh_token.re_used_count + 1),
            locked_at: Set(None),
            ..Default::default()
        };
        model.update(txn).await?
    };

    Ok(RefreshTokenClaims::from(&refresh_token_model, client))
}
