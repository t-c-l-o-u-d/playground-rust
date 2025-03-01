// GNU Affero General Public License v3.0 or later (see COPYING or https://www.gnu.org/licenses/agpl.txt)
//
// A rust program that will combine several linux package managers into a unified command structure.
// The goal is to only use core libraries as this is a learning experiement.
// This is very much a work in progress. If it becomes stable enough, it will get a dedicated repository.
// 
// Integrates pacman, dnf/yum, and apt.
// More may be added in the future...
// 
// Has the following options:
// 
// pkg search           -- Searches for packages
// pkg install          -- Installs packages
// pkg remove           -- Removes packages
// pkg purge            -- This purges all related config files based on the contents of the package
// pkg whatprovides     -- This shows which package provides any command when queried

use std::env;
use std::process::Command;

fn search(package: &str) {
    match Command::new("pacman")
        .arg("--sync")
        .arg("--search")
        .arg(package)
        .output()
    {
        Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn main() {
    // add all arguments given into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
    }

    // start counter at 2 to account for program and command
    let mut counter = 2;

    loop {
        let query = &args[counter];

        search(query);

        counter += 1;

        if counter == args.len() {
            break counter;
        }
    };

    dbg!(&args);
}
