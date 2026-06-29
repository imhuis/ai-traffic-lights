use crate::domain::status_signal::CodexState;

pub const PROVIDER_NAME: &str = "codex-app-server";

pub fn map_codex_activity(activity: &str) -> CodexState {
    let activity = activity.to_ascii_lowercase();
    if activity == "item/commandexecution/requestapproval"
        || activity == "item/filechange/requestapproval"
        || activity == "item/tool/requestuserinput"
        || activity == "mcpserver/elicitation/request"
        || activity == "item/permissions/requestapproval"
        || activity.contains("approval")
        || activity.contains("confirm")
        || activity.contains("waiting")
        || activity.contains("permission")
        || activity.contains("requestuserinput")
        || activity.contains("elicitation")
    {
        CodexState::WaitingForConfirmation
    } else if activity == "turn/start"
        || activity == "turn/steer"
        || activity.contains("running")
        || activity.contains("stream")
        || activity.contains("tool")
        || activity.contains("working")
        || activity.contains("turn")
    {
        CodexState::Working
    } else if activity.contains("idle") || activity.contains("available") {
        CodexState::Idle
    } else {
        CodexState::Unavailable
    }
}
