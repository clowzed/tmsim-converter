use structopt::StructOpt;
use std::io::prelude::*;


#[derive(StructOpt, Debug)]
#[structopt(name = "tmsim-converter", about = "Converter of human readable turing machine commands into json")]
struct Options
{
    #[structopt(parse(from_os_str))]
    source: std::path::PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    out: Option<std::path::PathBuf>,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct Command
{
    pub state        : usize,
    pub next_state   : usize,
    pub reading_char : char,
    pub place_char   : char, 
    pub next_move    : String,
}


#[derive(serde::Serialize)]
struct TMachineConfiguration
{
    commands: std::vec::Vec<Command>,
    alphabet: Option<String>,
    tape: Option<String>,
}

fn parse_alphabet_or_tape(line: &str, is_tape: bool) -> String
{
    let uncleaned = line.split('(').collect::<Vec<_>>()[1].trim_end_matches(')');
    let mut chars: Vec<char>= uncleaned.chars().collect();
    if !is_tape
    {
        chars.sort_unstable();
        chars.dedup();
    }
    String::from_iter(chars.iter())
}

fn parse_command(line: &str) -> Command
{
    let parts = line.split("->").collect::<Vec<_>>();
    let left_part = parts[0].trim().trim_start_matches('q');
    let right_part = parts[1].trim().trim_start_matches('q');

    let left_split = left_part.split('(').collect::<Vec<_>>();
    let state_num:usize = left_split[0].trim().parse().unwrap();
    let symbol = left_split[1].trim_end_matches(')').chars().collect::<Vec<_>>()[0];

    let right_split = right_part.split('(').collect::<Vec<_>>();
    let new_state_num:usize = right_split[0].trim().parse().unwrap();

    let sym_move_split = right_split[1].split(')').collect::<Vec<_>>();
    let new_symbol = sym_move_split[0].chars().collect::<Vec<_>>()[0];
    let movment = match sym_move_split[1].chars().collect::<Vec<_>>()[0]
    {
        'R' => "Right",
        'L' => "Left",
        'S' => "Stop",
        _ => "Stop" //? Just for sure
    }.to_string();

    Command{state: state_num, next_state: new_state_num, reading_char: symbol, place_char: new_symbol, next_move: movment}

}

fn main()
{
    let options = Options::from_args();

    let mut tmachineconf = TMachineConfiguration{
        commands: std::vec::Vec::new(),
        alphabet: None,
        tape: None
    };

    let command_pattern = regex::Regex::new(r"^q\d+[(].[)] -> q\d+[(].[)](R|L|S)$").unwrap();
    let alphabet_pattern = regex::Regex::new(r"^alphabet: *[(].*[)]$").unwrap();
    let tape_pattern = regex::Regex::new(r"^tape: *[(][*].*[)]$").unwrap();



    if !options.source.exists()
    {
        eprintln!("Specified file does not exists!");
        std::process::exit(1);
    }

    let opened_source = match std::fs::File::open(&options.source)
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file! Reason: {}", e);
            std::process::exit(2);
        }
    };

    let bufreader = std::io::BufReader::new(opened_source);


    for line in bufreader.lines()
    {
        let line = match line 
        {
            Ok(line) => line.trim().to_string(),
            Err(e) => 
            {
                eprintln!("Failed to read next line! Reason: {}", e);
                std::process::exit(3);
            }
        };


        if command_pattern.is_match(&line)
        {
            let new_command = parse_command(&line);
            tmachineconf.commands.push(new_command);
        }
        else if alphabet_pattern.is_match(&line)
        {
            tmachineconf.alphabet = Some(parse_alphabet_or_tape(&line, false));
        }
        else if tape_pattern.is_match(&line)
        {
            tmachineconf.tape = Some(parse_alphabet_or_tape(&line, true));
        }
    }

    if tmachineconf.alphabet.is_none()
    {
        eprintln!("No alphabet was provided!");
        std::process::exit(4);
    }

    if tmachineconf.tape.is_none()
    {
        eprintln!("No tape was provided!");
        std::process::exit(5);
    }

    if options.out.is_none()
    {
        match serde_json::to_string_pretty(&tmachineconf)
        {
            Ok(s) => println!("{}", s),
            Err(e) => {
                eprintln!("Error occured while converting to json! Reason: {}", e);
                std::process::exit(6)
            }
        }
    }
    else
    {
        let file = match std::fs::File::create(&options.out.unwrap())
            {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open file for output! Reason: {}", e);
                    std::process::exit(7);
                }
            };
            match serde_json::to_writer(&file, &tmachineconf)
            {
                Ok(()) => {},
                Err(e) => 
                {
                    eprintln!("Failed to save json to file! Reason: {}", e);
                    std::process::exit(8);
                }
            }
    }

}