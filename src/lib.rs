/*!
The world's tiniest real buildsystem.  This can be vaguely compared with the [cc](https://docs.rs/cc/1.0.71/cc/) crate, or with
writing shell scripts yourself.

Compared with `cc`:

1.  Buildkit implements the build system only; you supply the compiler by implementing some trait (that e.g. shells out to a compiler).
    The key is that this can be any command, not just C compilers.  Typical workflows for buildkit are building non-C programs,
    compiling GPU shaders, compiling art or sound assets, etc.
2.  For this reason, the ordinary way to use buildkit is indirectly through some other crate.
    In particular, if you're looking for a `cc` replacement, you want that other crate.
3.  Skips builds if no sourcefiles were changed.  Other features like advanced incremental builds and parallel builds are planned.

Compared with writing shell yourself:

1.  Skips builds if no sourcefiles were changed.  Other features like advanced incremental builds and parallel builds are planned.
2.  Easy integration with `build.rs`, debug vs release profiles, where to locate intermediate object files, etc
3.  Modeling both compile (1 sourcefile per output) and link (many sourcefiles per output).


*/

mod build_settings;
mod traits;
mod build_system;
mod dependency_parser;

pub use build_settings::{BuildSettings,SourceFileStrategy,BuildSettingsBuilder,Configuration,PathType};
pub use build_system::BuildSystem;
pub use traits::{CompileStep,LinkStep};