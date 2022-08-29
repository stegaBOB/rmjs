use std::{fs, path::PathBuf, time::Instant};

use glob::glob;

pub fn find_and_remove(directory: &str, debug: bool) {
    let start = Instant::now();
    println!("Searching files in {directory}...");
    let all_js_files: Vec<PathBuf> = glob(&format!("{directory}/**/*.js"))
        .expect("whoops")
        .filter_map(|f| {
            if let Ok(f) = f {
                (!f.is_symlink()).then(|| f)
            } else {
                None
            }
        })
        .collect();

    let total_js_files = all_js_files.len();
    if total_js_files == 0 {
        println!("Couldn't find any JS files!");
        return;
    } else {
        println!("Found {total_js_files} total JavaScript files.");
    };

    let mut rmjs_change_counter: usize = 0;
    let mut removed_change_counter: usize = 0;

    all_js_files.into_iter().for_each(|js_file| {
        let ts_file = {
            let mut file = js_file.clone();
            file.set_extension("ts");
            file
        };
        if ts_file.is_file() {
            rmjs_change_counter += 1;
            match fs::remove_file(&js_file) {
                Ok(_) => {
                    removed_change_counter += 1;
                }
                Err(e) => {
                    if debug {
                        println!("Unable to delete {js_file:?}! Error: {e}");
                    }
                }
            }
        }
    });
    let duration = start.elapsed();
    let duration_message = if duration.as_secs() > 5 {
        let duration = duration.as_secs();
        format!("{duration} seconds")
    } else {
        let duration = duration.as_millis();
        format!("{duration} milliseconds")
    };
    if rmjs_change_counter == 0 {
        println!("Couldn't find any eligible .js files to remove!");
    } else if rmjs_change_counter != removed_change_counter {
        println!("Removed {removed_change_counter} .js files out of {rmjs_change_counter} detected eligible files in {duration_message}.");
        if !debug {
            println!("Rerun with the --debug option enabled for more detailed information.");
        }
    } else {
        println!("Removed all {removed_change_counter} eligible .js files in {duration_message}.");
    }
}
