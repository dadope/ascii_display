extern crate clap;
extern crate rand;
extern crate terminal_size;

use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use rand::seq::SliceRandom;

use clap::{Arg, App};
use terminal_size::{Width, Height, terminal_size};

fn main() {
    let matches = App::new("ascii_display")
        .version("1.1")
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
        .arg(Arg::with_name("no center")
            .short("n")
            .long("no_center")
            .help("Disables image centering"))
        .arg(Arg::with_name("no timeout")
            .short("t")
            .long("no_timeout")
            .help("Disables initial delay (not recommended)"))
        .get_matches();

    let verbose = matches.is_present("verbose");
    let no_center = matches.is_present("no center");
    let no_timeout = matches.is_present("no timeout");

    let mut directory= PathBuf::new();

    let mut available_ascii = Vec::new();

    // Backup if no other ascii image can be found
    let backup_ascii = r"
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
             \+thread::++/***/
              \+/___/";

    // slight delay, as the program runs so fast that getting the terminal size can be difficult
    if !no_timeout { sleep(Duration::from_millis(100)) }

    match matches.value_of("asciiDirectory") {
        Some(dir) => {
            directory.push(dir);
        }
        None => {
            directory = get_project_data_directory(backup_ascii, verbose);
        }
    }

    if verbose { println!("asciiDir: {:?}", directory.to_str().unwrap()) }

    let size = terminal_size();
    let term_height;
    let term_width;

    if let Some((Width(w), Height(h))) = size {
        term_width = w as usize;
        term_height = h as usize;

        if verbose { println!("Your terminal is {} cols wide and {} lines tall", w, h) }
    } else {
        // arbitrary values for a fullscreen terminal
        term_width = 236;
        term_height = 60;

        if verbose { println!("Unable to get terminal size") }
    }

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

        let mut selected_string = available_ascii.choose(&mut rand::thread_rng())
            .unwrap_or(&backup_ascii.to_string()).to_string();

        let mut longest_line = selected_string.lines().max_by(
            |x, y| x.len().cmp(&y.len())
        ).unwrap().len();

        let mut lines = selected_string.lines().count();

        if lines >= term_height || longest_line >= term_width {
            selected_string = String::from(backup_ascii);

            longest_line = selected_string.lines().max_by(
                |x, y| x.len().cmp(&y.len())
            ).unwrap().len();

            lines = selected_string.lines().count();

            if verbose {
                println!("The selected image too small for the terminal, falling back to the backup image")
            }

            if lines >= term_height || longest_line >= term_width{
                println!("{:_^term$}", "#", term=term_width);

                if verbose{
                    println!("terminal too small, even for the backup image, exiting")
                }

                exit(1);
            }
        }

        if no_center{
            println!("{}", selected_string)
        } else {
            center_print_image(selected_string, term_width, longest_line);
        }

        //prints out a line to indicate the center of the window
        if verbose{
            println!("{:_^term$}", "#", term=term_width);
        }
    }

    // if ~/.asciiDisplay doesn't exist, print the backup image
    else {
        if verbose { println!("Directory ({:?}) does not exist!", directory.to_str().unwrap())}
        println!("{}", backup_ascii)
    }
}

// prints out the centered ascii image
fn center_print_image(selected_string:String, term_width:usize, longest_line:usize){
    for line in selected_string.split("\n") {
        println!("{: ^width$}", line=line, width=term_width - (longest_line - line.len()));
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