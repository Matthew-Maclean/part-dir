extern crate structopt;
#[macro_use] extern crate structopt_derive;

mod options;
mod item;
mod item_stream;
mod part;

use structopt::StructOpt;

use std::error::Error;
use std::path::Path;
use std::io;

use options::{Options, Packing};

fn main()
{
    use std::process::exit;

    let opts = Options::from_args();

    let (parts, errors) = match partition(&opts.input, opts.size.0, opts.recurse)
    {
        Ok((p, e)) => (p, e),
        Err(e) =>
        {
            eprintln!("Partitioning error: {}", e.description());
            exit(1);
        }
    };
    println!("{} parts ({} errors)", parts.len(), errors);

    let parts = pack(opts.packing, parts, opts.size.0);
    println!("{} parts after packing", parts.len());

    let (copied, errors) = copy(&opts.output, parts);
    println!("{} copied ({} errors)", copied, errors);
}

fn partition(input: &Path, size: u64, recurse: bool) -> io::Result<(Vec<part::Part>, u64)>
{
    let mut errors = 0u64;

    let parts =
    {
        let stream = item_stream::ItemStream::new(input, &mut errors, recurse)?;

        part::Part::partition(stream, size)
    };

    Ok((parts, errors))
}

fn pack(mode: Packing, mut parts: Vec<part::Part>, size: u64) -> Vec<part::Part>
{
    if mode == options::Packing::None
    {
        return parts;
    }
    else if mode == options::Packing::Tight
    {
        use std::cmp::Ord;

        parts.sort_by(|a, b| a.size.cmp(&b.size).reverse());
    }

    let mut packed = Vec::new();
    while let Some(mut part) = parts.pop()
    {
        if part.items.len() == 0
        {
            continue;
        }
        else if part.size < size
        {
            parts = parts.into_iter().map(|mut unpacked|
            {
                unpacked.items = unpacked.items.into_iter().filter_map(|item|
                {
                    if item.size() + part.size <= size
                    {
                        part.add(item);
                        None
                    }
                    else
                    {
                        Some(item)
                    }
                }).collect::<Vec<_>>();
                unpacked
            }).collect::<Vec<_>>();

            packed.push(part);
        }
        else
        {
            packed.push(part);
        }
    }

    packed
}

fn copy(output: &Path, parts: Vec<part::Part>) -> (u64, u64)
{
    let mut ok = 0u64;
    let mut err = 0u64;
    for (i, part) in parts.into_iter().enumerate()
    {
        let path = output.join(format!("part-{}", i));
        for item in part.items
        {
            if item.copy_to(&path)
            {
                ok += 1;
            }
            else
            {
                err += 1;
            }
        }
    }

    (ok, err)
}
