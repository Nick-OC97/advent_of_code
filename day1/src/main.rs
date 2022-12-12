use reqwest::header::*;


#[tokio::main]
async fn main() {
    let fact = get_cat_fact().await;

    let test_str: String = fact.unwrap();

//    test_1_with_arr(test_str);
//    test_1_faster(test_str);
        test_1b_fast(test_str);

}

async fn get_cat_fact() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get("https://adventofcode.com/2022/day/1/input")
        .header(COOKIE, "_ga=GA1.2.295906964.1670420247; session=53616c7465645f5f850bf02ea734ff236ad1666ec31f2dd21082c1c74ba714e0497fe0a9cdb893fbd869cc61f650bb1f018eafbec04c04c6cf41282fb6603940")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}

fn test_1_with_arr(fact: String) {

    let mut largest: i32 = 0;
    let mut current: i32 = 0;
    
    let arr: Vec<String> = fact.split("\n\n").map(|s| s.to_string()).collect();
    
    for i in &arr {

        let tmp: Vec<i32> = i.split("\n").map(|s| s.trim()).filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();

        for i in &tmp {
            current += i;
        }
        
        if current > largest {
            largest = current;
        }

        current = 0;

    }
    print!("{}", largest);
 
}

fn test_1_faster(test_str: String) {
    let mut largest: i32 = 0;
    let mut current: i32 = 0;
  
    let mut num: String = String::from("");
    let mut num_int: i32 = 0;

    for (i,c) in test_str.chars().enumerate() {
        //print!("{}", c);
        let j = i + 1;
        if j == test_str.len() - 1 {
            num.push(c);
            num_int = num.parse().unwrap();
            current += num_int;
            if current > largest {
                largest = current;
            }
            break;

        } else if c == '\n' {
            if num != "" {
                num_int = num.parse().unwrap();
                current += num_int;
            }else {
                current = 0;
            }
            if current > largest {
                largest = current;
            }
            num_int = 0;
            num = String::from("");
        } else {
            num.push(c);
        }
    }

    print!("{}", largest);

}

fn test_1b_fast(test_str: String) {
    let mut largest: i32 = 0;
    let mut current: i32 = 0;
    let mut largest_arr: [i32;3] = [0;3];
  
    let mut num: String = String::from("");
    let mut num_int: i32 = 0;

    for (i,c) in test_str.chars().enumerate() {
        //print!("{}", c);
        let j = i + 1;
        if j == test_str.len() - 1 {
            num.push(c);
            num_int = num.parse().unwrap();
            current += num_int;
            if current > largest_arr[0] {
                re_arrange(current, &mut largest_arr);
            }
            break;

        } else if c == '\n' {
            if num != "" {
                num_int = num.parse().unwrap();
                current += num_int;
            }else {
                current = 0;
            }
            if current > largest_arr[0] {
                re_arrange(current, &mut largest_arr);
            }
            num_int = 0;
            num = String::from("");
        } else {
            num.push(c);
        }
    }

    for i in largest_arr {
        print!("{}\n", i);
        largest += i;
    }

    print!("{}", largest);

}

fn re_arrange(current: i32, largest_arr: &mut [i32]) {
    //largest 3
    let mut i: i32 = 0;
    let mut tmp: i32;
    let mut tmp2: i32;
    
    tmp2 = current;

    while i !=3 && current > largest_arr[i as usize] {
        i += 1;
    }

    i = i - 1;

    while i >= 0 {
        tmp = largest_arr[i as usize];
        largest_arr[i as usize] = tmp2;
        tmp2 = tmp;
        i -= 1;
    }
}




