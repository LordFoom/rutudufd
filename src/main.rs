use std::ffi::OsString;
use std::path::{Path};
use std::fs;
use clap::Parser;
use color_eyre::Report;
use color_eyre::eyre::Result;

#[derive(Parser, Debug)]
#[clap(name = "RutuduFD")]
#[clap(author = "Foom <lordfoom@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Searches through collections of rutudu lists", long_about = None)]
struct Args{
    // #[clap(required = true, parse(from_os_str))]
    #[clap(required = true)]
    search_terms: Vec<String>,
}

///Scan directory and return list or rutudu (*.rtd) files
fn scan_directory(dir:Option<&str>) -> Result<Vec<String>>{
    let scan_dir = dir.unwrap_or(".");//default to the current directory
    let ext = OsString::from("rtd");
    let mut rtd_files:Vec<String> = fs::read_dir(scan_dir)?
        .filter_map(|r| r.ok())
        .map(|f| f.path())
        .filter(|path| path.extension() == Some(&ext) )
         .map(|entry| String::from(entry.to_str().unwrap()))
        .collect();
    rtd_files.sort();
    Ok(rtd_files)
}
/// Go through all rtd files and find matches for the search terms
/// printing out the files and the  matches
fn search_rtd_db_files() -> Result<(), Report> {


    //scan the  files
    Ok(())
}

fn main()->Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    println!("rtdfd {:?}", args.search_terms);
    Ok(())
}

#[cfg(test)]
pub mod tests{
    use super::*;
    use color_eyre::eyre::Result;

    #[test]
    fn scan_directory()->Result<()>{
       Ok(())
    }
}