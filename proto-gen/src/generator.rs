use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};

use pest::Parser;

use crate::parser_items::{File, FileContent};
use crate::{CSharpParser, ProtoEntity, Rule};

struct CreateOnWriteFile<'a> {
    path: &'a Path,
    package_prefix: &'a str,
    maybe_file: Option<BufWriter<std::fs::File>>,
}

impl<'a> CreateOnWriteFile<'a> {
    fn new(path: &'a Path, s: &'a str) -> Self {
        Self {
            path,
            maybe_file: None,
            package_prefix: s
        }
    }
}

impl<'a> io::Write for CreateOnWriteFile<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.maybe_file {
            Some(ref mut bufwriter) => bufwriter.write(buf),
            None => {
                let file = std::fs::File::create(self.path)?;
                let mut bufwriter = BufWriter::new(file);
                writeln!(bufwriter, "syntax = \"proto3\";\n\npackage {}.{};\n", self.package_prefix, self.path.file_stem().unwrap().to_str().unwrap())?;
                let r = bufwriter.write(buf);
                self.maybe_file = Some(bufwriter);
                r
            }
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self.maybe_file {
            Some(ref mut bufwriter) => bufwriter.flush(),
            None => Ok(()),
        }
    }
}

pub struct Generator {
    source: PathBuf,
    outdir: PathBuf,
    package_prefix: String
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CSharpParser failed to parse {0}")]
    FailedToParse(#[from] pest::error::Error<Rule>),
    #[error("failed to read file {0} {1}")]
    FailedToReadFile(PathBuf, io::Error),
    #[error("failed to write to file {0}")]
    FailedToWriteProto(io::Error),
    #[error("failed to read dir {0} {1}")]
    FailedToReadDir(PathBuf, io::Error),
}

pub type Result = std::result::Result<(), Error>;

impl Generator {
    pub fn from_source<P: AsRef<Path>, O: Into<PathBuf>, S: Into<String>>(source: P, outdir: O, package_prefix: S) -> Result {
        Self {
            source: source.as_ref().into(),
            outdir: outdir.into(),
            package_prefix: package_prefix.into()
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

        let mut file = CreateOnWriteFile::new(&path_outfile, &self.package_prefix);

        for entry in std::fs::read_dir(path).map_err(|e| Error::FailedToReadDir(path.into(), e))? {
            let entry = entry.map_err(|e| Error::FailedToReadDir(path.into(), e))?;
            let path = entry.path();

            match path.is_file() {
                true => {
                    Self::process_file(&path, &mut file)?;
                }
                false => {
                    self.process_dir_recu(&path)?;
                }
            }
        }
        Ok(())
    }

    fn process_file<W: io::Write>(path: &Path, mut out: W) -> Result {
        let content = std::fs::read_to_string(path).map_err(|e| Error::FailedToReadFile(path.into(), e))?;
        let mut parsed = CSharpParser::parse(Rule::file, &content[3..])?;
        let file_pair = parsed.next().unwrap();
        let parsed_file = File::try_from(file_pair).unwrap();
        match parsed_file.content {
            FileContent::Class(_class) => {}
            FileContent::Namespace(namespace) => {
                if let Ok(proto) = ProtoEntity::try_from(namespace) {
                    writeln!(out, "{}", proto).map_err(|e| Error::FailedToWriteProto(e))?;
                }
            }
        }
        Ok(())
    }
}
