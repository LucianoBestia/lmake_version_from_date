//! **lmake_version_from_date - In cargo.toml writes the version as the date**  
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

//region: extern and use statements
#[macro_use]
extern crate unwrap;
extern crate chrono;
extern crate clap;
extern crate glob;

#[allow(unused_imports)]
use crate::chrono::Timelike;
use ansi_term::Colour::{Green, Red, Yellow};
use chrono::{Datelike, Local};
//use ansi_term::Style;
use clap::App; //Arg

//use glob::glob;
use std::env;
use std::fs;
//use std::path::Path; //PathBuf
//endregion

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here. No arguments.
fn main() {
    //this function is different for Windows and for Linux.
    //Look at the code of this function (2 variations).
    enable_ansi_support();

    //define the CLI input line parameters using the clap library
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let current_dir = unwrap!(env::current_dir());

    println!(
        "current_dir: {}",
        Yellow.paint(unwrap!(current_dir.to_str()))
    );

    //find version in cargo.toml
    let cargo_filename = "cargo.toml";
    let mut cargo_content = unwrap!(fs::read_to_string(cargo_filename));
    let delim = r#"version = ""#;
    let option_location = cargo_content.find(delim);
    if let Some(location) = option_location {
        let start_version = location + 11;
        let option_end_quote = find_from(cargo_content.as_str(), start_version, r#"""#);
        if let Some(end_version) = option_end_quote {
            //delete all the lines in between the markers
            let old_version: String = cargo_content.drain(start_version..end_version).collect();
            println!(r#"old version: "{}""#, old_version.as_str());
            let date = Local::now();
            let new_version = format!("{}.{}.{}-{}.{}", 
            date.year() - 2000, date.month(), date.day(),
            date.hour(), date.minute());
            if new_version != old_version {
                println!("new_version {}", new_version);
                cargo_content.insert_str(start_version, new_version.as_str());
                println!("write file: {}", Yellow.paint(cargo_filename));
                let _x = fs::write(cargo_filename, cargo_content);
            }
        } else {
            panic!("no end quote for version");
        }
    } else {
        panic!("cargo.toml has no version");
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

//region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
///only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    //do nothing
}
//endregion
