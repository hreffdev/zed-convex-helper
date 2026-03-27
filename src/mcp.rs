use zed_extension_api as zed;

struct ConvexMcpDevExtension;

impl zed::Extension for ConvexMcpDevExtension {
    fn new() -> Self {
        Self
    }
}
fn ctx_server_cmd(
    &mut self,
    _ctx_server_id: &zed::ContextServerId,
    _project: &zed::Project,
) -> zed::Result<zed::Command> {
    // Get the system's node binary path, npx should be in the same directory
    let node_path = zed::node_binary_path()?;
    let npx_path = node_path.replace("/node", "/npx");

    Ok(zed::Command {
        command: npx_path,
        args: vec!["-y".to_string(), "zed-convex-mcp@latest".to_string()],
        env: std::env::vars().collect(),
    })
}
