//! **lmake_version_from_date - In cargo.toml and service_worker.js writes the version as the date**  
//region: lmake_readme insert "readme.md"

//endregion: lmake_readme insert "readme.md"

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
//endregion
#![allow(unused_imports)]

//region: use statements
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::offset::Utc;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Timelike;
use chrono::{Datelike, Local};
use unwrap::unwrap;
//use ansi_term::Style;
use clap::{App, Arg};
use filetime::FileTime;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs;
//use std::path::Path; //PathBuf
//endregion

///file metadata
#[derive(Serialize, Deserialize)]
struct FileMetaData {
    //filename with path from cargo.toml folder
    filename: String,
    //filedate from file
    filedate: String,
}

///the struct that is in the file lmakeversionfromdate.json
#[derive(Serialize, Deserialize)]
struct LmakeVersionFromDate {
    ///vector of file metadata
    vec_file_metadata: Vec<FileMetaData>,
}

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here. No arguments.
fn main() {
    //define the CLI input line parameters using the clap library
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("service_worker_filename").takes_value(true))
        .get_matches();

    let mut js_filename = "";
    if let Some(service_worker_filename) = matches.value_of("service_worker_filename") {
        println!(
            "Value for service_worker_filename: {}",
            service_worker_filename
        );
        js_filename = service_worker_filename;
    }

    let current_dir = unwrap!(env::current_dir());

    println!(
        "current_dir: {}",
        Yellow.paint(unwrap!(current_dir.to_str()))
    );

    let mut is_files_equal = true;

    //find lmakeversionfromdate.json
    let json_filepath = "lmakeversionfromdate.json";
    let js_struct: LmakeVersionFromDate;
    let f = fs::read_to_string(json_filepath);
    match f {
        Ok(x) => {
            println!("reading from {}", json_filepath);
            //read struct from file
            js_struct = unwrap!(serde_json::from_str(x.as_str()));
        }
        Err(_error) => {
            println!("file does not exist: {}", Red.paint(json_filepath));
            //create empty struct
            js_struct = LmakeVersionFromDate {
                vec_file_metadata: Vec::new(),
            }
        }
    };
    //make a vector of files
    let mut vec_of_metadata: Vec<FileMetaData> = Vec::new();
    //the cargo.toml and in the folder rs
    let filename = "cargo.toml".to_string();
    let metadata = unwrap!(fs::metadata(filename.as_str()));
    let mtime = FileTime::from_last_modification_time(&metadata);
    let filedate = format!("{}", mtime);
    vec_of_metadata.push(FileMetaData { filename, filedate });

    //println!("fs: {}", serde_json::to_string(&v).unwrap());

    let src_dir = format!("{}/src", unwrap!(current_dir.to_str()));
    for entry in fs::read_dir(src_dir).unwrap() {
        let entry = unwrap!(entry);
        let path = entry.file_name();

        let filename = format!("src/{:?}", path);
        let filename = filename.replace("\"", "");
        //println!("filename: {}", &filename);
        let metadata = unwrap!(fs::metadata(filename.as_str()));
        let mtime = FileTime::from_last_modification_time(&metadata);
        let filedate = format!("{}", mtime);
        vec_of_metadata.push(FileMetaData { filename, filedate });
    }

    //println!("fs: {}", serde_json::to_string(&v).unwrap());

    //if files are added or deleted, other files must be also changed
    //I need to check if the files on the filesystem are the same as in the json
    for x in &vec_of_metadata {
        //search in json file
        let mut is_equal = false;
        for y in &js_struct.vec_file_metadata {
            if x.filename == y.filename && x.filedate == y.filedate {
                is_equal = true;
                break;
            } else {
                //println!("{} {}\n", y.filename, y.filedate);
            }
        }
        if is_equal == false {
            println!("{} {}", x.filename, x.filedate);
            is_files_equal = false;
            break;
        }
    }

    println!("is_files_equal: {}", is_files_equal);

    if !is_files_equal {
        let date = Local::now();
        let new_version = format!(
            "{}.{}.{}-{}.{}",
            date.year() - 2000,
            date.month(),
            date.day(),
            date.hour(),
            date.minute()
        );

        //region: write version in cargo.toml
        {
            println!("{}", Green.paint("write version in cargo.toml"));
            //find version in cargo.toml
            let cargo_filename = "cargo.toml";
            let mut cargo_content = unwrap!(fs::read_to_string(cargo_filename));
            let delim = r#"version = ""#;
            let delim_len = delim.len();
            let option_location = cargo_content.find(delim);
            if let Some(location) = option_location {
                let start_version = location + delim_len;
                let option_end_quote = find_from(cargo_content.as_str(), start_version, r#"""#);
                if let Some(end_version) = option_end_quote {
                    //delete all the characters in between the markers
                    let old_version: String =
                        cargo_content.drain(start_version..end_version).collect();
                    println!(r#"old version: "{}""#, old_version.as_str());
                    if new_version != old_version {
                        println!("new_version {}", new_version);
                        cargo_content.insert_str(start_version, new_version.as_str());
                        println!("write file: {}", Yellow.paint(cargo_filename));
                        let _x = fs::write(cargo_filename, cargo_content);
                        //the cargo.toml is now different

                        //correct the vector
                        let filename = "cargo.toml".to_string();
                        let metadata = unwrap!(fs::metadata(filename.as_str()));
                        let mtime = FileTime::from_last_modification_time(&metadata);
                        let filedate = format!("{}", mtime);
                        unwrap!(vec_of_metadata.get_mut(0)).filedate = filedate;

                        println!("save the new file metadata");
                        let x = LmakeVersionFromDate {
                            vec_file_metadata: vec_of_metadata,
                        };
                        let y = unwrap!(serde_json::to_string(&x));
                        let _f = fs::write(json_filepath, y);
                    }
                } else {
                    panic!("no end quote for version");
                }
            } else {
                panic!("cargo.toml has no version");
            }
        }
        //endregion
        //region: write version in service_worker.js
        if js_filename != "" {
            println!("{}", Green.paint("write version in service_worker.js"));
            let js_filename = js_filename;
            let mut js_content = unwrap!(fs::read_to_string(js_filename));
            let delim = r#"const CACHE_NAME = '"#;
            let delim_len = delim.len();
            let option_location = js_content.find(delim);
            if let Some(location) = option_location {
                let start_version = location + delim_len;
                let option_end_quote = find_from(js_content.as_str(), start_version, r#"';"#);
                if let Some(end_version) = option_end_quote {
                    //delete all the characters in between the markers
                    let old_version: String =
                        js_content.drain(start_version..end_version).collect();
                    println!(r#"old version: "{}""#, old_version.as_str());
                    if new_version != old_version {
                        println!("new_version {}", new_version);
                        js_content.insert_str(start_version, new_version.as_str());
                        println!("write file: {}", Yellow.paint(js_filename));
                        let _x = fs::write(js_filename, js_content);
                    }
                } else {
                    panic!("no end quote for version");
                }
            } else {
                panic!("service_worker.js has no version");
            }
        }
        //endregion
    }
}

#[allow(clippy::integer_arithmetic)]
///find from
fn find_from(rs_content: &str, from: usize, find: &str) -> Option<usize> {
    let slice01 = rs_content.get(from..).unwrap();
    let option_location = slice01.find(find);
    if let Some(location) = option_location {
        //return Option with usize
        Some(from + location)
    } else {
        //return Option with none
        option_location
    }
}
