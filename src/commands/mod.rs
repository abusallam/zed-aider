use anyhow::Result;
use zed::Task;

pub async fn list_servers(cx: zed::CommandContext) -> Result<Task> {
    Ok(Task::Ready(Ok(())))
}

pub async fn list_applications(cx: zed::CommandContext) -> Result<Task> {
    Ok(Task::Ready(Ok(())))
}

pub async fn deploy_application(cx: zed::CommandContext) -> Result<Task> {
    Ok(Task::Ready(Ok(())))
}

pub async fn get_application_logs(cx: zed::CommandContext) -> Result<Task> {
    Ok(Task::Ready(Ok(())))
}
