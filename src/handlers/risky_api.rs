use super::*;

pub fn routes() -> Router<AppStateRef> {
    Router::new().route("/account/risky/api/check", post(risky_api_check))
}

async fn risky_api_check(_: String) -> &'static str {
    r#"{"data":{},message:"OK",retcode:0}"#
}
