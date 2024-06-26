//! Core functionality of shrs
#[macro_use]
extern crate derive_builder;
extern crate lazy_static;

pub mod alias;
pub mod builtin;
pub mod commands;
pub mod completion;
pub mod env;
pub mod history;
pub mod hooks;
pub mod jobs;
pub mod keybinding;
pub mod lang;
#[macro_use]
mod macros;
mod cmd_output;
mod output_writer;
pub mod plugin;
pub mod prompt_content_queue;
pub mod readline;
pub mod shell;
pub mod state;
pub mod theme;

pub use cmd_output::CmdOutput;
pub use output_writer::OutputWriter;

pub mod prelude {
    //! Conveniently import commonly used types

    pub use shrs_core_macros::*;

    pub use crate::{
        alias::{Alias, AliasInfo, AliasRule, AliasRuleCtx},
        builtin::*,
        cmd_output::CmdOutput,
        commands::Commands,
        completion::*,
        env::Env,
        history::*,
        hooks::{events::*, Hook, HookEventMarker, Hooks, IntoHook},
        jobs::{JobId, JobInfo, Jobs},
        keybinding::*,
        lang::{Lang, PosixLang},
        output_writer::OutputWriter,
        plugin::*,
        prompt_content_queue::{PromptContent, PromptContentQueue},
        readline::{
            buffer_history::{BufferHistory, DefaultBufferHistory},
            highlight::{DefaultHighlighter, Highlighter, SyntaxHighlighter, SyntaxTheme},
            line::{Line, LineContents, LineMode, Readline},
            line_events::*,
            menu::{DefaultMenu, DefaultMenuState, Menu},
            prompt::*,
            snippet::*,
            suggester::{DefaultSuggester, Suggester},
            vi::*,
        },
        shell::{set_working_dir, Runtime, Shell, ShellBuilder, ShellConfig},
        state::*,
        theme::Theme,
    };
}

/*
#[cfg(test)]
mod tests {
    use rexpect::session::PtySession;

    fn spawn_proc() -> anyhow::Result<PtySession> {
        let p = rexpect::spawn("cargo run --example tester", Some(2000))?;
        Ok(p)
    }

    #[test]
    fn echo() -> anyhow::Result<()> {
        let mut p = spawn_proc()?;

        p.send_line("echo hi")?;
        p.exp_regex("hi")?;

        p.send_control('c')?;
        Ok(())
    }

    #[test]
    fn pipes() -> anyhow::Result<()> {
        let mut p = spawn_proc()?;

        p.send_line("echo hello | tr e o")?;
        p.exp_regex("hollo")?;

        p.send_line("echo hello | tr e o | tr o a")?;
        p.exp_regex("halla")?;

        p.send_control('c')?;
        Ok(())
    }
}
*/
