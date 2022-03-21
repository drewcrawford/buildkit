/*!
# buildkit

The world's tiniest real buildsystem.  You might be looking for this crate if you are deciding whether to build some non-Rust
dependency by hand in `build.rs`, or wrestle the [cc](https://docs.rs/cc/1.0.71/cc/) crate in some way.

## Compared with `cc`:

1.  Buildkit implements the build system only; you supply the compiler by implementing some trait (that e.g. shells out to a compiler).
    The key is that this could be any command, not just C compilers.  Typical usecases for buildkit are building non-C programs,
    compiling GPU shaders, compiling art or sound assets, etc.
2.  For this reason, the ordinary way to use buildkit is indirectly through some other crate that knows how to do what you want to do
    (or writing such a crate).
    In particular, if you're looking for a `cc` replacement, you want that other higher-level crate.
3.  Skips builds if no sourcefiles were changed.  Other features like advanced incremental builds and parallel builds are planned.

## Compared with writing shell yourself:

1.  Buildkit skips builds if no sourcefiles were changed.  Other features like advanced incremental builds and parallel builds are planned.
2.  Easy integration with `build.rs`, debug vs release profiles, where to locate intermediate object files, etc
3.  Modeling both compile (1 sourcefile per output) and link (many sourcefiles per output).

# Usage

## Compile-side

To bring your own compiler, do
```
use buildkit::{CompileStep,CompileSystem,Configuration};
use std::path::{PathBuf,Path};
struct MyCompiler;
impl CompileStep for MyCompiler {
    const SOURCE_FILE_EXTENSION: &'static str = "mylang";
    fn compile_one<'a>(path: &Path, intermediate_dir: &Path, configuration: &Configuration, dependency_path: &Path,flags: impl Iterator<Item=&'a String>) -> PathBuf {
        //shell out to compiler here
        todo!()
    }
}
fn build_rs(exe_path: PathBuf) {
    let built_files = CompileSystem::<MyCompiler>::build_rs(exe_path);
}
```

Implement [LinkStep] if your usecase involves linking multiple compile files into a single result.

## Use-side

Call [CompileSystem::build_rs] from a `build.rs` file.  See documentation for additional entrypoints and options.


 */

mod build_settings;
mod traits;
mod build_system;
mod dependency_parser;
mod compile_system;
mod compile_settings;

pub use build_settings::{BuildSettings,BuildSettingsBuilder,Configuration,PathType};
pub use compile_settings::{CompileSettings,SourceFileStrategy,CompileSettingsBuilder};
pub use build_system::BuildSystem;
pub use compile_system::CompileSystem;
pub use traits::{CompileStep,LinkStep,suggest_intermediate_file};