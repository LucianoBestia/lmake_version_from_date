# lmake_version_from_date  
In cargo.toml writes the version as the date `yy.mm.dd` ex. `19.12.31`.  
For non-library projects, the semver specification is not really useful.  
Having the version as the date is just fine for executables and much more human readable.  
The util exe must be executed in the root project folder where is the cargo.toml.  
No arguments needed to execute the exe.  
## Makefile.toml for cargo make  
In `Makefile.toml` for `cargo make` add a call like this:  
```toml
[tasks.dev]
description = "cargo build development"
clear = true
dependencies = [
    "lmake_version_from_date",
    "build-dev",
    "post-build",
]

[tasks.lmake_version_from_date]
clear = true
private = true
description = "in cargo.toml change version to today date"
script= ["..\\utils\\lmake_version_from_date.exe"]
```


