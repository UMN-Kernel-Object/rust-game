mod tool;
mod maze;
use std::io;
use rand::Rng;

use std::time::Duration;
use std::thread::sleep;
fn main() {
    
    let mut poke1 = tool::Poke {
        Name: String::from("RUST_BUCKET"),
        HP: 50,
        Attack: (String::from("Dump"), 20),
        potions: 3,
        level : 1,
        xp : 0,
    };

    let mut poke2 = tool::Poke {
        Name: String::from("RUSTOLEUM"),
        HP: 75,
        Attack: (String::from("Paint Spray"), 10),
        potions: 2,
        level : 1,
        xp : 0,
    };

    let mut poke3 = tool::Poke {
        Name: String::from("cRUSTy"),
        HP: 60,
        Attack: (String::from("Corrode"), 15),
        potions: 4,
        level : 1,
        xp : 0,
    };

    let mut punishment_poke = tool::Poke{
        Name: String::from("Incorrect Input"),
        HP: 10,
        Attack: (String::from("Wrong Input"), 3),
        potions: 0,
        level : 1,
        xp: -10000,
    };

   // let start_pos = (12,0);

    println!("Three Pokes are displayed before you\n");
    println!("Which one will you choose?\n");
    let time = Duration::from_secs(1);
    sleep(time);

    println!("RUST_BUCKET | RUSTOLEUM | cRUSTy\n");
    println!("Please enter a number 1 - 3");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: i32 = match choice.trim().parse() {
        Ok(strn) => strn,
        Err(_) => 4,
    };

    if choice == 1{
        println!("You chose {}, he is your protector!", poke1.Name);
    }

    else if choice == 2{
        println!("You chose {}, he is your protector!", poke2.Name);
        poke1 = poke2;
    }
    else if choice == 3 {
        println!("You chose {}, he is your protector!", poke3.Name);
        poke1 = poke3;
    }
    else{
        println!("Obviously you don't listen to directions, you get {} poke", punishment_poke.Name);
        poke1 = punishment_poke;
    }

    sleep(time);
    println!("You enter a maze filled with danger");
    println!("The only thing you have to protect you is your poke\n");
    sleep(time);
    let mut mz = maze::maze_gen();
    let mut tup;
    mz.reaction.1 = poke1.HP;
    let mut boss_life = 0;
    //println!("Your Pokemon has {} HP ",poke1.HP);
    let mut total_kills = 0;
    while boss_life == 0 {
        if poke1.HP < 0 {
            println!("Your pokemon has died, painfully!");
            break;
        }


        println!("What would you like to do?\n");
        println!("You may: Move Left(ML) |  Move Right(MR) | Move Up(MU) | Move Down(MD) | Show Stats(SS) | Quit(Q)\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: String = match guess.trim().parse() {
            Ok(strn) => strn,
            Err(_) => continue,
        };

        if guess.to_lowercase() == "mr" {
           // poke1 = tool::Battle(poke1);

            let chance  = rand::thread_rng().gen_range(1..=10);
            tup = maze::make_move(guess.to_lowercase(), mz);
            mz = tup.0;
            if tup.1 == 1 {
                continue;
            }
            else if tup.1 == 3 {
                poke1 = tool::Treasure_Battle(poke1);
                mz.reaction.1 = poke1.HP;
                maze::print_maze(&mz);
                continue;
            }

            else if tup.1 == 4 {
                if poke1.level > 3{
                    poke1 = tool::Boss_Battle(poke1);
                    mz.reaction.1 = poke1.HP;

                }
                else{
                    println!("Level too low, you will certainely die!");
                    continue;
                }
            }
            if tup.1 != 3 && chance == 1 || chance == 2 || chance == 7 || chance == 9{
                poke1 = tool::Battle(poke1);
                mz.reaction.1 = poke1.HP;
            }
            maze::print_maze(&mz);
            total_kills += 1;

        }
        else if guess.to_lowercase() == "ml"{
           // poke1 = tool::Battle(poke1); 

            let chance  = rand::thread_rng().gen_range(1..=10);
            tup = maze::make_move(guess.to_lowercase(), mz);
            mz = tup.0;
            if tup.1 == 1 {
                continue;
            }
            else if tup.1 == 3 {
                poke1 = tool::Treasure_Battle(poke1);
                mz.reaction.1 = poke1.HP;
                maze::print_maze(&mz);
                continue;
            }

            else if tup.1 == 4 {
                if poke1.level > 3{
                    poke1 = tool::Boss_Battle(poke1);
                    mz.reaction.1 = poke1.HP;
                    maze::print_maze(&mz);
                }
                else{
                    println!("Level too low, you will certainely die!");
                    continue;
                }
            }
            if tup.1 != 3 && chance == 1 || chance == 2 || chance == 7 || chance == 9{
                poke1 = tool::Battle(poke1);
                mz.reaction.1 = poke1.HP;
            }
            maze::print_maze(&mz);
            total_kills += 1;
        }
        else if guess.to_lowercase() == "mu"{
            let chance  = rand::thread_rng().gen_range(1..=10);
            tup = maze::make_move(guess.to_lowercase(), mz);
            mz = tup.0;
            if tup.1 == 1 {
                continue;
            }
            else if tup.1 == 3 {
                poke1 = tool::Treasure_Battle(poke1);
                mz.reaction.1 = poke1.HP;
                maze::print_maze(&mz);
                continue;

            }
            else if tup.1 == 4 {
                if poke1.level > 3{
                    poke1 = tool::Boss_Battle(poke1);
                    mz.reaction.1 = poke1.HP;
                    maze::print_maze(&mz);
                }
                else{
                    println!("Level too low, you will certainely die!");
                    maze::print_maze(&mz);
                    continue;
                }
            }
            if tup.1 != 3 && chance == 1 || chance == 2 || chance == 7 || chance == 9{
                poke1 = tool::Battle(poke1);
                mz.reaction.1 = poke1.HP;
            }
            maze::print_maze(&mz);
            total_kills += 1;
        }
        else if guess.to_lowercase()  == "md"{

            let chance  = rand::thread_rng().gen_range(1..=10);
            tup = maze::make_move(guess.to_lowercase(), mz);
            mz = tup.0;
            if tup.1 == 1 {
                continue;
            }
            else if tup.1 == 3 {
                poke1 = tool::Treasure_Battle(poke1);
                mz.reaction.1 = poke1.HP;
                maze::print_maze(&mz);
                continue;
            }

            else if tup.1 == 4 {
                if poke1.level > 3{
                    poke1 = tool::Boss_Battle(poke1);
                    mz.reaction.1 = poke1.HP;
                }
                else{
                    println!("Level too low, you will certainely die!");
                    maze::print_maze(&mz);
                    continue;
                }
            }
            if tup.1 != 3 && chance == 1 || chance == 2 || chance == 7 || chance == 9{
                poke1 = tool::Battle(poke1);
                mz.reaction.1 = poke1.HP;
            }
            maze::print_maze(&mz);
            total_kills += 1;
        }
        else if guess.to_lowercase() == "ss"{
            tool::show_stats(&poke1);
            println!("Your pokemon feels : {}\n", mz.reaction.0);
        }
        else if guess.to_lowercase() == "q"{
            break;
        }
        else{
            println!("Incorrect input, please choose: ML | MR | MU | MD | SS | Q");
            continue;
        }

    }
    if boss_life == 1{
        println!("You Escaped! Congrats!");
        tool::show_stats(&poke1);
    }
    else{
        println!("GAME OVER, RUST IS REMOVED");
    }
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          