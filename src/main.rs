use clap::{App, Arg};

fn main() {
    let matches = App::new("git local-ignore")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Vyacheslav P. <vyacheslav.pukhanov@gmail.com>")
        .about("Locally exclude files from Git index")
        .arg(
            Arg::with_name("list")
                .short('l')
                .long("list")
                .about("List currently excluded files"),
        )
        .arg(
            Arg::with_name("file")
                .index(1)
                .multiple_values(true)
                .about("Files to exclude from index"),
        )
        .get_matches();

    if matches.is_present("list") {
        println!("List mode");
    } else {
        println!("Add mode");
    }
}
