use anyhow::Result;
use crate::api::CoolifyClient;

pub struct CoolifyExtension {
    pub client: Option<CoolifyClient>
}

#[async_trait::async_trait]
impl zed::Extension for CoolifyExtension {
    async fn activate(&mut self, cx: &mut zed::ExtensionContext) -> Result<()> {
        let config = cx.config_value("coolify.url").unwrap().to_string();
        self.client = Some(CoolifyClient::new(&config));

        cx.add_command("coolify.list_servers", crate::commands::list_servers)?;
        cx.add_command("coolify.list_applications", crate::commands::list_applications)?;
        cx.add_command("coolify.deploy_application", crate::commands::deploy_application)?;
        cx.add_command("coolify.get_application_logs", crate::commands::get_application_logs)?;
        
        Ok(())
    }
}

impl Default for CoolifyExtension {
    fn default() -> Self {
        Self {
            client: None
        }
    }
}

#[no_mangle]
pub fn init() -> Box<dyn zed::Extension> {
    Box::new(CoolifyExtension::default())
}
