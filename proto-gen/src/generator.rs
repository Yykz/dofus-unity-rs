
use std::io::{BufWriter, self};
use std::path::{Path, PathBuf};

use pest::Parser;

use crate::parser_items::{FileContent, File};
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

impl Generator {
    pub fn from_source<P: AsRef<Path>>(source: P, outdir: PathBuf) {
        let _ = Self {
            source: source.as_ref().into(),
            outdir,
        }
        .process_dir_recu(source.as_ref());
    }

    fn process_dir_recu(&self, path: &Path) -> io::Result<()> {
        let affix = path.strip_prefix(&self.source).unwrap();
        let mut dir_out_filename = self.outdir.clone();
        dir_out_filename.push(format!(
            "{}.proto",
            affix.to_str().unwrap().to_lowercase().replace("/", "_")
        ));

        let mut file = CreateOnWriteFile::new(&dir_out_filename);

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
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

    fn process_file<W: io::Write>(path: &Path, mut out: W) -> io::Result<()> {
        let content = std::fs::read_to_string(path)?;
        match CSharpParser::parse(Rule::file, &content[3..]) {
            Ok(mut parsed) => {
                let file_pair = parsed.next().unwrap();
                let parsed_file = File::try_from(file_pair).unwrap();
                match parsed_file.content {
                    FileContent::Class(_class) => {}
                    FileContent::Namespace(namespace) => {
                        if let Ok(proto) = ProtoEntity::try_from(namespace) {
                            writeln!(out, "{}", proto)?;
                        }
                    }
                }
            }
            Err(_e) => panic!("Failed to parse {:?}", path.to_str().unwrap()),
        }
        Ok(())
    }
}

