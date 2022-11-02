use xshell::Cmd;

pub fn wasmer_cmd<'a>(shell: &'a mut xshell::Shell) -> xshell::Cmd<'a> {
    let cmd = xshell::cmd!(shell, "wasmer");
    cmd.args(&[
        "--use",
        "sharrattj/coreutils",
        "--enable-threads",
        "--allow-multiple-wasi-versions",
    ])
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct ExecSnapshot {
    pub result: Result<(), String>,
    pub code: i32,
    pub stdin: String,
    pub stdout: String,
    pub stderr: String,
}

pub trait CommandExt {
    fn snapshot<I: Into<String>>(self, stdin: I) -> ExecSnapshot;
}

impl<'a> CommandExt for Cmd<'a> {
    fn snapshot<I: Into<String>>(self, stdin: I) -> ExecSnapshot {
        let stdin = stdin.into();
        match self.stdin(&stdin).output() {
            Ok(out) => ExecSnapshot {
                result: Ok(()),
                stdin,
                code: out.status.code().unwrap_or_default(),
                stdout: String::from_utf8_lossy(&out.stdout).to_string(),
                stderr: String::from_utf8_lossy(&out.stderr).to_string(),
            },
            Err(err) => ExecSnapshot {
                stdin,
                result: Err(err.to_string()),
                code: -1,
                stdout: String::new(),
                stderr: String::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use insta::assert_yaml_snapshot;
    use xshell::Shell;

    use super::*;

    fn wasm_dir() -> PathBuf {
        std::env::current_dir().unwrap().parent().unwrap().join("wasm")
    }

    #[test]
    fn test_dash() {
        let dash_wasm = wasm_dir().join("dash.wasm");
        assert!(dash_wasm.is_file());
        let mut sh = Shell::new().unwrap();
        let snap = wasmer_cmd(&mut sh)
            .arg(&dash_wasm)
            .snapshot("echo 2");
        assert_yaml_snapshot!(snap);
    }
}
