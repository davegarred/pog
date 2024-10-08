use crate::{Authorization, DISCORD_API_ROOT};
use reqwest::header::HeaderMap;

pub fn discord_api_root() -> String {
    DISCORD_API_ROOT.to_string()
}
pub fn discord_headers(authorization: &Authorization) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        authorization.auth_header().parse().unwrap(),
    );
    headers.insert(
        "Content-Type",
        "application/json; charset=UTF-8".parse().unwrap(),
    );
    headers.insert(
        "User-Agent",
        "DiscordBot (https://github.com/davegarred/server, 0.1.0)"
            .parse()
            .unwrap(),
    );
    headers
}
