use std::ffi::OsString;
use std::fs;
use std::sync::Once;
use clap::Parser;
use color_eyre::Report;
use color_eyre::eyre::Result;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
fn search_rtd_db_files(terms: Vec<String>, dir:&str) -> Result<(), Report> {

    let rtd_files = scan_directory(dir)?;


    //scan the  files
    Ok(())
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

    //let files = scan_directory(dir);
    if let Err(e) = search_rtd_db_files(terms, &dir){
        panic!("Could not search files: {} ", e);
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