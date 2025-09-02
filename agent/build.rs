use std::env;
use std::process::Command;
use anyhow::Result;
use chrono::Local;

fn get_branch() -> Result<String> {
    if let Ok(branch) = env::var("GITHUB_REF_NAME") {
        return Ok(branch);
    }

    let output = Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;
    if output.status.success() {
        return Ok(String::from_utf8(output.stdout)?);
    }

    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;
    if output.status.success() && &output.stdout != "HEAD".as_bytes() {
        return Ok(String::from_utf8(output.stdout)?);
    }

    let output = Command::new("git")
        .args(["log", "-n", "1", "--pretty=%D", "HEAD"])
        .output()?;
    if output.status.success() {
        // output: HEAD -> master, origin/main
        return match output.stdout.iter().position(|x| *x == ',' as u8) {
            Some(mut position) => {
                while (output.stdout[position] as char).is_ascii_whitespace()
                    && position < output.stdout.len()
                {
                    position += 1;
                }
                Ok(str::from_utf8(&output.stdout[position..])?.to_owned())
            }
            _ => Ok(String::from_utf8(output.stdout)?),
        };
    }

    panic!("no branch name found")
}
struct EnvCommand(&'static str, Vec<&'static str>);
fn set_build_info() -> Result<()> {
    println!("cargo:rustc-env=AGENT_NAME=deepflow-agent-ce");
    println!("cargo:rustc-env=BRANCH={}", get_branch()?);
    println!(
        "cargo:rustc-env=COMPILE_TIME={}",
        Local::now().format("%F %T")
    );
    let entries = vec![
        EnvCommand("COMMIT_ID", vec!["git", "rev-parse", "HEAD"]),
        EnvCommand("REV_COUNT", vec!["git", "rev-list", "--count", "HEAD"]),
        EnvCommand("RUSTC_VERSION", vec!["rustc", "--version"]),
    ];
    for e in entries {
        let output = Command::new(e.1[0]).args(&e.1[1..]).output()?.stdout;
        println!("cargo:rustc-env={}={}", e.0, String::from_utf8(output)?);
    }
    Ok(())
}
fn make_pulsar_proto()->Result<()>{
    tonic_build::configure()
        .field_attribute(".", "#[serde(skip_serializing_if = \"Option::is_none\")]")
        .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")
        .build_server(false)
        .emit_rerun_if_changed(false)
        .out_dir("src/flow_generator/protocol_logs/mq")
        .compile(
            &["src/flow_generator/protocol_logs/mq/PulsarApi.proto"],
            &["src/flow_generator/protocol_logs/mq"],
        )?;
    println!("cargo:rerun-if-changed=src/flow_generator/protocol_logs/mq/PulsarApi.proto");
    // remove `#[serde(skip_serializing_if = "Option::is_none")]` for non-optional fields
    let filename = "src/flow_generator/protocol_logs/mq/pulsar.proto.rs";
    let content = std::fs::read_to_string(filename)?;
    let lines = content.lines().collect::<Vec<_>>();
    let mut new_lines = Vec::new();
    new_lines.push(*lines.get(0).unwrap());
    for a in lines.windows(2) {
        if a[1].contains("skip_serializing_if") && !a[0].contains("optional") {
            continue;
        }
        new_lines.push(a[1]);
    }
    std::fs::write(filename, new_lines.join("\n"))?;
    Ok(())
}
fn main()-> Result<()> {
    set_build_info()?;
    /*
    * The protoc binary is too old (3.12) in rust-build image, which cannot handle optional fields in protobuf v3 correctly.
    * And it's not easy to upgrade because of the EOL issue of Centos7.
    * We are pushing the generated protobuf code to repo as a workaround.
    *
    * TODO: Fix this issue in the rust-build image.
    *
   compile_wasm_plugin_proto()?;
    */
    make_pulsar_proto()?;
    Ok(())
}