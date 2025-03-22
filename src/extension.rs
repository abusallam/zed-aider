use anyhow::Result;
use zed_extension::ext::ExtensionContext; // Import ExtensionContext

pub struct AiderExtension {
}

#[async_trait::async_trait]
impl zed::Extension for AiderExtension {
    async fn activate(&mut self, cx: &mut zed_extension::ExtensionContext) -> Result<()> {
        let config = cx.config_value("aider.url").unwrap_or_default().to_string();

        cx.add_command("aider.listFiles", crate::files::list_files)?;
        cx.add_command("aider.askAider", crate::files::ask_aider)?;

        Ok(())
    }
}

impl Default for AiderExtension {
    fn default() -> Self {
        Self {
        }
    }
}

#[no_mangle]
pub fn init() -> Box<dyn zed::Extension> {
    Box::new(AiderExtension::default())
}

// Implement missing trait methods for zed::Extension
impl zed::Extension for AiderExtension {
    fn manifest(&self) -> std::sync::Arc<zed_extension::extension_manifest::ExtensionManifest> {
        todo!()
    }
    fn work_dir(&self) -> std::sync::Arc<std::path::Path> {
        todo!()
    }
    fn language_server_command(&self, _: lsp::LanguageServerName, _: language::language_registry::LanguageName, _: std::sync::Arc<dyn zed_extension::workspace::WorktreeDelegate>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<zed_extension::command::Command, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn language_server_initialization_options(&self, _: lsp::LanguageServerName, _: language::language_registry::LanguageName, _: std::sync::Arc<dyn zed_extension::workspace::WorktreeDelegate>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Option<String>, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn language_server_workspace_configuration(&self, _: lsp::LanguageServerName, _: std::sync::Arc<dyn zed_extension::workspace::WorktreeDelegate>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Option<String>, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn labels_for_completions(&self, _: lsp::LanguageServerName, _: Vec<lsp::completion::Completion>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Vec<Option<zed_extension::components::code_label::CodeLabel>>, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn labels_for_symbols(&self, _: lsp::LanguageServerName, _: Vec<lsp::symbol::Symbol>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Vec<Option<zed_extension::components::code_label::CodeLabel>>, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn complete_slash_command_argument(&self, _: zed_extension::command::SlashCommand, _: Vec<String>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Vec<zed_extension::command::SlashCommandArgumentCompletion>, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn run_slash_command(&self, _: zed_extension::command::SlashCommand, _: Vec<String>, _: Option<std::sync::Arc<dyn zed_extension::workspace::WorktreeDelegate>>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<zed_extension::command::SlashCommandOutput, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn context_server_command(&self, _: std::sync::Arc<str>, _: std::sync::Arc<dyn zed_extension::project::ProjectDelegate>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<zed_extension::command::Command, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn suggest_docs_packages(&self, _: std::sync::Arc<str>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<Vec<String>, anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
    fn index_docs(&self, _: std::sync::Arc<str>, _: std::sync::Arc<str>, _: std::sync::Arc<dyn zed_extension::storage::key_value_store::KeyValueStoreDelegate>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<(), anyhow::Error>> + Send + 'async_trait::async_trait>> {
        todo!()
    }
}
