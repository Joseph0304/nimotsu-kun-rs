extern crate clap;

mod stage;

use clap::{App, Arg};
use console::{Key, Term};
use std::fs;

use stage::Stage;

static STAGE: &str = "\
########\n\
# .. p #\n\
# oo   #\n\
#      #\n\
########\
";

fn draw(term: &Term, stage: &Stage) {
    term.clear_screen().unwrap();
    let draw_text = stage.to_string();
    for h in 0..stage.height() {
        term.write_line(&draw_text[(h * stage.width())..((h + 1) * stage.width())])
            .unwrap();
    }
}

fn game_loop(term: &Term, stage: &mut Stage) {
    // First draw
    {
        draw(&term, &stage);
        term.flush().unwrap();
    }
    loop {
        if stage.check_clear() {
            term.write_line("Congratulations!").unwrap();
            return;
        }
        let key = term.read_key().unwrap();
        if let Key::Char('q') = key {
            return;
        }
        let (dx, dy) = match key {
            Key::Char('w') => (0, -1),
            Key::Char('a') => (-1, 0),
            Key::Char('s') => (0, 1),
            Key::Char('d') => (1, 0),
            _ => (0, 0),
        };
        stage.update(dx, dy);
        draw(&term, &stage);
        term.flush().unwrap();
    }
}

fn main() {
    let matches = App::new("nimotsukun")
        .version("0.1")
        .about("Nimotsu-kun")
        .arg(
            Arg::with_name("stage")
                .short("f")
                .long("file-name")
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();

    let stage = match matches.value_of("stage") {
        Some(file) => {
            let stage = fs::read_to_string(file).unwrap();
            String::from(&stage[..stage.len() - 1]) // remove new line
        }
        None => String::from(STAGE),
    };

    let mut stage = match stage.as_str().parse() {
        Ok(s) => s,
        Err(_) => {
            eprintln!("stage parse error");
            return;
        }
    };
    let term = Term::buffered_stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();
    term.flush().unwrap();
    game_loop(&term, &mut stage);
    term.show_cursor().unwrap();
    term.flush().unwrap();
}
