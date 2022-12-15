use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    
    let mut points = 0;

    let map_result = HashMap::from([("X", "l"), ("Y", "d"), ("Z", "w")]);
    let map_adv = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);
    let map_win = HashMap::from([("s", "r"), ("p", "s"), ("r", "p")]);
    let map_lose = HashMap::from([("s", "p"), ("p", "r"), ("r", "s")]);
    let map_points = HashMap::from([("r", 1), ("p", 2), ("s", 3)]);

    let games = include_str!("./day_2.input").lines();

    for game in games {
        let sides = game.split_whitespace().map(|v| v.trim()).collect::<Vec<&str>>();
        let adv_move = map_adv.get(sides[0]).unwrap();
        let expected_outcome = map_result.get(sides[1]).unwrap();
        match &expected_outcome[..] {
            "w" => {
                points += 6;
                let my_move = map_win.get(adv_move).unwrap();
                points += map_points.get(my_move).unwrap();
                println!("Game is {}, i should WIN, my move is {} against {}", game, my_move, adv_move);
            }
            "d" => { 
                points += 3;
                points += map_points.get(adv_move).unwrap();
                println!("Game is {}, should be a draw, my move is same as adv {}", game, adv_move);
            }
            "l" => {
                let my_move = map_lose.get(adv_move).unwrap();
                points += map_points.get(my_move).unwrap();
                println!("Game is {}, i should lose, my move is {} against {}", game, my_move, adv_move);
            }
            _ => println!("Outcome not recognized")
        }
    }

    println!("points: {}", points);
    Ok(())
}
