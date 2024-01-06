use crate::{common::command::FormatRequestData, RequestData, RequestMethod};

use super::NetworkConditions;
use serde_json::{json, Value};

/// Extra commands specific to Chrome.
#[derive(Debug)]
pub enum ChromeCommand {
    /// Launch the specified Chrome app.
    LaunchApp(String),
    /// Get the current simulated network conditions.
    GetNetworkConditions,
    /// Set the specified network conditions to simulate.
    SetNetworkConditions(NetworkConditions),
    /// Execute the specified Chrome DevTools Protocol command.
    ExecuteCdpCommand(String, Value),
    /// Get the current sinks.
    GetSinks,
    /// Get the issue message.
    GetIssueMessage,
    /// Set the specified sink to use.
    SetSinkToUse(String),
    /// Start tab mirroring.
    StartTabMirroring(String),
    /// Stop casting.
    StopCasting(String),
}

impl FormatRequestData for ChromeCommand {
    fn format_request(&self, session_id: &crate::SessionId) -> crate::RequestData {
        match &self {
            ChromeCommand::LaunchApp(app_id) => RequestData::new(
                RequestMethod::Post,
                format!("/session/{}/chromium/launch_app", session_id),
            )
            .add_body(json!({ "id": app_id })),
            ChromeCommand::GetNetworkConditions => RequestData::new(
                RequestMethod::Get,
                format!("/session/{}/chromium/network_conditions", session_id),
            ),
            ChromeCommand::SetNetworkConditions(conditions) => RequestData::new(
                RequestMethod::Post,
                format!("/session/{}/chromium/network_conditions", session_id),
            )
            .add_body(json!({ "network_conditions": conditions })),
            ChromeCommand::ExecuteCdpCommand(..) => RequestData::new(
                RequestMethod::Post,
                format!("/session/{}/goog/cdp/execute", session_id),
            ),
            ChromeCommand::GetSinks => RequestData::new(
                RequestMethod::Get,
                format!("/session/{}/goog/cast/get_sinks", session_id),
            ),
            ChromeCommand::GetIssueMessage => RequestData::new(
                RequestMethod::Get,
                format!("/session/{}/goog/cast/get_issue_message", session_id),
            ),
            ChromeCommand::SetSinkToUse(sink_name) => RequestData::new(
                RequestMethod::Post,
                format!("/session/{}/goog/cast/set_sink_to_use", session_id),
            )
            .add_body(json!({ "sinkName": sink_name })),
            ChromeCommand::StartTabMirroring(sink_name) => RequestData::new(
                RequestMethod::Post,
                format!("/session/{}/goog/cast/start_tab_mirroring", session_id),
            )
            .add_body(json!({ "sinkName": sink_name })),
            ChromeCommand::StopCasting(sink_name) => RequestData::new(
                RequestMethod::Post,
                format!("/session/{}/goog/cast/stop_casting", session_id),
            )
            .add_body(json!({ "sinkName": sink_name })),
        }
    }
}
