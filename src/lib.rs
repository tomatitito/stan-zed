use zed::{Command, LanguageServerId, Result, Worktree};
use zed_extension_api as zed;

struct Stan;

impl zed::Extension for Stan {
    fn new() -> Self {
        println!("[STAN EXTENSION] Hello, world!");
        Stan
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        eprintln!(
            "[STAN EXTENSION] language_server_command called for server: {:?}",
            language_server_id
        );

        let binary_path = worktree.which("stan-language-server");

        match binary_path {
            Some(path) => {
                eprintln!("[STAN EXTENSION]stan-language-server found at {path}");
                Ok(Command {
                    command: path,
                    args: Default::default(),
                    env: Default::default(),
                })
            }
            None => {
                eprintln!("[STAN EXTENSION] stan-language-server not found");
                Err("stan-language-server not found".into())
            }
        }
    }
}

zed::register_extension!(Stan);
