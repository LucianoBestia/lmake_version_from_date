# lmake_version_from_date  

[comment]: # (lmake_readme cargo.toml data start)
version: 0.3.32  date: 2020-04-29 authors: Luciano Bestia  
**In cargo.toml and service_worker.js writes the version as the date.**

[comment]: # (lmake_readme cargo.toml data end)  

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
