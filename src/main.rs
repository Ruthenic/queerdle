#![feature(variant_count)]

use pronouns::Pronouns;

use inquire::{
    validator::{ErrorMessage, Validation},
    Text,
};
use rand::{seq::SliceRandom, Rng};
use regex::Regex;

mod pronouns;

const THINGIES: [&str; 9] = [
    "pussy", "dick", "violence", "yuri", "yaoi", "vibes", "headpats", "kisses", "love",
];
const HUMAN_OR_LACKTHEREOF: [&str; 17] = [
    "dog",
    "cat",
    "system",
    "catboy",
    "catgirl",
    "fox",
    "foxboy",
    "foxgirl",
    "dragon",
    "dragonboy",
    "dragongirl",
    "demon",
    "demonboy",
    "demongirl",
    "robot",
    "roboboy",
    "robogirl",
];
const HYPERFIXATIONS: [&str; 25] = [
    "toki pona",
    "furry",
    "anime",
    "manga",
    "kpop",
    "jpop",
    "vocaloid",
    "minecraft",
    "roblox",
    "danganronpa",
    "homestuck",
    "undertale",
    "fnaf",
    "steven universe",
    "my little pony",
    "pokemon",
    "yugioh",
    "magic the gathering",
    "dungeons and dragons",
    "world of warcraft",
    "league of legends",
    "fortnite",
    "overwatch",
    "minecraft",
    "indie horror",
];

// queerdle: wordle, but for chronically online queer people :3
fn main() {
    let re = Regex::new(r"(.*/.*) (.*) from a (.*) with a (.*) hyperfixation").unwrap();

    let mut rng = rand::thread_rng();

    let pronouns = rng.gen::<Pronouns>();
    let thingy = *THINGIES.choose(&mut rng).unwrap();
    let human_or_lackthereof = *HUMAN_OR_LACKTHEREOF.choose(&mut rng).unwrap();
    let hyperfixation = *HYPERFIXATIONS.choose(&mut rng).unwrap();

    let full_answer = format!(
        "{} {} from a {} with a {} hyperfixation",
        pronouns, thingy, human_or_lackthereof, hyperfixation
    );

    //println!("DBG: {}", full_answer);
    println!("Please make sure to submit your guesses in the following format: \n\t<pronouns> <thingy> from a <human or lackthereof> with a <hyperfixation> hyperfixation");

    for _ in 0..5 {
        let guess = Text::new("What is your guess?")
            .with_validator(|input: &str| {
                let re = Regex::new(r"(.*/.*) (.*) from a (.*) with a (.*) hyperfixation").unwrap();
                if re.clone().is_match(input) {
                    Ok(Validation::Valid)
                } else {
                    Ok(Validation::Invalid(ErrorMessage::Custom(
                        "Invalidly formatted guess. Please try again.".to_string(),
                    )))
                }
            })
            .prompt()
            .unwrap();

        let caps = re.captures(&guess).unwrap();

        let guessed_pronouns = Pronouns::from(caps.get(1).unwrap().as_str().to_string());
        let guessed_thingy = caps.get(2).unwrap().as_str();
        let guessed_human_or_lackthereof = caps.get(3).unwrap().as_str();
        let guessed_hyperfixation = caps.get(4).unwrap().as_str();

        /* println!(
            "DBG: {} {} from a {} with a {} hyperfixation",
            guessed_pronouns, guessed_thingy, guessed_human_or_lackthereof, guessed_hyperfixation
        ); */

        let mut correct = true;

        if guessed_pronouns.first == pronouns.first {
            print!("🟩")
        } else if guessed_pronouns.first == pronouns.second {
            correct = false;
            print!("🟨")
        } else {
            correct = false;
            print!("🟥")
        }

        if guessed_pronouns.second == pronouns.second {
            print!("🟩")
        } else if guessed_pronouns.second == pronouns.first {
            correct = false;
            print!("🟨")
        } else {
            correct = false;
            print!("🟥")
        }

        if guessed_thingy == thingy {
            print!("🟩")
        } else {
            correct = false;
            print!("🟥")
        }

        if guessed_human_or_lackthereof == human_or_lackthereof {
            print!("🟩")
        } else if guessed_human_or_lackthereof.contains(human_or_lackthereof)
            || human_or_lackthereof.contains(guessed_human_or_lackthereof)
        {
            correct = false;
            print!("🟨");
        } else {
            correct = false;
            print!("🟥")
        }

        if guessed_hyperfixation == hyperfixation {
            print!("🟩")
        } else {
            correct = false;
            print!("🟥")
        }

        println!();

        if correct {
            return;
        }
    }
    println!("you are dumb and stupid and wrong and bad and you should feel bad");
    println!("real answer: {}", full_answer);
}
