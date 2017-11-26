use std::path::{Path, PathBuf};

/// An item in a directory
pub enum Item
{
    File
    {
        size: u64,
        relpath: PathBuf,
        path: PathBuf,
    },
    Directory
    {
        size: u64,
        contents: Vec<Item>,
    }
}

impl Item
{
    pub fn size(&self) -> u64
    {
        match self
        {
            &Item::File{ ref size, .. } => *size,
            &Item::Directory{ ref size, .. } => *size,
        }
    }

    pub fn copy_to(self, output: &Path) -> bool
    {
        use std::fs::{create_dir_all, copy};

        match self
        {
            Item::File
            {
                relpath,
                path,
                ..
            } =>
            {
                let outpath = output.join(relpath);

                if let Some(parent) = outpath.parent()
                {
                    if let Err(_) = create_dir_all(parent)
                    {
                        return false;
                    }
                }

                copy(path, outpath).is_ok()
            },
            Item::Directory
            {
                contents,
                ..
            } =>
            {
                let mut dirs = vec![contents];

                while let Some(dir) = dirs.pop()
                {
                    for item in dir
                    {
                        match item
                        {
                            Item::File
                            {
                                relpath,
                                path,
                                ..
                            } =>
                            {
                                let outpath = output.join(relpath);

                                if let Some(parent) = outpath.parent()
                                {
                                    if let Err(_) = create_dir_all(parent)
                                    {
                                        return false;
                                    }
                                }

                                if let Err(_) = copy(path, outpath)
                                {
                                    return false;
                                }
                            },
                            Item::Directory
                            {
                                contents,
                                ..
                            } => dirs.push(contents)
                        }
                    }
                }

                true
            }
        }
    }
}
