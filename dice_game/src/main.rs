use std::io;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut score1 = 0;
    let mut score2 = 0;

    print!("How many points are we playing to? ");

    io::Write::flush(&mut io::stdout()).expect("Flush failed");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut win_value = 100;

    let test = &input.trim().parse::<i32>();
    match test {
        Ok(ok) => win_value = *ok,
        Err(_) => println!("Invalid input. Value defaulting to 100."),
    }

    println!("Lets run it!\n");

    let mut round_count = 1;
    while score1 < win_value && score2 < win_value {
        println!("----------------------Round {}----------------------", round_count);

        score1 += play_round(1, score1, win_value);

        println!("Moving on to Player 2...");
        score2 += play_round(2, score2, win_value);

        println!("Round {} ends with Player 1 at {} points and Player 2 at {} points!\n", round_count, score1, score2);

        round_count += 1;
    }

    println!("-----------------Final Game Results-----------------");

    if score1 >= win_value && score2 >= win_value {
        println!("Both players win the game!");
        println!("Player 1 won with {} points\nPlayer 2 won with {} points!", score1, score2);
    } else if score1 > score2 {
        println!("Player 1 wins the game with {} points!", score1);
        println!("Player 2 was only {} points away!", win_value - score2);
    } else {
        println!("Player 2 wins the game with {} points!", score2);
        println!("Player 1 was only {} points away!", win_value - score1);
    }
}

fn play_round(player: i32, cur_sum: i32, win_val: i32) -> i32 {
    let mut round_sum = 0;

    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        print!("Player {}, do you want to \"roll\" or \"stop\": ", player);

        io::Write::flush(&mut io::stdout()).expect("Flush failed");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.contains("roll") {
            let throw = die.sample(&mut rng);
            print!("You rolled a {}! ", throw);

            if throw == 1 {
                println!("You lost all your points this round!\n");
                round_sum = 0; break;
            } else {
                round_sum += throw;
                println!("You've gained {} points this round!", round_sum);

                if win_val - (cur_sum + round_sum) <= 0 {
                    println!("You can stop now to win the game!\n")
                } else {
                    println!("Your possible total is {} points ({} points away)!\n", cur_sum + round_sum, win_val - (cur_sum + round_sum));
                }
            }
        } else if input.contains("stop") {
            println!("You end the round with {} points. Congratulations!\n", round_sum);
            break;
        } else {
            println!("Invalid input. Please try again.\n");
            continue;
        }
    }

    return round_sum;
}
