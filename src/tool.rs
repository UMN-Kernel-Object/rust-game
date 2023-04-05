use rand::Rng;
use std::io;
use std::time::Duration;
use std::thread::sleep;

pub struct Poke{
    pub Name: String,
    pub HP: i32,
    pub Attack: (String, i32),
    pub potions: i32,
    pub level: i32,
    pub xp: i32,
}

pub fn Battle(mut You : Poke) -> Poke{
    println!("YOU HAVE ENCOUNTERED AN ENEMY!");
    let mut enem = Gen_Enem(You.level);
    while enem.HP > 0 {
    println!("What will you do: Attack | Run | Heal \n");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: String = match guess.trim().parse() {
        Ok(strn) => strn,
        Err(_) => continue,
    };
        if guess.to_lowercase() == "run"{
            println!("You ran");
            return You;
        }
        else if guess.to_lowercase() == "attack"{

            println!("\nYou use {}!\n", You.Attack.0);
            enem.HP = enem.HP - You.Attack.1;
            if enem.HP <= 0{
                let t = Duration::from_secs(1);
                sleep(t);
                println!("You slaughtered the enemy 💀\n");
                let earn_p  = rand::thread_rng().gen_range(1..=10);
                if earn_p == 2 || earn_p == 3 {
                    println!("You earned a potion!");
                    You.potions = You.potions + 1;
                }
                println!("You gained 25 xp!");
                You.xp += 25;

                if You.xp as f32 >= 50 as f32 * (You.level as f32  * 0.65) {
                    println!("Your poke leveled up!\n");
                    You.level += 1;
                    println!("Your poke is now level {}\n", You.level);
                    You.xp = 0;
                    You.Attack.1 += 10;
                    You.potions += 1;
                    You.HP = ((You.level as f32 * 0.5) * 50.0) as i32 + You.HP;
                    println!("Damage and health boost and +1 potions!\n");

                }

                return You;
            }
            println!("{}'s HP is {}! 🤕", enem.Name, enem.HP);
            let time = Duration::from_secs(1);
            sleep(time);

            println!("The enemy attacks using {} and deals {} damage! 🤕", enem.Attack.0, enem.Attack.1);
            You.HP = You.HP - enem.Attack.1;
            if You.HP <= 0 {
                return You;
            }
            let heal = rand::thread_rng().gen_range(1..=20);

            if heal == 5{
                sleep(time);
                println!("The enemy has healed, gaining 10 HP");
                enem.HP = enem.HP + 10;
            }
            println!("Your Poke is at {} HP", You.HP);


        }
        else if guess.to_lowercase() == "heal"{
            if You.potions > 0{
                println!("You chose to heal!");
                You.potions = You.potions -1;
                You.HP = You.HP + 25;
                println!("Your HP is {} and you have {} potions left", You.HP, You.potions);
            }
            else{
                println!("You have no more potions!🤡");
            }
        }
        else{
            println!("Invalid input!🤡 Please choose between Attack | Run | Heal");
            continue;
        }
    }
    return You
}


pub fn Gen_Enem(difficulty : i32) -> Poke{
    let health = (rand::thread_rng().gen_range(25..=50) as f32 * (difficulty as f32* 0.55)).ceil() as i32;
    let dmg = (rand::thread_rng().gen_range(10..=20) as f32 * (difficulty as f32 * 0.55)).ceil() as i32;
    let attack = rand::thread_rng().gen_range(0..10);
    let Attacks : [&str ; 10] = ["kick", "punch", "puke", "round house", "splash", "RustAway", "shoot", "grab", "squirt", "throw"];

    let enemy = Poke {
        Name: String::from("RUST_REMOVER"),
        HP: health,
        Attack: (String::from(Attacks[attack]), dmg),
        potions: 0,
        level : 0,
        xp : 0,
    };

    return enemy;

}

pub fn show_stats( P : &Poke) {
    println!("Your Poke's stats\nHP: {}\nPotions: {}\nMove: {}\nDamage {}\nXP: {}\nLevel: {} ", P.HP, P.potions, P.Attack.0, P.Attack.1, P.xp, P.level );
} 

pub fn Treasure_Battle(mut You : Poke) -> Poke{

     println!("You've gained 25 XP and 1 potion!");
     let time = Duration::from_secs(1);
     You.xp += 25;
     You.potions += 1;
     sleep(time);
     println!("There's an enemey gaurding the exit!\n");

    println!("YOU FOUND MY TREASURE! TIME TO DIE\n");
    let mut enem = Gen_Enem(You.level);
    enem.HP += 10;
    enem.Attack.1 += 6;
    while enem.HP > 0 {
    println!("What will you do: Attack | Heal \n");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: String = match guess.trim().parse() {
        Ok(strn) => strn,
        Err(_) => continue,
    };
        if guess.to_lowercase() == "attack"{

            println!("\nYou use {}!\n", You.Attack.0);
            enem.HP = enem.HP - You.Attack.1;
            if enem.HP <= 0{
                let t = Duration::from_secs(1);
                sleep(t);
                println!("You slaughtered the enemy 💀\n");
                let earn_p  = rand::thread_rng().gen_range(1..=10);
                if earn_p == 2 || earn_p == 3 {
                    println!("You earned a potion!");
                    You.potions = You.potions + 1;
                }
                println!("You gained 25 xp!");
                You.xp += 25;

                if You.xp as f32 >= 50 as f32 * (You.level as f32  * 0.65) {
                    println!("Your poke leveled up!\n");
                    You.level += 1;
                    println!("Your poke is now level {}\n", You.level);
                    You.xp = 0;
                    You.Attack.1 += 10;
                    You.potions += 1;
                    You.HP = ((You.level as f32 * 0.5) * 50.0) as i32 + You.HP;
                    println!("Damage and health boost and +1 potions!\n");

                }

                return You;
            }
            println!("{}'s HP is {}! 🤕", enem.Name, enem.HP);
            let time = Duration::from_secs(1);
            sleep(time);
            You.HP = You.HP - enem.Attack.1;
            println!("The enemy attacks using {} and deals {} damage! 🤕", enem.Attack.0, enem.Attack.1);
            if You.HP <= 0 {
                return You;
            }
            let heal = rand::thread_rng().gen_range(1..=10);

            if heal == 5 || heal == 3{
                sleep(time);
                println!("The enemy has healed, gaining 10 HP 💪");
                enem.HP = enem.HP + 10;
            }
            println!("Your Poke is at {} HP", You.HP);
        }
        else if guess.to_lowercase() == "heal"{
            if You.potions > 0{
                println!("You chose to heal!");
                You.potions = You.potions -1;
                You.HP = You.HP + 25;
                println!("Your HP is {} and you have {} potions left", You.HP, You.potions);
            }
            else{
                println!("You have no more potions! 🤡");
            }
        }
        else{
            println!("Invalid input!🤡 Please choose between Attack | Heal");
            continue;
        }
    }
    return You
}

pub fn Boss_Battle(mut You : Poke) -> Poke{

    println!("PREPARE TO DIE!\n");
    let mut enem = Gen_Enem(You.level);
    enem.HP += 100;
    enem.Attack.1 += 20;

    while enem.HP > 0 {
    println!("What will you do: Attack | Heal \n");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: String = match guess.trim().parse() {
        Ok(strn) => strn,
        Err(_) => continue,
    };
        if guess.to_lowercase() == "attack"{

            println!("\nYou use {}!\n", You.Attack.0);
            enem.HP = enem.HP - You.Attack.1;
            if enem.HP <= 0{
                let t = Duration::from_secs(1);
                sleep(t);
                println!("You slaughtered the enemy\n");
                let earn_p  = rand::thread_rng().gen_range(1..=10);
                if earn_p == 2 || earn_p == 3 {
                    println!("You earned a potion!");
                    You.potions = You.potions + 1;
                }
                println!("You gained 25 xp!");
                You.xp += 25;

                if You.xp as f32 >= 50 as f32 * (You.level as f32  * 0.65) {
                    println!("Your poke leveled up!\n");
                    You.level += 1;
                    println!("Your poke is now level {}\n", You.level);
                    You.xp = 0;
                    You.Attack.1 += 10;
                    You.potions += 1;
                    You.HP = ((You.level as f32 * 0.5) * 50.0) as i32 + You.HP;
                    println!("Damage and health boost and +1 potions!\n");

                }

                return You;
            }
            println!("{}'s HP is {}!", enem.Name, enem.HP);
            let time = Duration::from_secs(1);
            sleep(time);
            You.HP = You.HP - enem.Attack.1;
            println!("The enemy attacks using {} and deals {} damage!", enem.Attack.0, enem.Attack.1);
            if You.HP <= 0 {
                println!("Oh no...");
                return You;
            }
            let heal = rand::thread_rng().gen_range(1..=10);

            if heal == 5 || heal == 3 || heal == 6{
                sleep(time);
                println!("The enemy has healed, gaining 10 HP");
                enem.HP = enem.HP + 10;
            }
            println!("Your Poke is at {} HP", You.HP);
        }
        else if guess.to_lowercase() == "heal"{
            if You.potions > 0{
                println!("You chose to heal!");
                You.potions = You.potions -1;
                You.HP = You.HP + 25;
                println!("Your HP is {} and you have {} potions left", You.HP, You.potions);
            }
            else{
                println!("You have no more potions!");
            }
        }
        else{
            println!("Invalid input! Please choose between Attack | Heal");
            continue;
        }
    }
    return You
}
