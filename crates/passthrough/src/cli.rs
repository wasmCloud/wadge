use crate::bindings::{exports, wasi};
use crate::Handler;

use wasi::cli::terminal_input::TerminalInput;
use wasi::cli::terminal_output::TerminalOutput;
use wasi::cli::terminal_stderr;
use wasi::cli::terminal_stdin;
use wasi::cli::terminal_stdout;
use wasi::cli::{stderr, stdin, stdout};

impl exports::wasi::cli::environment::Guest for Handler {
    fn get_environment() -> Vec<(String, String)> {
        wasi::cli::environment::get_environment()
    }

    fn get_arguments() -> Vec<String> {
        wasi::cli::environment::get_arguments()
    }

    fn initial_cwd() -> Option<String> {
        wasi::cli::environment::initial_cwd()
    }
}

impl exports::wasi::cli::stdin::Guest for Handler {
    fn get_stdin() -> exports::wasi::io::streams::InputStream {
        exports::wasi::io::streams::InputStream::new(stdin::get_stdin())
    }
}

impl exports::wasi::cli::stdout::Guest for Handler {
    fn get_stdout() -> exports::wasi::io::streams::OutputStream {
        exports::wasi::io::streams::OutputStream::new(stdout::get_stdout())
    }
}

impl exports::wasi::cli::stderr::Guest for Handler {
    fn get_stderr() -> exports::wasi::io::streams::OutputStream {
        exports::wasi::io::streams::OutputStream::new(stderr::get_stderr())
    }
}

impl exports::wasi::cli::terminal_input::Guest for Handler {
    type TerminalInput = TerminalInput;
}

impl exports::wasi::cli::terminal_input::GuestTerminalInput for TerminalInput {}

impl exports::wasi::cli::terminal_output::Guest for Handler {
    type TerminalOutput = TerminalOutput;
}

impl exports::wasi::cli::terminal_output::GuestTerminalOutput for TerminalOutput {}

impl exports::wasi::cli::terminal_stdin::Guest for Handler {
    fn get_terminal_stdin() -> Option<exports::wasi::cli::terminal_input::TerminalInput> {
        terminal_stdin::get_terminal_stdin()
            .map(exports::wasi::cli::terminal_input::TerminalInput::new)
    }
}

impl exports::wasi::cli::terminal_stdout::Guest for Handler {
    fn get_terminal_stdout() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        terminal_stdout::get_terminal_stdout()
            .map(exports::wasi::cli::terminal_output::TerminalOutput::new)
    }
}

impl exports::wasi::cli::terminal_stderr::Guest for Handler {
    fn get_terminal_stderr() -> Option<exports::wasi::cli::terminal_output::TerminalOutput> {
        terminal_stderr::get_terminal_stderr()
            .map(exports::wasi::cli::terminal_output::TerminalOutput::new)
    }
}
