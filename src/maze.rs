
//extern crate ndarray;

//use ndarray::arr2;
use rand::Rng;
pub struct Mz<'a>{
       pub maze: [[&'a str; 20]; 14],
       pub pos: (i32,i32),
       pub reaction: (&'a str, i32),

}
pub fn make_move(direct: String,  mut maze : Mz)->(Mz,i32){
    let mut x = 0;
    if direct == "mu" && maze.pos.0-1 > 0 && maze.maze[(maze.pos.0 -1) as usize][maze.pos.1 as usize] != "-" && maze.maze[(maze.pos.0 -1) as usize][maze.pos.1 as usize] != "|"{
        //Still have to check for special posisitons 
        if maze.reaction.1 < 50 {
            maze.reaction.0 = "😟";
        }
        else if maze.reaction.1 > 100 {
            maze.reaction.0 = "😈";
        }
        else {
            maze.reaction.0 = "😁";
        }
        maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] = " ";
        maze.pos.0 -= 1;

        if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "T"{
            x = 3;
            println!("You found Treaseure!\n");
        }

        else if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "B"{
            x = 4;
            println!("You have reached the boss!\n");
            return (maze,x);
        }
        maze.maze[(maze.pos.0) as usize][maze.pos.1 as usize] = "X";
    }
    else if direct == "mr" && maze.pos.1 + 1 < 20 && maze.maze[(maze.pos.0) as usize][(maze.pos.1+1) as usize] != "-" && maze.maze[maze.pos.0 as usize][(maze.pos.1 + 1) as usize] != "|"{
        maze.maze[maze.pos.0 as usize][(maze.pos.1) as usize] = " ";

        if maze.reaction.1 < 50{
            maze.reaction.0 = "😟";
        }

        else if maze.reaction.1 > 100 {
            maze.reaction.0 = "😈";
        }
        else {
            maze.reaction.0 = "😁";
        }
        maze.pos.1 += 1;
        if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "T"{
            x = 3;
            println!("You found Treaseure!");
        }

        else if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "B"{
            x = 4;
            println!("You have reached the boss!\n");
            return (maze,x);
        }
        maze.maze[maze.pos.0 as usize][(maze.pos.1) as usize] = "X";
       // print_maze(maze);

    }
    else if direct == "md"&& maze.pos.0 + 1 < 14 && maze.maze[(maze.pos.0 + 1) as usize][maze.pos.1 as usize] != "-" && maze.maze[(maze.pos.0 + 1) as usize][maze.pos.1 as usize] != "|"{
        maze.maze[(maze.pos.0) as usize][maze.pos.1 as usize] = " ";

        if maze.reaction.1 < 50 {
            maze.reaction.0 = "😟";
        }

        else if maze.reaction.1 > 100 {
            maze.reaction.0 = "😈";
        }
        else {
            maze.reaction.0 = "😁";
        }
        maze.pos.0 += 1;

        if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "T"{
            x = 3;
            println!("You found Treaseure!");
        }

        else if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "B"{
            x = 4;
            println!("You have reached the boss!\n");

            return (maze,x);
        }
        maze.maze[(maze.pos.0) as usize][maze.pos.1 as usize] = "X";
    }
    else if direct == "ml"&& maze.pos.1-1 > 0 && maze.maze[maze.pos.0 as usize][(maze.pos.1-1) as usize] != "-" && maze.maze[maze.pos.0 as usize][(maze.pos.1) as usize] != "|"{
        maze.maze[(maze.pos.0) as usize][maze.pos.1 as usize] = " ";

        if maze.reaction.1 < 50 {
            maze.reaction.0 = "😟";
        }

        else if maze.reaction.1 > 100 {
            maze.reaction.0 = "😈";
        }
        else {
            maze.reaction.0 = "😁";
        }
        maze.pos.1 -= 1;
        if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "T"{
            x = 3;
            println!("You found Treaseure!");
        }
        else if maze.maze[maze.pos.0 as usize][maze.pos.1 as usize] == "B"{
            x = 4;
            println!("You have reached the boss!\n");
            return (maze,x);
        }
        maze.maze[(maze.pos.0) as usize][(maze.pos.1) as usize] = "X";
    }
    else{
        x = 1;
        println!("MOVE NOT POSSIBLE, PLEASE TRY AGAIN");
    }
    return (maze, x);
}

pub fn print_maze(maze : &Mz){
   for i in 0..maze.maze.len() {
        for j in 0..maze.maze[i].len() {
            print!("{}",maze.maze[i][j]);
        }
        println!("");
    }
} 

pub fn maze_gen() -> Mz<'static> {
    let mut maze2 = Mz{
        maze : [["|","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","|"],
               ["|"," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," "," ","|"],
               ["|"," ","|","-"," ","-","|"," ","|","-"," ","|","-"," ","|","|","|","|"," ","|"],
               ["|"," ","|","-"," ","-","-"," ","-","-"," ","|","|"," ","-","|","|","-"," ","|"],
               ["|"," ","|"," "," ","|"," "," ","|","|"," ","|","|","T"," ","|","|","T"," ","|"],
               ["|"," ","|"," ","-","|"," ","-","|","-"," ","|","|","-"," ","|","-","-","-","|"],
               ["|"," ","|","T","|","|"," ","|","|"," "," "," "," ","|"," ","-"," ","|","|","|"],
               ["|"," ","|","|"," "," "," ","|","|"," ","-","-"," ","-"," "," "," ","|","|","|"],
               ["|"," ","|","|","|"," ","-","-","|"," ","|","|"," "," "," ","-"," ","-","-","|"],
               ["|"," ","|","T","|"," ","|"," "," "," ","|","-","-","-","-","-"," "," "," ","|"],
               ["|"," ","|"," ","|"," ","|"," ","|"," ","-","-","-","-","-","-","-","-"," ","|"],
               ["|"," "," "," "," "," ","|"," ","|"," "," "," "," ","|","-","-","-","-"," ","|"],
               ["X"," ","|","-","-"," "," "," ","|","-","-","-"," "," "," "," "," "," ","B"," "],
               ["|","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","|"]],

               pos : (12,0),
               reaction : ("😁",50),
    };

    let mut maze3 = Mz{
        maze : [["|","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","|"],
               ["|","X","|"," "," "," "," "," "," "," "," "," "," "," "," "," "," ","|","T","|"],
               ["|"," ","|"," ","|","-","|"," ","|","-"," ","|","-"," ","|","|","|","|"," ","|"],
               ["|"," ","|"," ","|","-","-"," ","-","-"," ","|","|"," ","-","|","|","-"," ","|"],
               ["|"," "," "," ","|","|"," ","T","|","|"," ","|","|","T"," ","|"," "," "," ","|"],
               ["|"," ","|"," ","-","|"," ","-","|","-"," ","|","|","-"," ","|","-","-","-","|"],
               ["|"," ","|"," "," "," "," ","|","|"," "," "," "," ","|"," ","-"," ","|","|","|"],
               ["|"," ","|","|","|"," "," "," "," "," ","-","-"," ","-"," "," "," ","|","|","|"],
               ["|"," ","|","|","|"," ","-","-","|"," ","|","|"," "," "," ","-"," ","-","-","|"],
               ["|"," ","|","T"," "," ","|"," ","|"," ","|","-","-","-","-","-"," "," "," ","|"],
               ["|"," ","|"," ","|"," ","|"," ","|"," ","-","-","-","-","-","-","-","-"," ","|"],
               ["|"," ","|","-","-"," ","|"," "," "," "," "," "," ","|","-","-","-","-"," ","|"],
               [" "," "," "," "," "," "," "," ","|","-","-","-"," "," "," "," "," "," ","B"," "],
               ["|","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","-","|"]],

               pos : (1,1),
               reaction : ("😁",50),
    };

    let choice = rand::thread_rng().gen_range(1..=2);
    if choice == 2 {
        maze2 = maze3;
    }

    for i in 0..maze2.maze.len() {
        for j in 0..maze2.maze[i].len() {
            print!("{}",maze2.maze[i][j]);
        }
        println!("");
    }
    return maze2;
}