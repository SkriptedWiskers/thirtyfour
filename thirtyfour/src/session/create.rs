use serde::Deserialize;
use url::Url;

use crate::{
    common::{
        command::{Command, FormatRequestData},
        config::WebDriverConfig,
    },
    prelude::{WebDriverError, WebDriverResult},
    session::http::run_webdriver_cmd,
    Capabilities, SessionId, TimeoutConfiguration,
};

use super::http::HttpClient;

/// Start a new WebDriver session, returning the session id and the
/// capabilities JSON that was received back from the server.
pub async fn start_session(
    http_client: &(dyn HttpClient + Send + Sync),
    server_url: &Url,
    config: &WebDriverConfig,
    capabilities: Capabilities,
) -> WebDriverResult<SessionId> {
    let request_data = Command::NewSession(serde_json::Value::Object(capabilities))
        .format_request(&SessionId::null());

    let v = match run_webdriver_cmd(http_client, request_data.clone(), server_url, config).await {
        Ok(x) => Ok(x),
        Err(e) => {
            // Selenium sometimes gives a bogus 500 error "Chrome failed to start".
            // Retry if we get a 500. If it happens twice in a row then the second error
            // will be returned.
            if let WebDriverError::UnknownError(x) = &e {
                if x.status == 500 {
                    run_webdriver_cmd(http_client, request_data, server_url, config).await
                } else {
                    Err(e)
                }
            } else {
                Err(e)
            }
        }
    }?;

    #[derive(Debug, Deserialize)]
    struct ConnectionData {
        #[serde(default, rename(deserialize = "sessionId"))]
        session_id: String,
        // #[serde(default)]
        // capabilities: serde_json::Value,
    }

    #[derive(Debug, Deserialize)]
    struct ConnectionResp {
        #[serde(default, rename(deserialize = "sessionId"))]
        session_id: String,
        value: ConnectionData,
    }

    let resp: ConnectionResp = serde_json::from_value(v.body)?;
    let data = resp.value;
    let session_id = SessionId::from(if resp.session_id.is_empty() {
        data.session_id
    } else {
        resp.session_id
    });

    // Set default timeouts.
    let request_data =
        Command::SetTimeouts(TimeoutConfiguration::default()).format_request(&session_id);
    run_webdriver_cmd(http_client, request_data, server_url, config).await?;

    Ok(session_id)
}
