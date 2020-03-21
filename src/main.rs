use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() {
    let toolbox_apps_dir = Path::new(env!("LOCALAPPDATA"))
        .join("JetBrains")
        .join("Toolbox")
        .join("apps");

    let idea_exe = vec!["IDEA-U", "IDEA-C", "IDEA-E"]
        .iter()
        .find_map(|it| -> Option<PathBuf> {
            let channel0 = toolbox_apps_dir.join(it).join("ch-0");
            if channel0.exists() {
                let files = fs::read_dir(channel0);
                if files.is_ok() {
                    return files
                        .unwrap()
                        .find_map(|it| -> Option<PathBuf> {
                            it.ok().and_then(|it| -> Option<PathBuf> {
                                if it
                                    .metadata()
                                    .ok()
                                    .map(|it| -> bool { it.is_dir() })
                                    .unwrap_or(false)
                                {
                                    if it
                                        .file_name()
                                        .to_str()
                                        .map(|it| -> bool { !it.ends_with(".plugins") })
                                        .unwrap_or(false)
                                    {
                                        return Some(it.path().join("bin").join("idea64.exe"));
                                    }
                                }
                                None
                            })
                        })
                        .or(None);
                }
            }
            None
        });

    if !idea_exe.is_some() {
        return;
    }

    let idea_exe = idea_exe.unwrap();
    // println!("{}", idea_exe.display());

    Command::new(idea_exe)
        .args(env::args().skip(1))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("IntelliJ Idea");
}
