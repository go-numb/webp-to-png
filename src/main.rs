// ディレクトリ内のWebPファイルをPNGに変換し、ディレクトリ名を変更、不要なファイルを削除するプログラム

// webp-to-png.exe --base-path "D:\Download\@anonymous"

use image::ImageFormat;
use rayon::prelude::*;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;

// 定数
const REMOVE_FIRST_DIRNAME: &str = "";
const REMOVE_SECOND_DIRNAME: &str = "";
const BASE_PATH: &str = "D:/Download/@anonymous/test";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Base path for processing
    #[clap(short, long, value_parser, default_value = BASE_PATH)]
    base_path: PathBuf,
}

fn parse_args() -> Args {
    Args::parse()
}

fn main() -> Result<()> {
    let start = std::time::Instant::now();
    println!("Processing started...");

    let args = parse_args();

    let path = Path::new(args.base_path.as_path());
    let dirs = list_directories(path)?;

    // ディレクトリの処理を並列化
    dirs.par_iter()
        .try_for_each(|dir| process_directory(path, dir))?;

    // ベースディレクトリ内のファイル削除
    remove_files_in_base_directory(path)?;

    println!("Elapsed: {:?}", start.elapsed());
    Ok(())
}

fn process_directory(base_path: &Path, dir: &str) -> Result<()> {
    let dir_path = base_path.join(dir);
    let webp_files = list_webp_files(&dir_path)?;

    // WebPファイルの処理を並列化
    webp_files
        .par_iter()
        .try_for_each(|file| convert_webp_to_png(file))?;

    // ディレクトリ名変更
    let new_dir_name = dir.replace(REMOVE_FIRST_DIRNAME, "");
    let new_dir_name = new_dir_name.replace(REMOVE_SECOND_DIRNAME, "");
    let abs_dir_path = base_path.join(new_dir_name);
    fs::rename(&dir_path, abs_dir_path)?;

    Ok(())
}

fn remove_files_in_base_directory(base_path: &Path) -> Result<()> {
    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !is_valid_filename(file_name) {
                delete_file(&path)?;
            }
        }
    }
    Ok(())
}

fn list_directories(path: &Path) -> Result<Vec<String>> {
    let mut directories = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                directories.push(dir_name.to_string());
            }
        }
    }
    Ok(directories)
}

fn list_webp_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut webp_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !is_valid_filename(file_name) {
                if path.extension().map_or(false, |ext| ext == "png") {
                    println!("Skipping PNG file: {:?}", path);
                    continue;
                }
                delete_file(&path)?;
            } else if path.extension().map_or(false, |ext| ext == "webp") {
                webp_files.push(path);
            }
        }
    }
    Ok(webp_files)
}

fn is_valid_filename(filename: &str) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9]+\.webp$").unwrap();
    re.is_match(filename)
}

fn convert_webp_to_png(webp_path: &Path) -> Result<()> {
    let img = image::open(webp_path)?;
    let png_path = webp_path.with_extension("png");
    img.save_with_format(&png_path, ImageFormat::Png)?;
    delete_file(webp_path)?;
    println!("Converted {:?} to {:?}", webp_path, png_path);
    Ok(())
}

fn delete_file(path: &Path) -> Result<()> {
    fs::remove_file(path)?;
    println!("Deleted {:?}", path);
    Ok(())
}

// カスタムエラー型
#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ImageError(image::ImageError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO error: {}", e),
            AppError::ImageError(e) => write!(f, "Image error: {}", e),
        }
    }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<image::ImageError> for AppError {
    fn from(error: image::ImageError) -> Self {
        AppError::ImageError(error)
    }
}

// 型エイリアス
type Result<T> = std::result::Result<T, AppError>;
