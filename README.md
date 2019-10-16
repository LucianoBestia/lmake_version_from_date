# lmake_version_from_date  

In cargo.toml writes the version as the date `yy.mm.dd` ex. `19.12.31`.  
But the date is not enought for debugging so I add also `-hh.MM` like `-23.59`.
For non-library projects, the semver specification is not really useful.  
Having the version as the date is just fine for executables and much more human readable.  
The util exe must be executed in the root project folder where is the cargo.toml.  

## service_worker.js

Inside the PWA service worker javascript file is also needed to change the version.  
This is done with the first argument like this:  
`../../utils_linux/lmake_version_from_date --js ../webfolder/mem5/service_worker.js`  

## no need to change version if no files changed

If src/*.rs or cargo.tom. files are not changed from last compile,
than no need to change version. I need to store the dates somewhere.  
Probably the Target folder is ok. The filename will be lmakeversionfromdate.json.
Warning: I don't check if the service worker has changed because it rarely does.  

## Linux vs. Windows

I will never build a Rust project exclusively for Windows.  
The server projects will run on Linux web servers.  
The frontend will be in Wasm/Webassembly that is OS agnostic.  
So it makes sense to always build projects for Linux.  
It is now possible easily because of Linux subsystem for windows.  
All utils I use in the building process must be also Linux executables.  

## Makefile.toml for cargo make  

In `Makefile.toml` for `cargo make` add a call like this:  

```toml
[tasks.dev]
description = "cargo build development"
clear = true
dependencies = [
    "lmake_version_from_date",
    "build_dev",
    "post_build",
]

[tasks.lmake_version_from_date]
clear = true
private = true
description = "in cargo.toml change version to today date"
script= ["..\\utils\\lmake_version_from_date.exe"]
```
