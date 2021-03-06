use std::path::PathBuf;

use commands::exec::language::Language;
use duct::{ cmd, Expression };

#[derive(Debug)]
pub struct Kotlin;

impl Kotlin {
    fn get_class_name(&self, src_path: &PathBuf) -> String {
        src_path.with_extension("").file_name().unwrap().to_str().unwrap().into()
    }
}

impl Language for Kotlin {
    fn get_lang_name(&self) -> String {
        "Kotlin".into()
    }

    fn get_source_file_ext(&self) -> String {
        ".kt".into()
    }

    fn pre_process_code(&self, code: &str, _src_path: &PathBuf) -> Option<String> {
        use regex::Regex;

        let re = Regex::new(r"fun\s*main\s*\(.*\)").unwrap();
        if !re.is_match(&code) {
            let result = format!("fun main() {{\r\n{}\r\n}}", code);
            return Some(result);
        }

        None
    }

    fn get_out_path(&self, src_path: &PathBuf) -> PathBuf {
        PathBuf::from(self.get_class_name(src_path))
    }

    fn get_compiler_command(&self, src_path: &PathBuf, _exe_path: &PathBuf) -> Option<Expression> {
        Some(cmd!("kotlinc", src_path, "-include-runtime", "-d", format!("{}.jar", self.get_class_name(src_path))))

    }

    fn get_execution_command(&self, path: &PathBuf) -> Expression {
        cmd!("java", "-jar", format!("{}.jar", path.to_str().unwrap()))
    }

    fn check_compiler_or_interpreter(&self) -> Expression {
        cmd!("kotlinc", "-version")
    }
}
