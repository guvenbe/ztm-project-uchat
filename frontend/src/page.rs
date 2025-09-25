pub mod register;
pub mod login;
pub mod home;
pub mod new_post;

pub use login::Login;
pub use register::Register;
pub use new_post::*;
pub use home::Home;

pub use route::*;

pub mod route {
    pub const ACCOUNT_REGISTER: &str = "/account/register";
    pub const ACCOUNT_LOGIN: &str = "/account/login";
    pub const HOME: &str = "/home";
    pub const POST_NEW_CHAT: &str = "/post/new_chat";
}
