// █▀▄ █ █ █ █▀▀ █     █▀▀ █▀█ ██▄█ ▀█▀
// █▀▀ █ ▄▀▄ ██▄ █▄▄   █▀  █▄█ █ ▀█  █

use std::collections::HashMap;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt()]
    text: String,
    #[structopt(
        short = "p",
        long = "prefix",
        help = "prefix each line with this character.",
        default_value = ""
    )]
    prefix: String,
    #[structopt(
        short = "o",
        long = "output",
        help = "output default font to json.",
        default_value = ""
    )]
    output_filepath: String,
    #[structopt(
        short = "i",
        long = "input",
        help = "path to json font.",
        default_value = ""
    )]
    input_filepath: String,
}

fn main() {
    let opt = Cli::from_args();
    let mut font = pixel_font();

    if !opt.input_filepath.is_empty() {
        let file = match std::fs::read_to_string(&opt.input_filepath) {
            Ok(file) => file,
            Err(error) => panic!("Error reading file: {:?}", error),
        };

        font = match serde_json::from_str(file.as_str()) {
            Ok(f) => f,
            Err(error) => panic!("Error deseralizing json: {}", error),
        };
    }

    if !opt.output_filepath.is_empty() {
        let output_str = serde_json::to_string_pretty(&font).expect("oops");
        let mut output = match std::fs::File::create(&opt.output_filepath) {
            Ok(file) => file,
            Err(error) => panic!("Error writing file: {:?}", error),
        };

        match output.write_all(output_str.as_bytes()) {
            Err(error) => panic!("Error writing file: {:?}],", error),
            Ok(_) => {
                println!("Font saved to {}!", &opt.output_filepath)
            }
        };
    }

    let mut buffers: Vec<String> = Vec::new();

    if !opt.prefix.is_empty() {
        for i in 0..font.font_height {
            buffers.insert(i as usize, String::new());
            buffers[i as usize] += &(opt.prefix.to_owned() + " ");
        }
    }

    let mut text = opt.text;

    // TODO: no caps!?
    text = text.to_lowercase();

    for c in text.chars() {
        if c == ' ' {
            for _i in 0..font.space_width {
                for h in 0..font.font_height {
                    buffers[h as usize] += " ";
                }
            }
            continue;
        }

        let symbol = match font.letters.get(&c) {
            None => continue,
            Some(s) => s,
        };

        for h in 0..font.font_height {
            buffers[h as usize] += &symbol.rows[h as usize];

            for _i in 0..font.letter_spacing {
                buffers[h as usize] += " ";
            }
        }
    }

    for buffer in buffers {
        println!("{}", buffer);
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Letter {
    rows: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct Font {
    font_height: u8,
    space_width: u8,
    letter_spacing: u8,
    letters: HashMap<char, Letter>,
}

fn pixel_font() -> Font {
    let mut font: Font = Font {
        font_height: 2,
        letters: HashMap::new(),
        letter_spacing: 1,
        space_width: 2,
    };
    font.letters.insert(
        'a',
        Letter {
            rows: ["█▀▄".into(), "█▀█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'b',
        Letter {
            rows: ["██▀".into(), "█▄█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'c',
        Letter {
            rows: ["█▀▀".into(), "▀▄▄".into()].to_vec(),
        },
    );
    font.letters.insert(
        'd',
        Letter {
            rows: ["█▀█".into(), "█▄▀".into()].to_vec(),
        },
    );
    font.letters.insert(
        'e',
        Letter {
            rows: ["█▀▀".into(), "██▄".into()].to_vec(),
        },
    );
    font.letters.insert(
        'f',
        Letter {
            rows: ["█▀▀".into(), "█▀ ".into()].to_vec(),
        },
    );
    font.letters.insert(
        'g',
        Letter {
            rows: ["█▀▀".into(), "█▄▀".into()].to_vec(),
        },
    );
    font.letters.insert(
        'h',
        Letter {
            rows: ["█ █".into(), "█▀█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'i',
        Letter {
            rows: ["█".into(), "█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'j',
        Letter {
            rows: ["  █".into(), "█▄█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'k',
        Letter {
            rows: ["█ █".into(), "█▀▄".into()].to_vec(),
        },
    );
    font.letters.insert(
        'l',
        Letter {
            rows: ["█  ".into(), "█▄▄".into()].to_vec(),
        },
    );
    font.letters.insert(
        'm',
        Letter {
            rows: ["██▄██".into(), "█ ▀ █".into()].to_vec(),
        },
    );
    font.letters.insert(
        'n',
        Letter {
            rows: ["██▄█".into(), "█ ▀█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'o',
        Letter {
            rows: ["█▀█".into(), "█▄█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'p',
        Letter {
            rows: ["█▀▄".into(), "█▀▀".into()].to_vec(),
        },
    );
    font.letters.insert(
        'q',
        Letter {
            rows: ["█▀█".into(), "▀▀█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'r',
        Letter {
            rows: ["█▀█".into(), "█▀▄".into()].to_vec(),
        },
    );
    font.letters.insert(
        's',
        Letter {
            rows: ["█▀▀".into(), "▄██".into()].to_vec(),
        },
    );
    font.letters.insert(
        't',
        Letter {
            rows: ["▀█▀".into(), " █ ".into()].to_vec(),
        },
    );
    font.letters.insert(
        'u',
        Letter {
            rows: ["█ █".into(), "█▄█".into()].to_vec(),
        },
    );
    font.letters.insert(
        'v',
        Letter {
            rows: ["█ █".into(), "▀▄▀".into()].to_vec(),
        },
    );
    font.letters.insert(
        'w',
        Letter {
            rows: ["█ ▄ █".into(), "▀▄▀▄▀".into()].to_vec(),
        },
    );
    font.letters.insert(
        'x',
        Letter {
            rows: ["█ █".into(), "▄▀▄".into()].to_vec(),
        },
    );
    font.letters.insert(
        'y',
        Letter {
            rows: ["█ █".into(), " █ ".into()].to_vec(),
        },
    );
    font.letters.insert(
        'z',
        Letter {
            rows: ["▀▀█".into(), "██▄".into()].to_vec(),
        },
    );
    font
}
