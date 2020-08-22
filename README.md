# lmake_version_from_date  

[comment]: # (lmake_readme cargo.toml data start)
version: 0.3.37  date: 2020-05-21 authors: Luciano Bestia  
**In cargo.toml and service_worker.js writes the version as the date.**

[comment]: # (lmake_readme cargo.toml data end)  

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-278-green.svg)](https://github.com/LucianoBestia/lmake_version_from_date/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-69-blue.svg)](https://github.com/LucianoBestia/lmake_version_from_date/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-73-purple.svg)](https://github.com/LucianoBestia/lmake_version_from_date/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/lmake_version_from_date/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-11-orange.svg)](https://github.com/LucianoBestia/lmake_version_from_date/)

[comment]: # (lmake_lines_of_code end)

In cargo.toml writes the version as the date `yyyy.mmdd.HHMM` ex. `2019.1221.2359`.  
For non-library projects, the semver specification is not really useful.  
Having the version as the date is just fine for executables and much more human readable.  
The util exe must be executed in the root project folder where is the cargo.toml.  

## service_worker.js

Inside the PWA service worker javascript file is also needed to change the version.  
The program searches for `service_worker.js` and modify the version.  

## no need to change version if no files changed

If src/*.rs or cargo.tom. files are not changed from last compile,
than no need to change version.  
This happend is workspaces when one project is modified and the others are not.  
I need to store the dates somewhere.  
Probably the Target folder is ok. The filename will be lmakeversionfromdate.json.
Warning: I don't check if the service worker has changed because it rarely does.  

## Install
																		  
																						   
																				   

`cargo install lmake_version_from_date`  

## Makefile.toml for cargo-make  

In `Makefile.toml` for `cargo make` add a task like this:  

```toml
[tasks.dev]
description = "cargo build release"
clear = true
dependencies = [
    "lmake_version_from_date",
    "build_release",
    "post_build",
]

[tasks.lmake_version_from_date]
clear = true
private = true
description = "in cargo.toml change version to today date"
script= ["lmake_version_from_date"]
```

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  
