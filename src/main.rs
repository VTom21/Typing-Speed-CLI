use rand::Rng;
use std::fs::{self, read_to_string};
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};
use std::{process, thread};

// Added crates for CLI design
use colored::*;
use prettytable::{Table, row, cell};
use indicatif::{ProgressBar, ProgressStyle};
use clearscreen;


fn input(prompt: &str) -> String {
    print!("{}", prompt.yellow().bold()); 
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}

fn main() {

    let mut file = fs::File::open("output.txt").unwrap();
    let mut sentences: Vec<&str> = vec![];
    let mut total_words: Vec<String> = Vec::new();
    let mut correct_words = 0; 
    let time = 60;
    let time_float = time as f32;
    let minutes: f32 = (time_float / 60.0) as f32;

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    sentences = content.lines().collect();


    let pb = ProgressBar::new(time);
    pb.set_style(ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len}s")
        .unwrap()
        .progress_chars("=>-"));

    let start = Instant::now();

    loop {
        clearscreen::clear().unwrap();

        println!("\n{}", "=== Typing Test ===".bright_green().bold());


        let mut rnd = rand::thread_rng();
        let rnd_index = rnd.gen_range(0..sentences.len());
        println!("{}\n", sentences[rnd_index].cyan().bold());


        let input = input("Write the sentence here: ");

        let sentence_words: Vec<&str> = sentences[rnd_index].split_whitespace().collect();
        let user_words: Vec<&str> = input.split_whitespace().collect();


        let mut correct_in_sentence = 0;
        print!("Feedback: ");
        for (user_word, correct_word) in user_words.iter().zip(sentence_words.iter()) {
            if user_word == correct_word {
                print!("{} ", user_word.green());
                correct_in_sentence += 1;
            } else {
                print!("{} ", user_word.red().bold());
            }
        }
        println!();

        correct_words += correct_in_sentence;
        total_words.extend(input.split_whitespace().map(String::from));


        let elapsed_secs = start.elapsed().as_secs();
        pb.set_position(elapsed_secs.min(time as u64));


        if start.elapsed() >= Duration::from_secs(time) {
            pb.finish_with_message("Time's up!");
            clearscreen::clear().unwrap();
            println!("{}", "=== Test Finished ===".bright_green().bold());

            println!("{} total words!", total_words.len());
            println!("{} of correct words!", correct_words);

            let accuracy = correct_words as f32 / total_words.len() as f32;
            let errors = total_words.len() - correct_words;
            let wpm = total_words.len() as f32 / minutes;
            let adjusted_wpm = (total_words.len() as f32 / minutes) * accuracy as f32;
            let net_wpm = (total_words.len() as f32 - errors as f32) / minutes;


            let mut table = Table::new();
            table.add_row(row![bFg => "Metric", "Value"]);
            table.add_row(row!["Total Words", total_words.len()]);
            table.add_row(row!["Correct Words", correct_words]);
            table.add_row(row!["Errors", errors]);
            table.add_row(row!["Accuracy", format!("{:.2}%", accuracy * 100.0)]);
            table.add_row(row!["Raw WPM", format!("{:.2}", wpm)]);
            table.add_row(row!["Adjusted WPM", format!("{:.2}", adjusted_wpm)]);
            table.add_row(row!["Net WPM", format!("{:.2}", net_wpm)]);

            table.printstd();

            process::exit(0);
        }    
    }
}
