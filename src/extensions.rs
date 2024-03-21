#[derive(Debug)]
pub struct FileExtension {
    pub name: String,
    pub extensions: Vec<String>,
}

impl FileExtension {
    fn new(name: &str, extensions: Vec<&str>) -> FileExtension {
        FileExtension {
            name: name.to_string(),
            extensions: extensions.iter().map(|s| s.to_string()).collect(),
        }
    }
}

pub fn get_extensions() -> Vec<FileExtension> {
    vec![
        FileExtension::new("Assembly", vec!["asm"]),
        FileExtension::new("C", vec!["c", "h"]),
        FileExtension::new("C++", vec!["cpp", "hpp"]),
        FileExtension::new("C#", vec!["cs"]),
        FileExtension::new("Java", vec!["java"]),
        FileExtension::new("Python", vec!["py"]),
        FileExtension::new("Rust", vec!["rs"]),
        FileExtension::new("Go", vec!["go"]),
        FileExtension::new("Ruby", vec!["rb"]),
        FileExtension::new("JavaScript", vec!["js"]),
        FileExtension::new("TypeScript", vec!["ts"]),
        FileExtension::new("HTML", vec!["html"]),
        FileExtension::new("CSS", vec!["css"]),
        FileExtension::new("Shell", vec!["sh"]),
        FileExtension::new("PowerShell", vec!["ps1"]),
        FileExtension::new("Batch", vec!["bat"]),
        FileExtension::new("Perl", vec!["pl"]),
        FileExtension::new("PHP", vec!["php"]),
        FileExtension::new("Swift", vec!["swift"]),
        FileExtension::new("Kotlin", vec!["kt"]),
        FileExtension::new("Dart", vec!["dart"]),
        FileExtension::new("R", vec!["r"]),
        FileExtension::new("Scala", vec!["scala"]),
        FileExtension::new("Groovy", vec!["groovy"]),
        FileExtension::new("Lua", vec!["lua"]),
        FileExtension::new("Haskell", vec!["hs"]),
        FileExtension::new("Erlang", vec!["erl"]),
        FileExtension::new("Elixir", vec!["ex"]),
        FileExtension::new("Clojure", vec!["clj"]),
        FileExtension::new("F#", vec!["fs"]),
        FileExtension::new("OCaml", vec!["ml"]),
        FileExtension::new("Racket", vec!["rkt"]),
        FileExtension::new("Scheme", vec!["scm"]),
        FileExtension::new("Common Lisp", vec!["lisp"]),
        FileExtension::new("Emacs Lisp", vec!["el"]),
        FileExtension::new("Vim Script", vec!["vim"]),
        FileExtension::new("SQL", vec!["sql"]),
        FileExtension::new("PL/SQL", vec!["plsql"]),
        FileExtension::new("T-SQL", vec!["tsql"]),
        FileExtension::new("Transact-SQL", vec!["t-sql"]),
        FileExtension::new("MySQL", vec!["mysql"]),
        FileExtension::new("PostgreSQL", vec!["pgsql"]),
    ]
}
