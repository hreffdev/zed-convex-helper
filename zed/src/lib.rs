use zed_extension_api as zed;

struct ZedConvexDevExtension;

impl zed::Extension for ZedConvexDevExtension {
    fn new() -> Self {
        Self
    }
    fn context_server_command(
        &mut self,
        _context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> zed::Result<zed::Command> {
        let bun_path = zed::node_binary_path()?;
        let bunx_path = bun_path.replace("/bun", "/bunx");

        Ok(zed::Command {
            command: bunx_path,
            args: vec!["-y".to_string(), "convex_analyzer@latest".to_string()],
            env: std::env::vars().collect(),
        })
    }
}

zed::register_extension!(ZedConvexDevExtension);
