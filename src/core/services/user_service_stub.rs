use crate::core::{entities::user::User, errors::user_errors::UserError, ports::user_port::IUserService};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UserServiceStub {
    pub success: bool,
}

#[async_trait]
impl IUserService for UserServiceStub {
    async fn get_users(&self) -> Result<Vec<User>, UserError> {
        if self.success {
            Ok(vec![User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
            }])
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn get_user_by_id(&self, _id: u8) -> Result<User, UserError> {
        if self.success {
            Ok(User {
                id: Some(1),
                name: "Keneth".to_string(),
                surname: "Riera".to_string(),
            })
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn create_user(&self, _user: User) -> Result<(), UserError> {
        if self.success {
            Ok(())
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn update_user(&self, _user_id: i32, _user: User) -> Result<(), UserError> {
        if self.success {
            Ok(())
        } else {
            Err(UserError::Unexpected)
        }
    }

    async fn delete_user(&self, _user_id: i32) -> Result<(), UserError> {
        if self.success {
            Ok(())
        } else {
            Err(UserError::Unexpected)
        }
    }
}
