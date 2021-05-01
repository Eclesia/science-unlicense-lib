//
// Public Domain - unlicense.science
//

pub struct FileSystemPath {
    path : String,
}

impl FileSystemPath {

    pub fn new(path: &str) -> Self {
        return FileSystemPath {
            path: path.to_string();
        }
    }

}

impl Path for FileSystemPath {

    fn open_read(&self) -> dyn Read {
        File::open(self.path);
    }

}