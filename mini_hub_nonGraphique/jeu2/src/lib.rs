use rand::Rng;
use std::io;
use std::io::Write;
use std::time::Instant;

enum GamePhase {
    Phase1,
    Phase2,
}

pub fn play_game2() -> i64 {
    ui::clear_terminal();

    let mut game = Game::new();
    game.run(&GamePhase::Phase1);
    ui::clear_terminal();
    if !game.phase1_reaction_times.is_empty() {
        game.run(&GamePhase::Phase2);
    }

    println!(
        "Fin du jeu. Score: {}, Temps de réaction moyen: {} ms",
        game.score,
        game.average_reaction_time()
    );
    game.score as i64 * 10
}

struct Game {
    words: Vec<&'static str>,
    words_phase2: Vec<&'static str>,
    score: i32,
    total_reaction_time: u64,
    phase1_reaction_times: Vec<u128>,
    target_reaction_time: u128,
}

impl Game {
    fn new() -> Game {
        Game {
            words: vec!["gauche", "droite"],
            words_phase2: vec!["gauche", "droite", "GAUCHE", "DROITE"],
            score: 0,
            total_reaction_time: 0,
            phase1_reaction_times: Vec::new(),
            target_reaction_time: 0,
        }
    }

    fn run(&mut self, phase: &GamePhase) {
        self.display_instructions(&phase);
        self.run_phase(&phase);
    }

    fn run_phase(&mut self, phase: &GamePhase) {
        let mut rng = rand::thread_rng();
        match phase {
            GamePhase::Phase1 => {
                for _ in 0..10 {
                    let random_word = self.words[rng.gen_range(0..self.words.len())];
                    let reaction_time = self.display_word(random_word, phase);
                    self.total_reaction_time += reaction_time;
                    wait_for_enter();
                }
            }
            GamePhase::Phase2 => {
                for _ in 0..10 {
                    let random_word = self.words_phase2[rng.gen_range(0..self.words_phase2.len())];
                    let reaction_time = self.display_word(random_word, phase);
                    self.total_reaction_time += reaction_time;
                    wait_for_enter();
                }
            }
        }
    }

    fn display_word(&mut self, word: &str, phase: &GamePhase) -> u64 {
        let start_time = Instant::now();

        ui::clear_terminal();
        self.display_highlighted_word(word);
        let key_pressed = get_key_press();

        let reaction_time = start_time.elapsed().as_millis();
        if self.is_correct_key(word, key_pressed) {
            match phase {
                GamePhase::Phase1 => {
                    println!("Correct! Temps de réaction: {} ms", reaction_time);
                    self.score += 1;
                    self.phase1_reaction_times.push(reaction_time)
                }
                GamePhase::Phase2 => {
                    if reaction_time <= self.target_reaction_time {
                        println!("Correct! Temps de réaction: {} ms", reaction_time);
                        self.score += 1;
                    } else {
                        println!(
                            "Correct mais temps de réaction trop lent ({} ms contre {} attendus) !",
                            reaction_time, self.target_reaction_time
                        );
                    }
                }
            }
        } else {
            println!("Incorrect! Temps de réaction: {} ms", reaction_time);
        }
        reaction_time as u64
    }

    fn display_instructions(&mut self, phase: &GamePhase) {
        match phase {
            GamePhase::Phase1 => {
                println!("Bienvenue dans le jeu de réaction en Rust!");
                println!("Appuyez le plus vite possible sur la flèche droite sur le mot 'droite' et gauche sur le mot 'gauche'");
                println!("Ensuite, appuyez sur la touche 'Entrée' pour valider votre choix !");
            }
            GamePhase::Phase2 => {
                self.target_reaction_time = self.phase1_reaction_times.iter().sum::<u128>()
                    / self.phase1_reaction_times.len() as u128;
                // Augmenter sa valeur de 33%
                self.target_reaction_time += self.target_reaction_time / 3;
                println!("A présent, nous entrons dans la phase 2 de l'exercice");
                println!("Maintenant, si les mots 'DROITE' et 'GAUCHE' sont affichés en majuscule, appuyez sur les touches inverses !");
                println!("Si les mots sont en minuscule, appuyez sur les touches normales.");
                println!("Comme avant, appuyez sur la touche 'Entrée' pour valider votre choix !");
                println!("Attention, basé sur vos scores durant la phase 1, il faudra répondre en moins de {} millisecondes !", self.target_reaction_time);
            }
        }
        wait_for_enter();
    }

    fn display_highlighted_word(&self, word: &str) {
        let styled_word = format!("*{}*", word);
        println!("{}", styled_word);
    }

    fn is_correct_key(&self, word: &str, key_pressed: char) -> bool {
        match key_pressed.to_string().as_str() {
            "d" if word == "droite" => true,
            "q" if word == "gauche" => true,
            "d" if word == "GAUCHE" => true,
            "q" if word == "DROITE" => true,
            _ => false,
        }
    }

    fn average_reaction_time(&self) -> u64 {
        if self.score == 0 {
            0
        } else {
            self.total_reaction_time / self.score as u64
        }
    }
}

fn wait_for_enter() {
    print!("Appuyez sur Entrée pour continuer...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn get_key_press() -> char {
    print!("Appuyez sur la touche correspondante...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().chars().next().unwrap_or('\0')
}
