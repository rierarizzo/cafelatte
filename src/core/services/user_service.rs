use crate::{
    core::{
        entities::user::User,
        errors::user_errors::UserError,
        ports::user_port::{IUserRepository, IUserService},
    },
    utils::security_util::crypt_password,
};

#[derive(Clone)]
pub struct UserService<R>
where
    R: IUserRepository,
{
    user_repository: R,
}

#[async_trait::async_trait]
impl<R> IUserService for UserService<R>
where
    R: IUserRepository,
{
    async fn get_users(&self) -> Result<Vec<User>, UserError> {
        self.user_repository.get_users().await
    }

    async fn get_user_by_id(&self, id: u8) -> Result<User, UserError> {
        self.user_repository.get_user_by_id(id).await
    }

    async fn register(&self, mut user: User) -> Result<(), UserError> {
        let cypted_password = crypt_password(&user.password);
        user.set_password(cypted_password);

        self.user_repository.register(user).await
    }

    async fn update_user(&self, user_id: i32, user: User) -> Result<(), UserError> {
        self.user_repository.update_user(user_id, user).await
    }

    async fn delete_user(&self, user_id: i32) -> Result<(), UserError> {
        self.user_repository.delete_user(user_id).await
    }
}

impl<R> UserService<R>
where
    R: IUserRepository,
{
    pub fn new(user_repository: R) -> Self {
        UserService { user_repository }
    }
}
