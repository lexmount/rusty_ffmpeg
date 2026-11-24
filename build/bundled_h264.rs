use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

const VERSION: &str = "8.1.3";
const LIBRARIES: [&str; 4] = ["avformat", "avcodec", "swscale", "avutil"];

pub fn link(out_dir: &Path) {
    let archive = format!("vendor/ffmpeg-{VERSION}-h264.tar.xz");
    println!("cargo:rerun-if-changed={archive}");
    println!("cargo:rerun-if-env-changed=NASM");
    println!("cargo:rerun-if-env-changed=MAKE");

    let root = Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap()).join(archive);
    run(Command::new("tar")
        .arg("-xJf")
        .arg(root)
        .arg("-C")
        .arg(out_dir));

    let build_dir = out_dir.join("ffmpeg-build");
    fs::create_dir_all(&build_dir).expect("create FFmpeg build directory");
    let toolchain = cc::Build::new()
        .opt_level(2)
        .debug(false)
        .warnings(false)
        .get_compiler();
    let mut configure = Command::new(out_dir.join(format!("ffmpeg-{VERSION}/configure")));
    configure.current_dir(&build_dir).args([
        "--disable-everything",
        "--disable-programs",
        "--disable-doc",
        "--disable-network",
        "--disable-autodetect",
        "--disable-debug",
        "--disable-shared",
        "--enable-static",
        "--enable-pic",
        "--disable-avdevice",
        "--disable-avfilter",
        "--disable-swresample",
        "--disable-iconv",
        "--enable-decoder=h264",
        "--enable-parser=h264",
        "--enable-demuxer=mov",
    ]);
    configure.arg(format!("--cc={}", toolchain.path().display()));
    configure.arg(format!(
        "--extra-cflags={}",
        toolchain.cflags_env().to_string_lossy()
    ));
    if env::var_os("HOST") != env::var_os("TARGET") {
        configure
            .arg("--enable-cross-compile")
            .arg("--target-os=linux")
            .arg(format!(
                "--arch={}",
                env::var("CARGO_CFG_TARGET_ARCH").unwrap()
            ));
    }
    let assembler = env::var_os("NASM").unwrap_or_else(|| "nasm".into());
    if env::var("CARGO_CFG_TARGET_ARCH").is_ok_and(|arch| arch == "x86" || arch == "x86_64") {
        if Command::new(&assembler)
            .arg("-v")
            .output()
            .is_ok_and(|v| v.status.success())
        {
            configure.arg(format!("--x86asmexe={}", assembler.to_string_lossy()));
        } else {
            configure.arg("--disable-x86asm");
        }
    }
    run(&mut configure);
    let jobs = env::var("NUM_JOBS").unwrap_or_else(|_| "1".into());
    let mut make = Command::new(env::var_os("MAKE").unwrap_or_else(|| "make".into()));
    make.current_dir(&build_dir).arg(format!("-j{jobs}"));
    for library in LIBRARIES {
        make.arg(format!("lib{library}/lib{library}.a"));
    }
    run(&mut make);

    for library in LIBRARIES {
        println!(
            "cargo:rustc-link-search=native={}",
            build_dir.join(format!("lib{library}")).display()
        );
        println!("cargo:rustc-link-lib=static={library}");
    }
    let config = fs::read_to_string(build_dir.join("ffbuild/config.mak"))
        .expect("read FFmpeg library dependencies");
    let mut dependencies = BTreeSet::new();
    for library in LIBRARIES {
        let key = format!("EXTRALIBS-{library}=");
        if let Some(flags) = config.lines().find_map(|line| line.strip_prefix(&key)) {
            dependencies.extend(flags.split_whitespace().filter_map(|flag| {
                if flag == "-pthread" {
                    Some("pthread")
                } else {
                    flag.strip_prefix("-l")
                }
            }));
        }
    }
    for library in dependencies {
        println!("cargo:rustc-link-lib={library}");
    }
}

fn run(command: &mut Command) {
    let output = command
        .output()
        .unwrap_or_else(|error| panic!("could not run {command:?}: {error}"));
    assert!(
        output.status.success(),
        "{command:?} failed with {}\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
