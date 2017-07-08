use std::io::BufRead;
use std::option::Option;
use std::process::Command;

struct Game {
    word: String,
    guesses: Vec<String>,
    filled: Vec<String>,
    wrong_guesses: usize,
}

impl Game {
    fn draw_gallows(n:usize) {
        let gallows = format!("
    __________
    |        |
    |        {}
    |       {}{}{}
    |       {} {}
    |
    |
---------",
            if n >= 1 {"0"} else {" "},
            if n >= 3 {"/"} else {" "},
            if n >= 2 {"|"} else {" "},
            if n >= 4 {"\\"} else {" "},
            if n >= 5 {"/"} else {" "},
            if n >= 6 {"\\"} else {" "}).to_string();

        println!("{}", gallows);
    }

    fn draw_game_state(&self) {
        Game::draw_gallows(self.wrong_guesses);
        println!("{}", self.filled.join(" "));
        if self.guesses.len() > 0 {
            println!("guesses: {}", self.guesses.join(""));
        }
    }

    fn status(&self) -> Option<bool> {
        let mut success = true;
        for c in self.filled.iter() {
            if c == "_" {
                success = false;
                break;
            }
        }
        if success {
            return Some(true);
        } else if self.wrong_guesses >= 7 {
            return Some(false);
        } else {
            return None;
        }
    }

    fn play(&mut self) -> bool {
        loop {
            self.draw_game_state();
            let status = self.status();
            match status {
                Some(x) => {
                    println!("you {}!", if x {"won"} else {"lost"});
                    if !x {
                        println!("the word was {}", self.word);
                    }
                    return x;
                }
                _ => {}
            }

            self.step();
        }
    }

    fn step(&mut self) {
        let stdin = std::io::stdin();
        let input = stdin.lock().lines().next().unwrap().unwrap();
        self.guess(input);
    }

    fn guess(&mut self, c:String) {
        let guesses = &mut self.guesses;
        let mut chars = self.word.chars();
        let mut correct = false;
        for i in 0..self.word.len() {
            let char = chars.next();
            if self.filled[i] == "_" {
                if char.unwrap().to_string() == c {
                    self.filled[i] = c.clone();
                    correct = true;
                }
            }
        }
        if !correct {
            self.wrong_guesses += 1;
        }
        guesses.push(c);
    }
}

fn main() {
    // holy shit
    let random_word_output = Command::new("perl").arg("-e").arg("open IN, '</usr/share/dict/words';rand($.) < 1 && ($n=$_) while <IN>;print $n").output();
    let stdout = random_word_output.unwrap().stdout;
    let raw = String::from_utf8(stdout).unwrap();
    let random_word = raw.trim().to_string();

    let random_word_len = random_word.len();
    let mut game = Game {word: random_word, guesses: Vec::new(), filled: vec!["_".to_string(); random_word_len], wrong_guesses: 0};
    game.play();
}
