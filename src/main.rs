use clap::{Parser, Subcommand};
use std::{path::PathBuf};
use anyhow::{Result, anyhow};
use std::process::Command;
use tempfile::TempDir;


#[derive(Parser)]
#[command(name = "zstbund")]
#[command(about = "打包pacman软件包及其依赖以供离线使用", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 打包一个或多个软件包及其依赖成zip文件，包含gpg密钥
    Bundle {
        /// 要打包的包名（多个）
        #[arg(required = true)]
        packages: Vec<String>,
        /// 输出 zip 文件的路径（默认 ./bundle.zip）
        #[arg(short, long, default_value = "bundle.zsts.zip")]
        output: PathBuf,
    },
    /// 安装一个打包完成的zst包组
    Install {
        /// 要安装的包的路径
        #[arg(required = true)]
        packages: PathBuf
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bundle { packages, output } => {
            bundle_packages(&packages, &output)?;
        }
        Commands::Install { packages } => {
            install_zsts(&packages )?;
        }
    }

    Ok(())
}

fn run_cmd(mut cmd: std::process::Command, name: &str) -> Result<()> {
    match cmd.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(anyhow!("{} 执行失败，退出码：{:?}", name, status.code())),
        Err(e) => Err(anyhow!("无法启动 {}：{}", name, e)),
    }
}

fn bundle_packages(packages: &[String], output: &PathBuf) -> Result<()> {
    if output == "bundle.zip" {
        println!("您似乎没有指定输出文件，默认输出为运行目录下的bundle.zsts.zip");
    }
    println!("开始打包: {:?}", packages);

    let temp_dir = TempDir::new()?;
    let tmp_path = temp_dir.path();

    println!("tmppath:{}\n",tmp_path.display());

    let mut pacman = Command::new("sudo");
    pacman.arg("pacman")
    .arg("-Syw")
    .arg("--dbpath").arg("/tmp")
    .arg("--cachedir").arg(tmp_path)
    .arg("--noconfirm")
    .args(packages);

    let mut zip = Command::new("zip");
    zip.arg("-j")
    .arg("-r")
    .arg(output)
    .arg(tmp_path);

    run_cmd(pacman, "pacman 下载")?;
    run_cmd(zip, "zip 打包")?;
    println!("打包成功！");

    Ok(())
}

fn install_zsts(packages: &PathBuf) -> Result<()>{
    println!("开始安装: {:?}", packages);

    let temp_dir = TempDir::new()?;
    let tmp_path = temp_dir.path();

    println!("tmppath:{}\n",tmp_path.display());

    let mut unzip = Command::new("unzip");
    unzip.arg(packages)
    .arg("-d")
    .arg(tmp_path);

    let mut pacman = Command::new("sudo");
    pacman.arg("find")
    .arg(tmp_path)
    .arg("-maxdepth").arg("1")
    .arg("-name").arg("*.pkg.tar.zst")
    .arg("-exec").arg("pacman").arg("-U").arg("{}").arg("+");

    let _ = unzip.status()?;
    let _ = pacman.status()?;
    run_cmd(unzip, "zip 解压")?;
    run_cmd(pacman, "pacman 安装")?;
    println!("安装完成！");

    Ok(())
}
