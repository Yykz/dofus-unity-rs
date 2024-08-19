use std::io::{self, BufWriter};
use std::path::{Path, PathBuf};

use pest::Parser;

use crate::parser_items::{File, FileContent};
use crate::{CSharpParser, ProtoEntity, Rule};

struct CreateOnWriteFile<'a> {
    path: &'a Path,
    maybe_file: Option<BufWriter<std::fs::File>>,
}

impl<'a> CreateOnWriteFile<'a> {
    fn new(path: &'a Path) -> Self {
        Self {
            path,
            maybe_file: None,
        }
    }
}

impl<'a> io::Write for CreateOnWriteFile<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.maybe_file {
            Some(ref mut bufwriter) => bufwriter.write(buf),
            None => {
                let file = std::fs::File::create(self.path)?;
                let bufwriter = BufWriter::new(file);
                self.maybe_file = Some(bufwriter);
                self.write(buf)
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
    pub fn from_source<P: AsRef<Path>>(source: P, outdir: PathBuf) -> Result {
        Self {
            source: source.as_ref().into(),
            outdir,
        }
        .process_dir_recu(source.as_ref())
    }

    fn process_dir_recu(&self, path: &Path) -> Result {
        let affix = path.strip_prefix(&self.source).unwrap();
        let mut path_outfile = self.outdir.clone();
        path_outfile.push(format!(
            "{}.proto",
            affix.to_str().unwrap().to_lowercase().replace("/", "_")
        ));

        let mut file = CreateOnWriteFile::new(&path_outfile);

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
