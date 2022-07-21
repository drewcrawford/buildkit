use std::path::{ PathBuf};
use crate::{CompileStep, Configuration, PathType};
use std::str::FromStr;
use crate::compile_system::dir_walk;

///How to find sourcefiles for compiling
#[derive(Clone)]
pub enum SourceFileStrategy {
    ///Use exactly the sourcefiles specified.
    SourceFiles(Vec<PathBuf>),
    ///Search recursively in these paths, starting from the manifest directory. e.g. payload like "src/"
    ///
    /// Note that if this path is absolute, we will search the absolute path instead.
    SearchFromManifest(Vec<PathBuf>)
}

impl SourceFileStrategy {
    pub(crate) fn resolve<C: CompileStep>(&self) -> Vec<PathBuf> {
        match self {
            SourceFileStrategy::SourceFiles(paths) => paths.into_iter().map(|f| f.clone()).collect(),
            SourceFileStrategy::SearchFromManifest(manifest_paths) => {
                let manifest_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
                let m_path = PathBuf::from_str(&manifest_string).unwrap();
                let mut vec = Vec::new();

                for path in manifest_paths {
                    let mut new_path = m_path.clone();
                    new_path.push(path);
                    dir_walk(&new_path, C::SOURCE_FILE_EXTENSION, &mut vec);
                }
                vec
            }
        }
    }
}
#[derive(Clone)]
pub struct CompileSettings {
    //Will scan this path for sourcefiles
    pub(crate) source_strategy: SourceFileStrategy,
    ///Path for output/intermediates
    pub(crate) intermediate_path: PathBuf,
    ///Whether debug/release
    pub(crate) configuration: Configuration,
    ///Pass these flags to the compiler.
    pub(crate) flags: Vec<String>,
}

#[derive(Clone)]
pub struct CompileSettingsBuilder {
    source_strategy: Option<SourceFileStrategy>,
    intermediate_path: Option<PathType>,
    configuration: Option<Configuration>,
    flags: Vec<String>,
}

impl CompileSettingsBuilder {
    pub fn new() -> Self {
        Self {
            source_strategy: None,
            intermediate_path: None,
            configuration: None,
            flags: Vec::new(),
        }
    }
    pub fn source_strategy(&mut self,strategy: SourceFileStrategy) -> &mut Self {
        self.source_strategy = Some(strategy);
        self
    }
    pub fn configuration(&mut self, configuration: Configuration) -> &mut Self {
        self.configuration = Some(configuration);
        self
    }
    pub fn intermediate_path(&mut self, path: PathType) -> &mut Self {
        self.intermediate_path = Some(path);
        self
    }
    pub(crate) fn _finish(&mut self, with_link: bool) -> CompileSettings {
        let intermediate_path = match &self.intermediate_path {
            Some(path) => path.path(),
            None => {
                if with_link {
                    //compile is an intermediate step, find an intermediate dir
                    let out_dir = std::env::var("OUT_DIR").expect("Must set `OUT_DIR` environment variable, or call `.intermediate_path()`");
                    PathBuf::from_str(&out_dir).unwrap()
                }
                else {
                    //compile is the final step; like a product path
                    PathType::EXERelative(PathBuf::new()).path().to_path_buf()
                }
            }
        };
        let source_strategy = match &self.source_strategy {
            None => {
                SourceFileStrategy::SearchFromManifest(vec![PathBuf::from_str("src").unwrap()])
            }
            Some(strategy) => strategy.clone()
        };
        let configuration = match self.configuration {
            Some(config) => config,
            None => {
                let debug_var = std::env::var("DEBUG").expect("Must set DEBUG environment variable or call .configuration().  (DEBUG was not valid unicode.)");
                if debug_var == "true" {
                    Configuration::Debug
                }
                else if debug_var == "false" {
                    Configuration::Release
                }
                else {
                    panic!("Must set DEBUG environment variable or call .configuration().  (We saw DEBUG={})",debug_var);
                }
            }
        };
        CompileSettings {
            source_strategy,
            intermediate_path,
            configuration,
            flags: self.flags.clone(),
        }
    }
    ///Set compiler flags.
    pub fn set_flags(&mut self, flags: Vec<String>) -> &mut Self {
        self.flags = flags;
        self
    }
    pub fn finish(&mut self) -> CompileSettings {
        //public version is non-link
        self._finish(false)
    }
}

#[test] fn source_walk() {
    use std::path::Path;
    struct YamlCompiler;
    impl CompileStep for YamlCompiler {
        const SOURCE_FILE_EXTENSION: &'static str = "yaml";
        fn compile_one<'a>(_path: &Path, _intermediate_dir: &Path, _configuration: &Configuration, _dependency_path: &Path, _flags: impl Iterator<Item=&'a str>) -> PathBuf {
            todo!()
        }
    }
    let s = SourceFileStrategy::SearchFromManifest(vec![PathBuf::from_str("src").unwrap()]).resolve::<YamlCompiler>();
    assert_eq!(s.len(), 0); //no yaml files in our build directory
}