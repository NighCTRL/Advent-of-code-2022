use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {

    let mut points = 0;
    // win conditions
    let win_cond = HashMap::from([("s", "p"), ("p", "r"), ("r", "s")]);
    // Translate ABC into (r)ock (p)aper (s)cissors
    let adv = HashMap::from([("A", "r"), ("B", "p"), ("C", "s")]);
    // Same for XYZ
    let me = HashMap::from([("X", "r"), ("Y", "p"), ("Z", "s")]);

    let move_points = HashMap::from([("r", 1), ("p", 2), ("s", 3)]);
    let games = include_str!("../inputs/day_2.input").lines();

    for game in games {
        let moves = game.split_whitespace().map(|v| v.trim()).collect::<Vec<&str>>();
        let adv_move = adv.get(moves[0]).unwrap();
        let my_move = me.get(moves[1]).unwrap();

        if adv_move == my_move {
            points += 3;
        } else if win_cond.get(my_move).unwrap() == adv_move {
            points += 6;
        }

        points += move_points.get(my_move).unwrap();
    };
    println!("Score is {}", points);
    return Ok(());
// println!("this is The score {:?}", score);
}
