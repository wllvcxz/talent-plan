use clap::{App, AppSettings, Arg, SubCommand};
use kvs::KvStore;
use std::process::exit;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true))
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();
    
    let mut kv = KvStore::new();
    match matches.subcommand() {
        ("set", Some(_matches)) => {
            let k:String = _matches.value_of("KEY").unwrap().to_string();
            let v:String = _matches.value_of("VALUE").unwrap().to_string();
            kv.set(k, v);
            eprintln!("unimplemented");
            exit(1);
        }
        ("get", Some(_matches)) => {
            let k:String = _matches.value_of("KEY").unwrap().to_string();
            kv.get(k);            
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_matches)) => {
            let k:String = _matches.value_of("KEY").unwrap().to_string();
            kv.remove(k);
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
