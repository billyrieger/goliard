#[macro_use]
extern crate failure;

#[derive(Debug, Fail)]
enum Error {
    #[fail(display = "term size unavailable")]
    TermSizeUnavailable,
}

fn run(siv: &mut cursive::Cursive, filename: &str) -> Result<(), failure::Error> {
    let (term_width, term_height) = term_size::dimensions().ok_or(Error::TermSizeUnavailable)?;
    let life = smeagol::Life::from_rle_file(filename)?;

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
        .arg(clap::Arg::with_name("FILE").index(1).required(true))
        .get_matches();

    let mut siv = cursive::Cursive::ncurses();
    run(&mut siv, matches.value_of("FILE").unwrap())?;

    Ok(())
}
