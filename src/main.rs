
// █▀▄ █ █ █ █▀▀ █     █▀▀ █▀█ ██▄█ ▀█▀ 
// █▀▀ █ ▄▀▄ ██▄ █▄▄   █▀  █▄█ █ ▀█  █  

use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt()]
    comment_char: String, 
    #[structopt()]
    text: String
}

fn main() {
    let opt = Cli::from_args();
    // TODO: Load font from JSON
    let font = pixel_font();

    // TODO: Fix dynamic linebuffers
    let mut buffer1: String = String::new();
    let mut buffer2: String = String::new();
    let mut text = opt.text;

    // TODO: no caps!
    text = text.to_lowercase();

    buffer1 += &(opt.comment_char.to_owned() + &" ");
    buffer2 += &(opt.comment_char.to_owned() + &" ");

    for c in text.chars() {
        if c == ' ' {
            buffer1 += "  ";
            buffer2 += "  ";
            continue;
        }

        let symbol = match font.letters.get(&c) {
            None => continue, 
            Some(s) => s
        };
        buffer1 += " ";
        buffer2 += &font.letters.get(&c).unwrap().rows[1];
        buffer2 += " ";
    }

    println!("{}", buffer1);
    println!("{}", buffer2);
}


struct Letter {
    rows: Vec<String>
}

struct Font {
    letters: HashMap<char, Letter>
}

fn pixel_font() -> Font {
    let mut font: Font = Font{
        letters: HashMap::new()
    };
    font.letters.insert('a', Letter{
        rows: [
            "█▀▄".into(), 
            "█▀█".into() 
        ].to_vec()
    });
    font.letters.insert('b', Letter{
        rows: [
            "██▀".into(), 
            "█▄█".into() 
        ].to_vec()
    });
    font.letters.insert('c', Letter{
        rows: [
            "█▀▀".into(), 
            "▀▄▄".into() 
        ].to_vec()
    });
    font.letters.insert('d', Letter{
        rows: [
            "█▀█".into(), 
            "█▄▀".into() 
        ].to_vec()
    });
    font.letters.insert('e', Letter{
        rows: [
            "█▀▀".into(), 
            "██▄".into() 
        ].to_vec()
    });
    font.letters.insert('f', Letter{
        rows: [
            "█▀▀".into(), 
            "█▀ ".into() 
        ].to_vec()
    });
    font.letters.insert('g', Letter{
        rows: [
            "█▀▀".into(), 
            "█▄▀".into() 
        ].to_vec()
    });
    font.letters.insert('h', Letter{
        rows: [
            "█ █".into(), 
            "█▀█".into() 
        ].to_vec()
    });
    font.letters.insert('i', Letter{
        rows: [
            "█".into(), 
            "█".into() 
        ].to_vec()
    });
    font.letters.insert('j', Letter{
        rows: [
            "  █".into(), 
            "█▄█".into() 
        ].to_vec()
    });
    font.letters.insert('k', Letter{
        rows: [
            "█ █".into(), 
            "█▀▄".into() 
        ].to_vec()
    });
    font.letters.insert('l', Letter{
        rows: [
            "█  ".into(), 
            "█▄▄".into() 
        ].to_vec()
    });
    font.letters.insert('m', Letter{
        rows: [
            "██▄██".into(), 
            "█ ▀ █".into() 
        ].to_vec()
    });
    font.letters.insert('n', Letter{
        rows: [
            "██▄█".into(), 
            "█ ▀█".into() 
        ].to_vec()
    });
    font.letters.insert('o', Letter{
        rows: [
            "█▀█".into(), 
            "█▄█".into() 
        ].to_vec()
    });
    font.letters.insert('p', Letter{
        rows: [
            "█▀▄".into(), 
            "█▀▀".into() 
        ].to_vec()
    });
    font.letters.insert('q', Letter{
        rows: [
            "█▀█".into(), 
            "▀▀█".into() 
        ].to_vec()
    });
    font.letters.insert('r', Letter{
        rows: [
            "█▀█".into(), 
            "█▀▄".into() 
        ].to_vec()
    });
    font.letters.insert('s', Letter{
        rows: [
            "█▀▀".into(), 
            "▄██".into() 
        ].to_vec()
    });
    font.letters.insert('t', Letter{
        rows: [
            "▀█▀".into(), 
            " █ ".into() 
        ].to_vec()
    });
    font.letters.insert('u', Letter{
        rows: [
            "█ █".into(), 
            "█▄█".into() 
        ].to_vec()
    });
    font.letters.insert('v', Letter{
        rows: [
            "█ █".into(), 
            "▀▄▀".into() 
        ].to_vec()
    });
    font.letters.insert('w', Letter{
        rows: [
            "█ ▄ █".into(), 
            "▀▄▀▄▀".into() 
        ].to_vec()
    });
    font.letters.insert('x', Letter{
        rows: [
            "█ █".into(), 
            "▄▀▄".into() 
        ].to_vec()
    });
    font.letters.insert('y', Letter{
        rows: [
            "█ █".into(), 
            " █ ".into() 
        ].to_vec()
    });
    font.letters.insert('z', Letter{
        rows: [
            "▀▀█".into(), 
            "██▄".into() 
        ].to_vec()
    });
    return font;
}
// ▄▀▀█ █▀▀▄ █▀▀▄ █▀▀▄  ▄▄  ▀▀█ █ █ █ █▀█	                   		  =
// █▄▄█ █▀▀█ █  ▄ █  █  █ █▀█ █ █▀█	                      	  =
// ▀  ▀ ▀▀▀▀ ▀▀▀▀ ▀▀▀  ▀▀▀▀
// ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
//
//  █▀▄ ██▀ █▀▀ █▀█ █▀▀ █▀▀ █▀▀ █ █ █   █ █ █ █   ██▄██ ██▄█ █▀█ █▀▄ █▀█ █▀█ █▀▀ ▀█▀ █ █ █ █ █ ▄ █ █ █ █ █ ▀▀█ 
//  █▀█ █▄█ ▀▄▄ █▄▀ ██▄ █▀  █▄▀ █▀█ █ █▄█ █▀▄ █▄▄ █ ▀ █ █ ▀█ █▄█ █▀▀ ▀▀█ █▀▄ ▄██  █  █▄█ ▀▄▀ ▀▄▀▄▀ ▄▀▄  █  ██▄ 
//                                                                        
//
// █▀▀  a
// ▄██
