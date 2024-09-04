use std::env;
use std::path::PathBuf;
use std::fs;
use std::process::Command;
use flate2::read::GzDecoder;
use tar::Archive;

fn mkdir()->(PathBuf,PathBuf) {
    // 设置SRT仓库的目标路径和安装路径
    let srt_source_path = PathBuf::from("depends/build");
    let srt_install_path = PathBuf::from("depends/srt");
    // 确保目录存在
    fs::create_dir_all(&srt_source_path).expect("Failed to create SRT source directory");
    fs::create_dir_all(&srt_install_path).expect("Failed to create SRT install directory");

    println!("SRT source path: {}", srt_source_path.display());
    println!("SRT install path: {}", srt_install_path.display());

    (srt_source_path.canonicalize().unwrap(),srt_install_path.canonicalize().unwrap())
}

fn download_srt_source() -> PathBuf {
    let (srt_source_path, _) = mkdir();

    // 将解压后的目录移动到 srt 目录
    let srt_dir = srt_source_path.join("srt");
    if srt_dir.exists() {
        return srt_dir.canonicalize().unwrap();
    }
    println!("下载 SRT 源码...");
    let url = "https://github.com/Haivision/srt/archive/refs/tags/v1.5.3.tar.gz";
    let resp = reqwest::blocking::get(url).expect("无法下载 SRT 源码");
    let tar = GzDecoder::new(resp);
    let mut archive = Archive::new(tar);

    // 创建临时目录用于解压
    let temp_dir = srt_source_path.join("temp");
    std::fs::create_dir_all(&temp_dir).expect("无法创建临时目录");

    // 解压到临时目录
    archive.unpack(&temp_dir).expect("无法解压 SRT 源码");

    // 找到解压后的目录（通常是 "srt-1.5.3"）
    let extracted_dir = std::fs::read_dir(&temp_dir)
        .expect("无法读取临时目录")
        .filter_map(Result::ok)
        .find(|entry| entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
        .expect("无法找到解压后的目录")
        .path();

    std::fs::rename(extracted_dir, &srt_dir).expect("无法移动解压后的目录");

    // 清理临时目录
    std::fs::remove_dir_all(&temp_dir).expect("无法删除临时目录");

    println!("SRT 源码已解压到 srt 目录");
    return srt_dir.canonicalize().unwrap();
}

fn compile_srt_lib() {
    let (_, srt_install_path) = mkdir();

    // 检查是否已经编译成功
    let lib_path = srt_install_path.join("lib").join("libsrt.a");
    let include_path = srt_install_path.join("include").join("srt");

    if lib_path.exists() && include_path.exists() {
        println!("SRT 库已经编译，跳过编译步骤");
        return;
    }

    // 编译并安装SRT
    let srt_dir = download_srt_source();
    let srt_build_dir = srt_dir.join("build");
    std::fs::create_dir_all(&srt_build_dir).expect("无法创建SRT构建目录");

    // 在 CMake 配置中使用 pkg-config 的输出
    Command::new("cmake")
        .current_dir(&srt_build_dir)
        .args(&[
            "..",
            &format!("-DCMAKE_INSTALL_PREFIX={}", srt_install_path.to_str().unwrap()),
            "-DENABLE_SHARED=OFF",
            "-DENABLE_STATIC=ON",
            "-DUSE_STATIC_LIBSTDCXX=ON",
            "-DUSE_ENCLIB=openssl",
            "-DENABLE_CXX11=ON",
            "-DCMAKE_POSITION_INDEPENDENT_CODE=ON",
            // "-DCMAKE_C_FLAGS=-pie",
            // "-DCMAKE_CXX_FLAGS=-pie",
            // "-DCMAKE_EXE_LINKER_FLAGS=-pie",
            // "-DCMAKE_SHARED_LINKER_FLAGS=-pie",
        ])
        .status()
        .expect("CMake配置失败");

    Command::new("cmake")
        .current_dir(&srt_build_dir)
        .args(&["--build", ".", "--config", "Release"])
        .status()
        .expect("CMake构建失败");

    Command::new("cmake")
        .current_dir(&srt_build_dir)
        .args(&["--install", "."])
        .status()
        .expect("CMake安装失败");

    println!("SRT 库编译完成");
}

fn main() {
    let  (_, srt_install_path) = mkdir();
    compile_srt_lib();

    println!("cargo:rustc-link-lib=c++");
    // 或者，如果使用 libc++：
    // println!("cargo:rustc-link-lib=c++abi");
    // 设置链接路径
    println!("cargo:rustc-link-search=native={}/lib", srt_install_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=srt");

    // 告诉cargo在SRT源码或wrapper.h发生变化时重新运行此脚本
    println!("cargo:rerun-if-changed=depends/build/srt");
    println!("cargo:rerun-if-changed=wrapper.h");

    

    // SRT头文件的路径
    let srt_include_path = srt_install_path.join("include");

    // 使用bindgen生成绑定
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", srt_include_path.display()))
        .size_t_is_usize(true)
        // .whitelist_function("srt_.*")
        // .whitelist_type("SRT.*")
        // .whitelist_var("SRT.*")
        .bitfield_enum("SRT_EPOLL_OPT")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // 忽略 IPPORT_RESERVED 的定义
        .blocklist_item("IPPORT_RESERVED")
        .generate()
        .expect("无法生成绑定");

    // 将生成的绑定写入文件
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("无法写入绑定");

}
