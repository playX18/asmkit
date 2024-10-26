use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

static RISCV_OPCODES: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = OUT_DIR.join("riscv-opcodes");
    if path.exists() {
        return path;
    }

    git2::Repository::clone("https://github.com/riscv/riscv-opcodes", &path).unwrap();
    path
});

fn run(cmd: &str, args: &[impl AsRef<str>]) {
    std::process::Command::new(cmd)
        .env("RISCV_OPCODES", (*RISCV_OPCODES).to_str().unwrap())
        .args(args.iter().map(|x| x.as_ref()))
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn mv(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    let to = to.as_ref();
    let from = from.as_ref().canonicalize().unwrap();
    eprintln!("cp {}->{}", from.display(), to.display());

    std::fs::copy(&from, to).unwrap();
    std::fs::remove_file(from).unwrap();
}

static OUT_DIR: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from(std::env::var("OUT_DIR").unwrap()));

fn build_riscv() {
    println!("cargo::rerun-if-changed=meta/parse.py");
    let args = vec!["meta/parse.py", "-rust", "rv64*", "rv_*"];

    run("python", &args);
    eprintln!("{}", OUT_DIR.display());
    mv("inst.rs", OUT_DIR.join("inst_rv.rs"));
    std::fs::remove_file("inst_impl.rs").unwrap();
    println!(
        "cargo::rerun-if-changed={}",
        OUT_DIR.join("inst_rv.rs").display()
    );
}

/// Build X86/X64 encoder.
///
/// Uses fadec table and modified fadec parseinstrs.py
fn build_x86() {
    println!("cargo::rerun-if-changed=meta/fadec.py");
    let out_pub = OUT_DIR.join("inst_pub.rs");
    let out_priv = OUT_DIR.join("inst_x86_v1.rs");
    let out_pub = out_pub.to_str().unwrap();
    let out_priv = out_priv.to_str().unwrap();

    let args = vec![
        "meta/fadec.py",
        "--64",
        "--32",
        "encode",
        "meta/instrs.txt",
        out_pub,
        out_priv,
    ];

    run("python", &args);
}

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    build_riscv();
    if std::env::var("REBUILD_X86").is_ok() {
        build_x86();
    }
}
