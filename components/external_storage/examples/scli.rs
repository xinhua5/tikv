use clap::*;

static CMD: &str = r#"
name: storagecli
version: "0.1"
about: an example using storage to save and load a file

settings:
    - ArgRequiredElseHelp

args:
    - storage:
        help: storage url
        short: s
        long: storage
        multiple: false
        takes_value: true

subcommands:
    - save:
        about: save a file into the storage
        args:
            - file:
                short: f
                long: file
                help: the file path in disk
                takes_value: true
            - name:
                short: n
                long: name
                help: the name saved in storage
                takes_value: true
    - load:
        about: load a file from the storage
        args:
            - file:
                short: f
                long: file
                help: the file path in disk
                takes_value: true
            - name:
                short: n
                long: name
                help: the name saved in storage
                takes_value: true
"#;

fn main() {
    let yaml = &clap::YamlLoader::load_from_str(CMD).unwrap()[0];
    let matches = App::from_yaml(yaml).get_matches();

    let url = matches.value_of("storage").unwrap();
    let s = external_storage::create_storage(url).unwrap();
    if let Some(matches) = matches.subcommand_matches("save") {
        let file_path = matches.value_of("file").unwrap();
        let name = matches.value_of("name").unwrap();
        let mut f = std::fs::File::open(file_path).unwrap();
        s.write(name, &mut f).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("load") {
        let file_path = matches.value_of("file").unwrap();
        let name = matches.value_of("name").unwrap();
        let mut f = std::fs::File::create(file_path).unwrap();
        let mut reader = s.read(name).unwrap();
        std::io::copy(&mut reader, &mut f).unwrap();
    } else {
        let _ = App::from_yaml(yaml).print_help();
        return;
    }
    println!("done");
}
