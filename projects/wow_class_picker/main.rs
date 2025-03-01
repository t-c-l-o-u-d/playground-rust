// GNU Affero General Public License v3.0 or later (see COPYING or https://www.gnu.org/licenses/agpl.txt)

use std::io::{self, Write};

fn main() {
    // Dictionary of all class and spec combinations
    let classes_and_specs = [
        ("Death Knight", vec!["Blood", "Frost", "Unholy"]),
        ("Demon Hunter", vec!["Havoc", "Vengeance"]),
        ("Druid", vec!["Balance", "Feral", "Guardian", "Restoration"]),
        ("Evoker", vec!["Devastation", "Augmentation", "Preservation"]),
        ("Hunter", vec!["Beast Mastery", "Marksmanship", "Survival"]),
        ("Mage", vec!["Arcane", "Fire", "Frost"]),
        ("Monk", vec!["Windwalker", "Brewmaster", "Mistweaver"]),
        ("Paladin", vec!["Holy", "Protection", "Retribution"]),
        ("Priest", vec!["Discipline", "Holy", "Shadow"]),
        ("Rogue", vec!["Assassination", "Outlaw", "Subtlety"]),
        ("Shaman", vec!["Elemental", "Enhancement", "Restoration"]),
        ("Warlock", vec!["Affliction", "Demonology", "Destruction"]),
        ("Warrior", vec!["Arms", "Fury", "Protection"]),
    ];

    let mut answer = String::new();

    // Loop until user response
    while answer.to_lowercase() != "y" {
        let class_index = rand_number(classes_and_specs.len());
        let class_selection = classes_and_specs[class_index].0;
        let spec_index = rand_number(classes_and_specs[class_index].1.len());
        let spec_selection = classes_and_specs[class_index].1[spec_index];
        println!("Class: {}", class_selection);
        println!("Spec: {}", spec_selection);
        print!("Are you satisfied with that result? [y/n]: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        answer = input.trim().to_string();
        println!();
    }
}

fn rand_number(max: usize) -> usize {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as usize;
    (seed % max) as usize
}
