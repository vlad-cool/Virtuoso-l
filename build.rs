use glob::glob;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::process::Command;
use std::vec::Vec;

fn generate_licenses() {
    match Command::new("cargo")
        .args(["license", "--avoid-dev-deps", "--all-features", "--json"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let json: String = String::from_utf8_lossy(&output.stdout).to_string();
            fs::write("licenses.json", json).unwrap();
        }
        Ok(_) => {
            println!("cargo:warning=`cargo-license` failed to run or returned error. Is it inst");
        }
        Err(_) => {
            println!(
                "cargo:warning=`cargo-license` not installed. Run: cargo install cargo-license"
            );
        }
    }
}

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
struct PositionData {
    #[serde(rename = "fontSize")]
    font_size: Option<String>,
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
    #[serde(default)]
    shapes: Vec<String>,
    #[serde(rename = "positionData")]
    position_data: Option<Vec<PositionData>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Color {
    color: String,
    name: String,
}

fn float_to_rounded_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let val: f64 = f64::deserialize(deserializer)?;
    Ok(val as u32)
}

fn float_to_rounded_u16<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let val: f64 = f64::deserialize(deserializer)?;
    Ok(val as u16)
}

fn float_to_rounded_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let val: f64 = f64::deserialize(deserializer)?;
    Ok(val as i32)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LayoutText {
    id: String,
    #[serde(deserialize_with = "float_to_rounded_i32")]
    x: i32,
    #[serde(deserialize_with = "float_to_rounded_i32")]
    y: i32,
    #[serde(deserialize_with = "float_to_rounded_u32")]
    width: u32,
    #[serde(deserialize_with = "float_to_rounded_u32")]
    height: u32,
    #[serde(deserialize_with = "float_to_rounded_u16")]
    font_size: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct LayoutRect {
    id: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    radius: u32,
}

fn parse_layout(main_id: &String, path: String, writer: &mut BufWriter<File>) {
    let file: File =
        File::open(format!("mockup/extracted/files/{main_id}/pages/{path}.json").as_str())
            .expect("cargo::error=Failed to open page file");
    let reader: BufReader<File> = BufReader::new(file);
    let page: PenpotFile =
        serde_json::from_reader(reader).expect("cargo::error=Failed to parse page file");

    let name: String = page.name;

    if name == "layout_480x320" {
        return;
    }

    let file: File = File::open(format!("mockup/extracted/files/{main_id}/pages/{path}/00000000-0000-0000-0000-000000000000.json").as_str())
        .expect("cargo::error=Failed to open root node file");
    let reader: BufReader<File> = BufReader::new(file);
    let root_node: PenpotNode =
        serde_json::from_reader(reader).expect("cargo::error=Failed to parse root node file");

    writer
        .write(
            format!(
                "
pub const {}: Layout = Layout {{\n",
                name.to_ascii_uppercase()
            )
            .as_bytes(),
        )
        .expect("cargo::error=Failed to write to file");

    for node_id in root_node.shapes {
        let file: File = File::open(
            format!("mockup/extracted/files/{main_id}/pages/{path}/{node_id}.json").as_str(),
        )
        .expect("cargo::error=Failed to open child node file");
        let reader: BufReader<File> = BufReader::new(file);
        let node: PenpotNode =
            serde_json::from_reader(reader).expect("cargo::error=Failed to parse child node file");

        let node_type: &str = node.node_type.as_str();

        if node.name == "Rectangle" {
            continue;
        }

        let position_data: Option<Vec<PositionData>> = node.position_data.clone();

        match node_type {
            "text" => {
                writer
                    .write(
                        format!(
                            "    {}: TextProperties {{
        x: {},
        y: {},
        width: {},
        height: {},
        font_size: {},
    }},
",
                            node.name,
                            node.x as i32,
                            node.y as i32,
                            node.width as u32,
                            node.height as u32,
                            position_data
                                .clone()
                                .expect("cargo::error=No position data in text node")
                                .first()
                                .cloned()
                                .expect("cargo::error=Position data is empty in text node")
                                .font_size
                                .expect("cargo::error=No font size in position data")
                                .replace("px", "")
                        )
                        .as_bytes(),
                    )
                    .expect("cargo::error=Failed to write to file");
            }
            "rect" => {
                writer
                    .write(
                        format!(
                            "    {}: RectangleProperties {{
        x: {},
        y: {},
        width: {},
        height: {},
        radius: {},
    }},
",
                            node.name,
                            node.x as i32,
                            node.y as i32,
                            node.width as u32,
                            node.height as u32,
                            node.rx as u32,
                        )
                        .as_bytes(),
                    )
                    .expect("cargo::error=Failed to write to file");
            }
            "frame" => {}

            t => {
                println!("cargo::warning=Unknown node type: {t}")
            }
        }
    }

    writer
        .write("};\n".as_bytes())
        .expect("cargo::error=Failed to write to file");
}

fn parse_penpot() {
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

    let file: File =
        File::create("src/sdl_frontend/layouts.rs").expect("Failed to open file for writing");

    let mut writer: BufWriter<File> = BufWriter::new(file);

    let header: String = format!(
        "// Generated file
use crate::sdl_frontend::layout_structure::*;
"
    );

    writer
        .write(header.as_bytes())
        .expect("cargo::error=Failed to write to file");

    for entry in glob(format!("mockup/extracted/files/{virtuoso_file_id}/pages/*.json").as_str())
        .expect("cargo::error=Failed to read glob pattern")
    {
        match entry {
            Ok(path) => {
                let name: std::borrow::Cow<'_, str> = path.file_name().unwrap().to_string_lossy();
                let name: String = name.split(".").next().unwrap().to_string();

                // writer.write(buf);
                parse_layout(&virtuoso_file_id, name, &mut writer);
            }
            Err(e) => {
                println!("cargo::error=Error processing json file: {:?}", e);
                panic!()
            }
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=penpot parsing disabled");
    generate_licenses();
    // parse_penpot();
}
