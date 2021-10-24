use std::path::PathBuf;
use crate::{CompileStep, Configuration, PathType};
use std::str::FromStr;
use crate::compile_system::dir_walk;

///How to find sourcefiles for compiling
#[derive(Clone)]
pub enum SourceFileStrategy {
    ///Use exactly the sourcefiles specified.
    SourceFiles(Vec<PathBuf>),
    ///Search recursively in this path, starting from the manifest directory. e.g. payload like "src/"
    ///
    /// Note that if this path is absolute, we will search the absolute path instead.
    SearchFromManifest(String)
}

impl SourceFileStrategy {
    pub(crate) fn resolve<C: CompileStep>(&self) -> Vec<PathBuf> {
        match self {
            SourceFileStrategy::SourceFiles(paths) => paths.into_iter().map(|f| f.clone()).collect(),
            SourceFileStrategy::SearchFromManifest(manifest_path) => {
                let manifest_string = std::env::var("CARGO_MANIFEST_DIR").unwrap();
                let mut m_path = PathBuf::from_str(&manifest_string).unwrap();
                m_path.push(manifest_path);
                let mut vec = Vec::new();
                dir_walk(&m_path, C::SOURCE_FILE_EXTENSION, &mut vec);
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
}

pub struct CompileSettingsBuilder {
    source_strategy: Option<SourceFileStrategy>,
    intermediate_path: Option<PathBuf>,
    configuration: Option<Configuration>
}

impl CompileSettingsBuilder {
    pub fn new() -> Self {
        Self {
            source_strategy: None,
            intermediate_path: None,
            configuration: None
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
    pub fn intermediate_path(&mut self, path: PathBuf) -> &mut Self {
        self.intermediate_path = Some(path);
        self
    }
    pub(crate) fn _finish(&mut self, with_link: bool) -> CompileSettings {
        let intermediate_path = match &self.intermediate_path {
            Some(path) => path.clone(),
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
                SourceFileStrategy::SearchFromManifest("src".to_owned())
            }
            Some(strategy) => strategy.clone()
        };
        let configuration = match self.configuration {
            Some(config) => config,
            None => if std::env::var("DEBUG").expect("Must set DEBUG environment variable or call .configuration()") == "1" { Configuration::Debug } else { Configuration::Release }
        };
        CompileSettings {
            source_strategy,
            intermediate_path,
            configuration
        }
    }
    pub fn finish(&mut self) -> CompileSettings {
        //public version is non-link
        self._finish(false)
    }
}