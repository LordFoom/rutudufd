use std::ffi::OsString;
use std::fs;
use std::sync::Once;
use clap::Parser;
use color_eyre::Report;
use color_eyre::eyre::{eyre, Result};
use rusqlite::Connection;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use std::fmt::{Display, Formatter};
use colored::Colorize;

#[derive(Parser, Debug)]
#[clap(name = "RutuduFD")]
#[clap(author = "Foom <lordfoom@gmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Searches through collections of rutudu lists", long_about = None)]
struct Args{
    // #[clap(required = true, parse(from_os_str))]
    #[clap(required = true)]
    search_terms: Vec<String>,
    #[clap(short, long, default_value = "./")]
    search_dir: String,
    #[clap(short, long)]
    verbose: bool,

}

struct SearchResult{
    title: String,
    description: String,
    list_name: String,
    search_terms: Vec<String>,
}

impl Display for SearchResult{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut colored_title = String::new();
        self.title.split_whitespace()
            .for_each(|word| {
                if self.search_terms.contains(&word.to_string()){
                    colored_title = format!("{} {}", colored_title, word.red());
                }else{
                    colored_title = format!("{} {}", colored_title, word.white());
                };
            });
        write!(f, "{} => {} : {}", self.list_name, colored_title, self.description)
    }
}
static INIT: Once = Once::new();

fn init_logging(verbose:bool){
    INIT.call_once(||{
        let log_level = if verbose {
            Level::TRACE
        }else{
            Level::ERROR
        };
        let subs = FmtSubscriber::builder()
            .with_max_level(log_level)
            .finish();

        tracing::subscriber::set_global_default(subs).expect("setting stdout logger failed");
    });
}

///Initialize our things
fn init(verbose:bool) -> Result<()>{
    if std::env::var("RUST_LIB_BACKTRACE").is_err(){
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    init_logging(verbose);

    Ok(())
}
///Scan directory and return list or rutudu (*.rtd) files
fn scan_directory(scan_dir:&str) -> Result<Vec<String>>{
    // let scan_dir = dir.unwrap_or(".");//default to the current directory
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
fn search_rtd_db_files(search_terms: Vec<String>, dir:&str) -> Result<Vec<SearchResult>, Report> {

    if search_terms.len() == 0{
        return Err(eyre!("Must have at least one search term"));
    }
    let rtd_files = scan_directory(dir)?;
    //go through each database, looking for our keywords
    let mut results = Vec::<SearchResult>::new();

    info!("Search term: {:?}", search_terms);
    //start with one term
    // let term = terms.get(0).ok_or(Err(eyre!("Unable to get first search term"))).unwrap();
    let term = search_terms.get(0).unwrap();
    for list in rtd_files {
            info!("Checking file: {}", list);

            let conn = Connection::open(list.clone())?;
            let mut stmt = conn
                .prepare("select title, entry from rutudu_list where title like :title or entry like :entry ")?;
            let search_result_iter = stmt.query_map(&[( ":title", &term ), (":entry", &term)], |row| {
                info!("Found result in {}", list.clone());
                Ok(SearchResult{
                   list_name: list.clone(),
                    title: row.get(0)?,
                    description:  row.get(1)?,
                    search_terms: search_terms.clone(),
                })
            })?;
            search_result_iter.for_each(|sr| {
              results.push(sr.unwrap());
            });
        };

    Ok(results)
}

fn print_results(results: Vec<SearchResult>){
    results.iter()
        .for_each( |r| println!("{}", r.to_string()));
}

fn main()->Result<()> {
    //want this before anything else
    color_eyre::install()?;


    let args = Args::parse();
    let verbose = args.verbose;
    if let Err(e) = init(verbose){
        panic!("Unable to init {}", e);
    }
    let terms = args.search_terms;
    let dir = args.search_dir;
    info!("rtdfd searching for {:?}", &terms);
    info!("Searching in {}", dir);

    match search_rtd_db_files(terms, &dir){
        Ok(results) => print_results(results),
        Err(e) => panic!("Could not search files: {} ", e),
    }

    Ok(())
}

#[cfg(test)]
pub mod tests{
    use super::*;
    use color_eyre::eyre::Result;

    #[test]
    fn scan_directory()->Result<()>{
        init_logging(true);
        info!("Scanning directory test fired...");
        //ensure at least one rtd file exists
        let path = "./test_scan.rtd";
        std::fs::write(path, "")?;
        if let Ok(v) = super::scan_directory("."){
            info!("Directory scanned, found {} entries", v.len());
            // for s in &v{
            //     info!("{}", s);
            // }
            assert!(v.iter().any(|s|{ s== path }));
        }else{
            panic!("Could not find created file");
        };
        std::fs::remove_file(path)?;
        Ok(())
    }
}