use std::path::PathBuf;

use commands::exec::language::Language;
use duct::{ cmd, Expression };

#[derive(Debug)]
pub struct Python;

impl Python {
    fn get_interpreter(&self) -> String {
        if cfg!(windows) {
            "python".into()
        } else {
            "python3".into()
        }
    }
}

impl Language for Python {
    fn get_lang_name(&self) -> String {
        "Python".into()
    }

    fn get_source_file_ext(&self) -> String {
        ".py".into()
    }

    fn get_execution_command(&self, path: &PathBuf) -> Expression {
        cmd!(self.get_interpreter(), path)
    }

    fn check_compiler_or_interpreter(&self) -> Expression {
        cmd!(self.get_interpreter(), "--version")
    }
}
