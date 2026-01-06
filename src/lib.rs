use zed_extension_api as zed;

struct MossExtension {
    // ... state
}

impl zed::Extension for MossExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Ok(zed::Command {
            command:
                "C:/resource/moss-lang/target/debug/moss-language-server.exe"
                    .into(),
            args: Default::default(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(MossExtension);
