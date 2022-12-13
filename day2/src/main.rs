use reqwest::header::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let fact = get_cat_fact().await;
    let mut points: i32 = 0;
    let test_str: String = fact.unwrap();
    let mut mine: char = ' ';
    let mut opponent: char = ' ';

    //print!("{}", test_str);

    for c in test_str.chars() {
        //print!("{}", c);
        if c == 'A' || c == 'B' || c == 'C' {
            opponent = c;
        } else if c == 'X' || c == 'Y' || c == 'Z' {
            mine = c;
        } else if c == '\n' {
            points += get_points2(mine, opponent);
        }
    }

    print!("{}", points);

}

async fn get_cat_fact() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get("https://adventofcode.com/2022/day/2/input")
        .header(COOKIE, "_ga=GA1.2.295906964.1670420247; session=53616c7465645f5f850bf02ea734ff236ad1666ec31f2dd21082c1c74ba714e0497fe0a9cdb893fbd869cc61f650bb1f018eafbec04c04c6cf41282fb6603940")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}

fn get_points(mine: char, opponent: char) -> i32 {
    let a_map = HashMap::from([
        ('X', 4),
        ('Y', 8),
        ('Z', 3),
    ]);

    let b_map = HashMap::from([
        ('X', 1),
        ('Y', 5),
        ('Z', 9),
    ]);

    let c_map = HashMap::from([
        ('X', 7),
        ('Y', 2),
        ('Z', 6),
    ]);

    let master = HashMap::from([
        ('A', a_map),
        ('B', b_map),
        ('C', c_map),
    ]);

    return master[&opponent][&mine];
}


fn get_points2(mine: char, opponent: char) -> i32 {
    let p_map = HashMap::from([
        ('X', 0),
        ('Y', 3),
        ('Z', 6),
    ]);

    let lose_map = HashMap::from([
        ('A', 3),
        ('B', 1),
        ('C', 2),
    ]);

    let draw_map = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
    ]);

    let win_map = HashMap::from([
        ('A', 2),
        ('B', 3),
        ('C', 1),
    ]);

    if p_map[&mine] == 0 {
        return p_map[&mine] + lose_map[&opponent];
    } else if p_map[&mine] == 3 {
        return p_map[&mine] + draw_map[&opponent];
    } else {
        return p_map[&mine] + win_map[&opponent];
    }

}











