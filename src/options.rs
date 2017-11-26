use std::path::PathBuf;
use std::str::FromStr;

#[derive(StructOpt, Clone)]
#[structopt(name = "part-dir", about = "Partition a directory by size")]
pub struct Options
{
    #[structopt(help = "The input directory", parse(from_os_str))]
    pub input: PathBuf,
    #[structopt(help = "The output directory", parse(from_os_str))]
    pub output: PathBuf,
    #[structopt(short = "s", long = "size", help = "The partition size")]
    pub size: Size,
    #[structopt(short = "p", long = "pack", help = "The packing mode", default_value = "none")]
    pub packing: Packing,
    #[structopt(short = "r", long = "recurse", help = "Recurse into subdirectories")]
    pub recurse: bool,
}

/// A size in bytes
#[derive(Copy, Clone)]
pub struct Size(pub u64);

impl FromStr for Size
{
    type Err = String;

    fn from_str(s: &str) -> Result<Size, String>
    {
        let mut num = String::new();
        let mut suf = String::new();

        let mut in_num = true;
        for c in s.chars()
        {
            if in_num && c.is_digit(10)
            {
                num.push(c);
            }
            else
            {
                suf.push(c);
                in_num = false;
            }
        }

        let num = match num.parse::<u64>()
        {
            Ok(n) => n,
            Err(_) => return Err(format!("'{}' is not a valid size", num))
        };

        match suf.trim().to_lowercase().as_str()
        {
            "" | "b" =>   Ok(Size(num * 1)),
            "k" | "kb" => Ok(Size(num * 1_000)),
            "m" | "mb" => Ok(Size(num * 1_000_000)),
            "g" | "gb" => Ok(Size(num * 1_000_000_000)),
            _ => Err(format!("'{}' is not a valid suffix", suf))
        }
    }
}

/// A packing mode
#[derive(Copy, Clone, PartialEq)]
pub enum Packing
{
    None,
    Normal,
    Tight,
}

impl FromStr for Packing
{
    type Err = String;

    fn from_str(s: &str) -> Result<Packing, String>
    {
        match s.to_lowercase().as_str()
        {
            "none" => Ok(Packing::None),
            "normal" => Ok(Packing::Normal),
            "tight" => Ok(Packing::Tight),
            _ => Err(format!("'{}' is not a packing options", s))
        }
    }
}
