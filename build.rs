use glob::glob;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use std::vec::Vec;

fn extract_zip(src_path: &str, dest_dir: &str) {
    let archive: fs::File = fs::File::open(src_path).expect("Failed to open penpot archibe");

    let mut archive: zip::ZipArchive<fs::File> =
        zip::ZipArchive::new(archive).expect("Failed to create archive object");
    archive
        .extract(dest_dir)
        .expect("Failed to extract penpot archive");
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PenpotFile {
    id: String,
    name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PenpotManifest {
    version: u32,
    files: Vec<PenpotFile>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct PenpotNode {
    id: String,
    name: String,
    #[serde(rename = "type")]
    node_type: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    #[serde(default)]
    rx: f64,
    shapes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Color {
    color: String,
    name: String,
}

fn parse_layout(main_id: &String, path: String) {
    let file: File =
        File::open(format!("mockup/extracted/files/{main_id}/pages/{path}.json").as_str())
            .expect("cargo::error=Failed to open page file");
    let reader: BufReader<File> = BufReader::new(file);
    let page: PenpotFile =
        serde_json::from_reader(reader).expect("cargo::error=Failed to parse page file");

    let name: String = page.name;
    // println!("cargo::warning=OPENING mockup/extracted/files/{main_id}/pages/{path}/00000000-0000-0000-0000-000000000000.json");

    let file: File = File::open(format!("mockup/extracted/files/{main_id}/pages/{path}/00000000-0000-0000-0000-000000000000.json").as_str())
        .expect("cargo::error=Failed to open root node file");
    let reader: BufReader<File> = BufReader::new(file);
    let root_node: PenpotNode =
        serde_json::from_reader(reader).expect("cargo::error=Failed to parse root node file");

    // println!("cargo::warning={:?}", root_node.shapes);
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=mockup/Virtuoso.penpot");

    let archive_path: &'static str = "mockup/Virtuoso.penpot";
    let extract_dir: &'static str = "mockup/extracted";

    if !Path::new(extract_dir).exists() {
        fs::create_dir_all(extract_dir)
            .expect("cargo::error=Failed to create extraction directory");
    }

    extract_zip(archive_path, extract_dir);

    let file: File = File::open("mockup/extracted/manifest.json")
        .expect("cargo::error=Failed to open manifest file");
    let reader: BufReader<File> = BufReader::new(file);
    let manifest: PenpotManifest =
        serde_json::from_reader(reader).expect("cargo::error=Failed to parse manifest file");

    println!("cargo::warning={manifest:?}");

    let mut virtuoso_file_id: Option<String> = None;
    for file in manifest.files {
        if file.name == "Virtuoso" {
            virtuoso_file_id = Some(file.id);
        }
    }

    let virtuoso_file_id: String = if let Some(id) = virtuoso_file_id {
        id
    } else {
        println!("cargo::error=Failed to find file in manifest with name Virtuoso");
        panic!()
    };

    for entry in glob(format!("mockup/extracted/files/{virtuoso_file_id}/pages/*.json").as_str())
        .expect("Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let name: std::borrow::Cow<'_, str> = path.file_name().unwrap().to_string_lossy();
                let name: String = name.split(".").next().unwrap().to_string();

                parse_layout(&virtuoso_file_id, name);
            }
            Err(e) => {
                println!("cargo::error=Error processing json file: {:?}", e);
                panic!()
            }
        }
    }
}
