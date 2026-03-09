use zed_extension_api as zed;

struct SilverStripeExtension;

impl zed::Extension for SilverStripeExtension {
    fn new() -> Self {
        SilverStripeExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree
            .which("silverstripe-language-server")
            .ok_or_else(|| {
                "silverstripe-language-server not found. Install with: npm install -g silverstripe-language-server".to_string()
            })?;

        Ok(zed::Command {
            command: path,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(SilverStripeExtension);
