use anyhow::Result;

pub struct AiderExtension {
}

#[async_trait::async_trait]
impl zed::Extension for AiderExtension {
    async fn activate(&mut self, cx: &mut zed::ExtensionContext) -> Result<()> {
        let config = cx.config_value("aider.url").unwrap_or_default().to_string();

        cx.add_command("aider.listFiles", crate::commands::files::list_files)?;
        cx.add_command("aider.askAider", crate::commands::files::ask_aider)?;
        
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
