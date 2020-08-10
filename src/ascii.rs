use colored::*;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;

pub fn logo() {
    let number_of_stars = 4;
    let star = ["▄ ██╗▄", " ████╗", "▀╚██╔▀", "  ╚═╝ "];

    let logo_rows = 6;

    let gta_logo = vec![
        "   ██████╗ ████████╗ █████╗",
        "  ██╔════╝ ╚══██╔══╝██╔══██╗",
        "  ██║  ███╗   ██║   ███████║",
        "  ██║   ██║   ██║   ██╔══██║",
        "  ╚██████╔╝   ██║   ██║  ██║",
        "   ╚═════╝    ╚═╝   ╚═╝  ╚═╝",
    ]
    .iter()
    .map(|e| e.white())
    .collect::<Vec<ColoredString>>();

    let five_logo = vec![
        " ▀████▀  ▀███▀",
        "  ███║   ██║ ",
        "  ███║   ██║ ",
        "  ╚██║  ▄█╔╝ ",
        "   ╚█████╔╝  ",
        "    ╚════╝   ",
    ]
    .into_iter()
    .map(|e| e.green().dimmed())
    .collect::<Vec<ColoredString>>();

    let solo_logo = vec![
        "   ███████╗ ██████╗ ██╗      ██████╗",
        "   ██╔════╝██╔═══██╗██║     ██╔═══██╗",
        "   ███████╗██║   ██║██║     ██║   ██║",
        "   ╚════██║██║   ██║██║     ██║   ██║",
        "   ███████║╚██████╔╝███████╗╚██████╔╝",
        "   ╚══════╝ ╚═════╝ ╚══════╝ ╚═════╝ ",
    ]
    .into_iter()
    .map(|e| e.white())
    .collect::<Vec<ColoredString>>();

    let logo = vec![gta_logo, five_logo, solo_logo];

    print!("\n");

    let max_number_of_stars = 5;
    let static_stars_distribution = Uniform::new(1, max_number_of_stars);

    let mut rng = rand::thread_rng();

    let static_stars = static_stars_distribution.sample(&mut rng);
    let blinking_stars = max_number_of_stars - static_stars;

    for i in 0..4 {
        for ii in 0..static_stars {
            if ii == 0 {
                print!("{:>59}  ", star[i].black().dimmed().dimmed());
            } else {
                print!("{}  ", star[i].black().dimmed().dimmed())
            }
        }

        for _ in 0..blinking_stars {
            print!("{}  ", star[i].blink().white());
        }

        print!("\n");
    }

    print!("\n");

    for i in 0..logo_rows {
        for elem in &logo {
            print!("{}  ", elem[i]);
        }

        print!("\n");
    }

    print!("\n");
    lester_quotes();
}

fn lester_quotes() {
    let quotes = vec![
        "I take it this isn't purely social call ?\n\t Of course not... you're in trouble and you need my help... no problem.",
        "Oh can't handle the heat my friend?\n\t Alright, leave it to creepy Uncle Lester.",
        "Ok so you are in serious trouble and you need my help.\n\t Okay... Okay..."
    ];

    println!(
        "\t{}",
        &format!(
            "“{}”\n",
            quotes.choose(&mut rand::thread_rng()).unwrap_or(&"")
        )
        .black()
        .bold()
    );
}
