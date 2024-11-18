#!/usr/bin/env cargo +nightly -Zscript
```cargo
[package]
edition = "2021"
[dependencies]
walkdir = "2"
```

use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;
use std::io::{Error, Write};

fn main() {
    let source_dir = PathBuf::from("/Users/alan/Videos");
    let mp4s = get_files_with_extension(&source_dir, vec!["mp4"]);
    generate_crops(&mp4s);
}

fn get_files_with_extension(source_dir: &PathBuf, exts: Vec<&str>) -> Vec<PathBuf> {
    WalkDir::new(source_dir)
        .into_iter()
        .filter(|e| match e.as_ref().unwrap().path().extension() {
            Some(x) => exts.contains(&x.to_str().unwrap()),
            None => false,
        })
        .map(|e| e.unwrap().into_path())
        .collect()
}

fn generate_crops(files: &Vec<PathBuf>) {
    files.iter().for_each(|raw_path| {
        if let Some(base_name) = raw_path.file_name() {
            if let Some(raw_parent) = raw_path.parent() {
                if let Some(raw_check) = raw_parent.file_name() {
                    if raw_check == "raw" {
                        if let Some(main_dir) = raw_parent.parent() {
                            let cropped_path = main_dir.join("cropped").join(base_name);
                            if !cropped_path.exists() {
                                println!("Making: {}", cropped_path.to_str().unwrap());
                                let raw_clone = raw_path.display().to_string();
                                let get_crop_args = vec![
                                    "-hide_banner",
                                    "-i",
                                    raw_clone.as_str(),
                                    "-ss",
                                    "0.1",
                                    "-t",
                                    "0.1",
                                    "-vf",
                                    "cropdetect=limit=20,metadata=mode=print:key=cropdetect",
                                    "-f",
                                    "null",
                                    "-",
                                ];
                                let crop_output =
                                    Command::new("ffmpeg").args(get_crop_args).output().unwrap();
                                let crop_stderr = String::from_utf8(crop_output.stderr).unwrap();
                                if let Ok(crop) = get_crop_details(&crop_stderr) {
                                    let create_args = vec![
                                        "-loglevel",
                                        "error",
                                        "-hide_banner",
                                        "-i",
                                        raw_path.to_str().unwrap(),
                                        "-vf",
                                        &crop,
                                        "-an",
                                        "-y",
                                        cropped_path.to_str().unwrap(),
                                    ];
                                    let create_output =
                                        Command::new("ffmpeg").args(create_args).output().unwrap();
                                }
                            }
                        }
                    }
                }
            }
        }
    });
}

fn get_crop_details(input: &str) -> Result<String, Error> {
    let args = vec!["/crop/ { print $NF }"];
    let mut cmd = Command::new("awk")
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let cmd_stdin = cmd.stdin.as_mut().unwrap();
    cmd_stdin.write_all(input.as_bytes())?;
    let output = cmd.wait_with_output()?;
    let response = String::from_utf8(output.stdout).unwrap();
    Ok(response.lines().next().unwrap().to_string())
}
