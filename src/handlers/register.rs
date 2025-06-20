use super::*;
use crate::database::schema::{Password, PasswordError, Username};

const REGISTER_PAGE: Html<&str> = Html(include_str!("../../html/register.html"));

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/account/register", get(|| async { REGISTER_PAGE }))
        .route("/account/register", post(process_register_request))
}

#[derive(Deserialize)]
struct RegisterForm {
    pub username: String,
    pub password: String,
    pub password_v2: String,
}

async fn process_register_request(
    State(state): State<AppStateRef>,
    Form(form): Form<RegisterForm>,
) -> Html<String> {
    let Some(username) = Username::parse(form.username) else {
        return html_result(ResultStatus::Failure, "无效的用户名格式；应包含[A-Za-z0-9_]字符且至少6个字符长。");
    };

    let password = match Password::new(form.password, form.password_v2) {
        Ok(password) => password,
        Err(PasswordError::PairMismatch) => {
            return html_result(ResultStatus::Failure, "两次输入的密码不匹配")
        }
        Err(PasswordError::RequirementsMismatch) => {
            return html_result(
                ResultStatus::Failure,
                "密码长度应在8到30个字符之间",
            )
        }
        Err(PasswordError::HashFailed(err)) => {
            tracing::error!("密码哈希失败, 错误: {err}");
            return html_result(ResultStatus::Failure, "服务器内部错误");
        }
    };

    match state.db.create_account(username, password).await {
        Ok(Some(_)) => html_result(
            ResultStatus::Success,
            "账号注册成功，现在您可以使用游戏内登录",
        ),
        Ok(None) => html_result(
            ResultStatus::Failure,
            "该用户名已被注册",
        ),
        Err(err) => {
            tracing::error!("数据库操作错误: {err}");
            html_result(ResultStatus::Failure, "服务器内部错误")
        }
    }
}

enum ResultStatus {
    Failure,
    Success,
}

impl ResultStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Failure => "error",
            Self::Success => "success",
        }
    }
}

fn html_result(result: ResultStatus, message: &str) -> Html<String> {
    static RESULT_HTML: &str = include_str!("../../html/result.html");

    Html(
        RESULT_HTML
            .replace("%RESULT%", result.as_str())
            .replace("%MESSAGE%", message),
    )
}