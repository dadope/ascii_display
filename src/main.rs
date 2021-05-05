use std::fs;
use rand::seq::SliceRandom;

fn main() {
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

    match dirs::home_dir() {
        Some(mut path) => {

            // ~/.asciiDisplay
            path.push(".asciiDisplay");

            // if  ~/.asciiDisplay exists
            if fs::metadata(&path).is_ok() {

                let paths = fs::read_dir(path).unwrap();

                // iterates over all the files in the directory
                for p in paths {
                    let data = fs::read_to_string(p.unwrap().path())
                        .expect("Unable to read file");

                    available_ascii.push(data.clone());
                }

                // prints out a randomly selected file
                println!("{}", available_ascii.choose(&mut rand::thread_rng())
                    .as_deref().unwrap_or(&backup_ascii.to_string())
                );
            }

            // if ~/.asciiDisplay doesn´t exist, print the backup image
            else { println!("{}", backup_ascii) }
        }

        None => println!("Could not get the home directory!, exiting"),
    }
}