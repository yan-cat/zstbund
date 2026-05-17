use clap::{Parser, Subcommand};
use std::{path::PathBuf};
use anyhow::Result;
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bundle { packages, output } => {
            bundle_packages(&packages, &output)?;
        }
    }

    Ok(())
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

    let _ = pacman.status()?;
    let _ = zip.status()?;
    println!("打包成功！");

    Ok(())
}
