extern crate clap;
extern crate rand;

use std::fs;
use std::process::exit;
use std::path::{PathBuf};

use clap::{Arg, App};
use rand::seq::SliceRandom;

fn main() {
    let matches = App::new("ascii_display")
        .version("1.0")
        .author("dadope")
        .about("a simple commandline utility, which displays a randomly selected ascii image")
        .arg(Arg::with_name("asciiDirectory")
            .short("d")
            .long("directory")
            .value_name("directory")
            .help("Sets a custom asciiDirectory")
            .takes_value(true))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Verbose output"))
        .get_matches();

    let verbose = matches.is_present("verbose");

    let mut directory= PathBuf::new();

    let mut available_ascii = Vec::new();

    // Backup if no other ascii image can be found
    let backup_ascii = r#"
  _______________________________
 /\                              \
/++\    __________________________\
\+++\   \ ************************/
 \+++\   \___________________ ***/
  \+++\   \             /+++/***/
   \+++\   \           /+++/***/
    \+++\   \         /+++/***/
     \+++\   \       /+++/***/
      \+++\   \     /+++/***/
       \+++\   \   /+++/***/
        \+++\   \ /+++/***/
         \+++\   /+++/***/
          \+++\ /+++/***/
           \+++++++/***/
            \+++++/***/
             \+++/***/
              \+/___/
           "#;

    match matches.value_of("asciiDirectory") {
        Some(dir) => {
            directory.push(dir);
        }
        None => {
            directory = get_project_data_directory(backup_ascii, verbose);
        }
    }

    if verbose { println!("asciiDir: {:?}", directory.to_str().unwrap()) }

    if fs::metadata(&directory).is_ok() {

        let paths = fs::read_dir(directory).unwrap();

        // iterates over all the files in the directory
        for p in paths {

            let file = p.unwrap();

            if file.file_type().unwrap().is_file(){

                let data = fs::read_to_string(file.path())
                    .expect("Unable to read file");

                available_ascii.push(data.clone());
            }
        }

        // prints out a randomly selected file
        println!("{}", available_ascii.choose(&mut rand::thread_rng())
            .as_deref().unwrap_or(&backup_ascii.to_string())
        );
    }

    // if ~/.asciiDisplay doesn't exist, print the backup image
    else {
        if verbose { println!("Directory ({:?}) does not exist!", directory.to_str().unwrap())}
        println!("{}", backup_ascii)
    }
}

// returns ~/.asciiDisplay
fn get_project_data_directory(backup_ascii:&str, verbose:bool) -> PathBuf {
    if verbose { println!("No directory set! falling back to default directory...")}

    match dirs::home_dir() {
        Some(mut path) => {
            path.push(".asciiDisplay");
            return path
        }

        None => {
            println!("Error!, could not find the home directory");
            println!("{}", backup_ascii);
            exit(1)
        }
    }
}