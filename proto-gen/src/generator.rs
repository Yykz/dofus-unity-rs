use std::collections::HashSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use pest::Parser;

use crate::parser_items::{self, File, FileContent};
use crate::{CSharpParser, ProtoEntity, Rule};

#[derive(Debug, Default)]
struct ProtoFileBuffer {
    content: Vec<u8>,
    imports: HashSet<String>,
}

impl ProtoFileBuffer {
    pub fn write_to_file(&self, path: &Path, package: &str) -> io::Result<()> {
        if self.content.is_empty() {
            return Ok(());
        }
        let mut file = std::fs::File::create(path)?;
        writeln!(file, "syntax = \"proto3\";\n")?;
        for import in &self.imports {
            writeln!(file, "import \"{}.proto\";", &import[1..])?;
        }
        writeln!(file, "package {};\n", package)?;
        file.write_all(&self.content)?;
        Ok(())
    }

    pub fn add_imports(&mut self, imports: Vec<parser_items::Import>, base_namespace: &str) {
        self.imports.extend(
            imports
                .into_iter()
                .filter(|import| import.0.starts_with(base_namespace))
                .map(|import| {
                    import
                        .0
                        .strip_prefix(base_namespace)
                        .unwrap()
                        .to_ascii_lowercase()
                }),
        )
    }
}

impl io::Write for ProtoFileBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.content.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.content.flush()
    }
}

pub struct Generator {
    source: PathBuf,
    outdir: PathBuf,
    package_prefix: String,
    base_namespace: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CSharpParser failed to parse {0}")]
    Parse(#[from] Box<pest::error::Error<Rule>>),
    #[error("failed to read file {0} {1}")]
    ReadFile(PathBuf, io::Error),
    #[error("failed to write to proto buffer {0}")]
    WriteProto(io::Error),
    #[error("failed to read dir {0} {1}")]
    ReadDir(PathBuf, io::Error),
    #[error("failed to write to file {0} {1}")]
    WriteFile(PathBuf, io::Error),
}

pub type Result = std::result::Result<(), Error>;

impl Generator {
    pub fn from_source<P, O, S, S2>(
        source: P,
        outdir: O,
        package_prefix: S,
        base_namespace: S2,
    ) -> Result
    where
        P: AsRef<Path>,
        O: Into<PathBuf>,
        S: Into<String>,
        S2: Into<String>,
    {
        Self {
            source: source.as_ref().into(),
            outdir: outdir.into(),
            package_prefix: package_prefix.into(),
            base_namespace: base_namespace.into(),
        }
        .process_dir_recu(source.as_ref())
    }

    fn process_dir_recu(&self, path: &Path) -> Result {
        let affix = path.strip_prefix(&self.source).unwrap();
        let mut path_outfile = self.outdir.clone();
        path_outfile.push(format!(
            "{}.proto",
            affix.to_str().unwrap().to_lowercase().replace("/", ".")
        ));

        let mut protofile = ProtoFileBuffer::default();

        for entry in std::fs::read_dir(path).map_err(|e| Error::ReadDir(path.into(), e))? {
            let entry = entry.map_err(|e| Error::ReadDir(path.into(), e))?;
            let path = entry.path();

            match path.is_file() {
                true => {
                    self.process_file(&path, &mut protofile)?;
                }
                false => {
                    self.process_dir_recu(&path)?;
                }
            }
        }
        let package_file = path_outfile
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap();
        let package = format!("{}.{}", self.package_prefix, package_file);
        protofile
            .write_to_file(&path_outfile, &package)
            .map_err(|e| Error::WriteFile(path_outfile, e))
    }

    fn process_file(&self, path: &Path, out: &mut ProtoFileBuffer) -> Result {
        let content =
            std::fs::read_to_string(path).map_err(|e| Error::ReadFile(path.into(), e))?;
        let mut parsed = CSharpParser::parse(Rule::file, &content[3..]).map_err(Box::new)?;
        let file_pair = parsed.next().unwrap();
        let parsed_file = File::try_from(file_pair).unwrap();
        out.add_imports(parsed_file.imports, &self.base_namespace);
        match parsed_file.content {
            FileContent::Class(_class) => {}
            FileContent::Namespace(namespace) => {
                if let Ok(proto) = ProtoEntity::try_from(namespace) {
                    writeln!(out, "{}", proto).map_err(Error::WriteProto)?;
                }
            }
        }
        Ok(())
    }
}
