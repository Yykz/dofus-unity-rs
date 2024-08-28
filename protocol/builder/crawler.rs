use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct Crawler {
    pub dirs: Vec<PathBuf>,
    pub files: Vec<PathBuf>,
}

impl Crawler {
    pub fn start<P: Into<PathBuf>>(p: P) -> Self {
        let mut crawler = Self::default();
        crawler.in_dir(p.into());
        crawler
    }

    fn in_dir(&mut self, p: PathBuf) {
        for entry in std::fs::read_dir(&p).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            match path.is_file() {
                true => self.files.push(path),
                false => self.in_dir(path),
            }
        }
        self.dirs.push(p);
    }
}
