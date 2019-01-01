use clap::{load_yaml, App};
use skribe::{Id, Skribe};

fn main() {
    let yaml = load_yaml!("skribe.yml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("Subcommand: {:?}", matches.subcommand());
    match matches.subcommand() {
        ("init", Some(_)) => {
            let skribe = Skribe::init().unwrap();
            println!(
                "New skribe directory is {}",
                skribe.get_home().to_str().unwrap()
            );
        }
        ("add", Some(sub_m)) => println!(
            "You're about adding a new note \"{}\" with tags: {}",
            sub_m
                .values_of("text")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" "),
            sub_m
                .values_of("tags")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(", ")
        ),
        ("", None) => println!("Nothing to do"),
        _ => unreachable!(),
    }
}

fn add_item(text: String, tags: Vec<String>) -> Id {
    let mut item = Skribe::create_item();
    let id = item.id;
    item.text = text;
    Skribe::new().save_item(item);
    id
}
