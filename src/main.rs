#[macro_use]
extern crate failure;

use std::io::Read;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "term size unavailable")]
    TermSizeUnavailable,
}

fn run(siv: &mut cursive::Cursive, life: smeagol::Life) -> Result<(), failure::Error> {
    let (term_width, term_height) = term_size::dimensions().ok_or(Error::TermSizeUnavailable)?;

    let state = goliard::State::new_centered(life, term_width as u64, (term_height - 1) as u64);

    goliard::views::add_main_view(siv, &state);
    goliard::key::setup_key_commands(siv, &state);
    goliard::start_smeagol_thread(siv, &state);

    siv.run();

    Ok(())
}

fn main() -> Result<(), failure::Error> {
    let matches = clap::App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            clap::Arg::with_name("FILE")
                .index(1)
                .required_unless("STDIN")
                .help("The file to load. Not required if reading a file from stdin."),
        )
        .arg(
            clap::Arg::with_name("STDIN")
                .short("s")
                .long("stdin")
                .help("Reads a file from stdin"),
        )
        .get_matches();

    let life = if matches.is_present("STDIN") {
        let mut buffer = vec![];
        let stdin = std::io::stdin();
        stdin.lock().read_to_end(&mut buffer)?;
        smeagol::Life::from_rle_file_contents(&buffer)?
    } else {
        let filename = matches.value_of("FILE").unwrap();
        smeagol::Life::from_rle_file(filename)?
    };

    let mut siv = cursive::Cursive::ncurses();
    run(&mut siv, life)?;

    Ok(())
}
