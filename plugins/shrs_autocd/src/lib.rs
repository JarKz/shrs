use std::fs;

use shrs::prelude::*;

pub struct AutocdPlugin;

pub fn after_command_hook(
    mut rt: StateMut<Runtime>,
    sh: &Shell,
    ctx: &AfterCommandCtx,
) -> anyhow::Result<()> {
    // Bash exit code for invalid command
    if let Some(exit_code) = ctx.cmd_output.status.code() {
        if exit_code == 127 {
            // Check if the command name matches a directory
            let Some(cmd_name) = ctx.command.split(' ').next() else {
                return Ok(());
            };

            let paths = fs::read_dir("./").unwrap();

            for path in paths {
                let path = path.unwrap();
                if path.file_type().unwrap().is_dir() && path.file_name() == cmd_name {
                    set_working_dir(&sh, &mut rt, &path.path(), true)?;
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

impl Plugin for AutocdPlugin {
    fn init(&self, shell: &mut ShellConfig) -> anyhow::Result<()> {
        shell.hooks.insert(after_command_hook);

        Ok(())
    }
}
