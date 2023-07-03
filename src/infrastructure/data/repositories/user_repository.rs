use crate::core::{
    entities::user::User as UserCore, errors::user_errors::UserError,
    ports::user_port::IUserRepository,
};
use crate::infrastructure::data::models::user::User as UserModel;

#[derive(Clone)]
pub struct UserRepository {
    pub conn: sqlx::MySqlPool,
}

#[async_trait::async_trait]
impl IUserRepository for UserRepository {
    async fn get_users(&self) -> core::result::Result<Vec<UserCore>, UserError> {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM user")
            .fetch_all(&self.conn)
            .await;

        match result {
            Ok(rows) => Ok(UserCore::from_user_model_vec(rows)),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn get_user_by_id(&self, id: u8) -> core::result::Result<UserCore, UserError> {
        let result = sqlx::query_as::<_, UserModel>("SELECT * FROM user WHERE id=?")
            .bind(id)
            .fetch_one(&self.conn)
            .await;

        match result {
            Ok(row) => Ok(UserCore::from_user_model(row)),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn create_user(&self, user: UserCore) -> core::result::Result<(), UserError> {
        let user_model = UserModel::from_user_core(user);

        let result = sqlx::query(
            "INSERT INTO user (name, surname, phone_number, email, password) VALUES (?,?,?,?,?)",
        )
        .bind(&user_model.name)
        .bind(&user_model.surname)
        .bind(&user_model.phone_number.unwrap_or_default())
        .bind(&user_model.email)
        .execute(&self.conn)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("SQLx error: {:?}", err);
                Err(UserError::Unexpected)
            }
        }
    }

    async fn update_user(
        &self,
        user_id: i32,
        user: UserCore,
    ) -> core::result::Result<(), UserError> {
        let user_model = UserModel::from_user_core(user);

        let result =
            sqlx::query("UPDATE user SET name=?, surname=?, phone_number=?, email=? WHERE id=?")
                .bind(&user_model.name)
                .bind(&user_model.surname)
                .bind(&user_model.phone_number.unwrap_or_default())
                .bind(&user_model.email)
                .bind(user_id)
                .execute(&self.conn)
                .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }

    async fn delete_user(&self, user_id: i32) -> core::result::Result<(), UserError> {
        let result = sqlx::query("DELETE FROM user WHERE id=?")
            .bind(user_id)
            .execute(&self.conn)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => match &err {
                sqlx::Error::RowNotFound => Err(UserError::NotFound),
                _ => {
                    log::error!("SQLx error: {:?}", err);
                    Err(UserError::Unexpected)
                }
            },
        }
    }
}

impl UserRepository {
    pub fn new(conn: sqlx::MySqlPool) -> Self {
        UserRepository { conn }
    }
}
