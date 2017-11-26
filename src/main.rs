extern crate structopt;
#[macro_use] extern crate structopt_derive;

mod options;
mod item;
mod item_stream;

use structopt::StructOpt;

use std::error::Error;
use std::path::Path;
use std::io;

use options::{Options, Packing};

fn main()
{
    use std::process::exit;

    let opts = Options::from_args();

    let (parts, ok, errors) = match partition(&opts.input, opts.size.0, opts.recurse)
    {
        Ok((p, o, e)) => (p, o, e),
        Err(e) =>
        {
            eprintln!("Partitioning error: {}", e.description());
            exit(1);
        }
    };

    println!("{} parts ({} items, {} errors)", parts.len(), ok, errors);

    let parts = pack(opts.packing, parts);

    let (copied, errors) = match copy(&opts.output, parts)
    {
        Ok((c, e)) => (c, e),
        Err(e) =>
        {
            eprintln!("Copying error: {}", e.description());
            exit(1);
        }
    };

    println!("{} copied ({} errors)", copied, errors);
}

fn partition(_input: &Path, _size: u64, _recurse: bool) -> io::Result<(Vec<()>, u64, u64)>
{
    unimplemented!()
}

fn pack(_mode: Packing, _parts: Vec<()>) -> Vec<()>
{
    unimplemented!()
}

fn copy(_output: &Path, _parts: Vec<()>) -> io::Result<(u64, u64)>
{
    unimplemented!()
}
