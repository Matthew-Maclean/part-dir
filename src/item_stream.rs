use std::fs::{ReadDir, read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::io;

use item::Item;

pub struct ItemStream
{
    path: PathBuf,
    recurse: bool,
    current: ReadDir,
    dirs: Vec<ReadDir>,
    errors: u64,
}

impl ItemStream
{
    pub fn new<P>(path: P, recurse: bool) -> io::Result<ItemStream>
        where P: AsRef<Path>
    {
        Ok(ItemStream
        {
            path: path.as_ref().to_owned(),
            recurse: recurse,
            current: read_dir(path.as_ref())?,
            dirs: Vec::new(),
            errors: 0,
        })
    }

    fn get_next_entry(&mut self) -> Option<DirEntry>
    {
        loop
        {
            if let Some(res) = self.current.next()
            {
                match res
                {
                    Ok(entry) => return Some(entry),
                    Err(_) => self.errors += 1,
                }
            }
            else
            {
                match self.dirs.pop()
                {
                    Some(dir) => self.current = dir,
                    None => return None,
                }
            }
        }
    }

    fn entry_to_item(entry: &DirEntry, input: &Path) -> Option<Item>
    {
        let ft = match entry.file_type()
        {
            Ok(ft) => ft,
            Err(_) => return None,
        };

        if ft.is_file()
        {
            let data = match entry.metadata()
            {
                Ok(md) => md,
                Err(_) => return None,
            };

            let path = entry.path();
            let relpath = match path.strip_prefix(input)
            {
                Ok(rp) => rp.to_owned(),
                Err(_) => return None,
            };

            Some(Item::File
            {
                size: data.len(),
                relpath: relpath,
                path: path,
            })
        }
        else if ft.is_dir()
        {
            let data = match entry.metadata()
            {
                Ok(md) => md,
                Err(_) => return None,
            };

            let mut size = data.len();
            let mut dirs = vec![match read_dir(entry.path())
            {
                Ok(rd) => rd,
                Err(_) => return None,
            }];
            let mut contents = Vec::new();

            while let Some(dir) = dirs.pop()
            {
                for entry in dir
                {
                    let entry = match entry
                    {
                        Ok(e) => e,
                        Err(_) => return None,
                    };

                    let ft = match entry.file_type()
                    {
                        Ok(ft) => ft,
                        Err(_) => return None,
                    };

                    let data = match entry.metadata()
                    {
                        Ok(md) => md,
                        Err(_) => return None,
                    };

                    size += data.len();

                    if ft.is_file()
                    {
                        let path = entry.path();
                        let relpath = match path.strip_prefix(input)
                        {
                            Ok(rp) => rp.to_owned(),
                            Err(_) => return None,
                        };
                        
                        contents.push(Item::File
                        {
                            size: data.len(),
                            relpath: relpath,
                            path: path,
                        });
                    }
                    else if ft.is_dir()
                    {
                        let rd = match read_dir(entry.path())
                        {
                            Ok(rd) => rd,
                            Err(_) => return None,
                        };

                        dirs.push(rd);
                    }
                    else // lol symlinks
                    {
                        return None;
                    }
                }
            }

            Some(Item::Directory
            {
                size: size,
                contents: contents,
            })
        }
        else // lol symlinks
        {
            None
        }
    }
}

impl Iterator for ItemStream
{
    type Item = Item;

    fn next(&mut self) -> Option<Item>
    {
        loop
        {
            let entry = self.get_next_entry()?;

            let ft = match entry.file_type()
            {
                Ok(ft) => ft,
                Err(_) =>
                {
                    self.errors += 1;
                    continue;
                },
            };

            if ft.is_file()
            {
                return ItemStream::entry_to_item(&entry, &self.path);
            }
            else if ft.is_dir()
            {
                let rd = match read_dir(entry.path())
                {
                    Ok(rd) => rd,
                    Err(_) =>
                    {
                        self.errors += 1;
                        continue;
                    },
                };

                if self.recurse
                {
                    self.dirs.push(rd);
                    continue;
                }
                else
                {
                    return ItemStream::entry_to_item(&entry, &self.path);
                }
            }
            else // lol symlinks
            {
                self.errors += 1;
                continue;
            }
        }
    }
}
