use reqwest::header::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let fact = get_cat_fact().await;
    let test_str: String = fact.unwrap();
    let mut score: i32 = 0;

    let scores: HashMap<char, i32> = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43), 
        ('R', 44), 
        ('S', 45),
        ('T', 46), 
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);
    //score = check_double(test_str, scores);
    score = check_sticker(test_str, scores);
    print!("{}", score);
    return;
}

async fn get_cat_fact() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get("https://adventofcode.com/2022/day/3/input")
        .header(COOKIE, "_ga=GA1.2.295906964.1670420247; session=53616c7465645f5f850bf02ea734ff236ad1666ec31f2dd21082c1c74ba714e0497fe0a9cdb893fbd869cc61f650bb1f018eafbec04c04c6cf41282fb6603940")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}

fn check_double(test_str: String, scores: HashMap<char, i32>) -> i32 {

    let mut word_len: i32 = 0;

    let mut a_map: HashMap<char, Vec<i32>> = HashMap::from([]);
    let mut b_map: Vec<char> = [].to_vec();
    let mut score: i32 = 0;


    for c in test_str.chars() {
        //print!("{}",c);
        if c == '\n' {
            let tmp: i32 = word_len / 2 - 1;
            for c in &b_map {
                if a_map[&c][0] <= tmp && a_map[&c][a_map[&c].len() - 1] > tmp {
                    //print!("{}\n", scores[&c]);
                    score += scores[&c];
                    break;
                }
            }

            //testing
            //for val in b_map{
            //    print!("{}\n", val);
            //}
            //break;


            a_map = HashMap::from([]);
            b_map = [].to_vec();
            word_len = 0;
        }else {
            if a_map.contains_key(&c) {
                //print!("{}", a_map.get(&c).unwrap()[0]);
                let mut tmp: Vec<i32> = a_map.get(&c).unwrap().to_vec();
                //print!("{}", tmp[0]);
                tmp.push(word_len);
                a_map.insert(c, tmp);
                if b_map.contains(&c) == false {
                    b_map.push(c);
                }
            } else {
                let mut vec: Vec<i32> = Vec::new();
                vec.push(word_len);
                a_map.insert(c, vec);
            }
            word_len += 1;
        }
    }

    return score;

}

fn check_sticker(test_str: String, scores: HashMap<char, i32>) -> i32 {

    let mut counter: i32 = 0;

    let mut a_map: HashMap<char, i32> = HashMap::from([]);
    let mut score: i32 = 0;
    let mut flag: bool = false;


    for c in test_str.chars() {
        //print!("{}",c);
        if c == '\n' {
            if counter == 0 {
                counter += 1;
            }else if counter == 1 {
                counter += 1;
            }else {
                a_map = HashMap::from([]);
                counter = 0;
                flag = false;
            }

        }else {
            if a_map.contains_key(&c) {
                if counter == 1 {

                    a_map.insert(c, 1);

                } else if counter == 2 {
                    if a_map[&c] == 1 {
                        if flag == false {
                            score += scores[&c];
                        }
                        flag = true;
                    }
                }


            } else {
                if counter == 0 {
                    a_map.insert(c, 0);
                }
            }
        }
    }

    return score;


}


