pub fn verse0() -> String {
    "No more bottles of beer on the wall, no more bottles of beer.\n\
    Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
}

pub fn get_bottles(n: u32) -> String {
   match n {
       2..=99 => format!("{} bottles", n),
       1 => format!("1 bottle"),
       _ => format!("no more bottles")
   } 
}

pub fn take(n: u32) -> String {
    match n {
        1 => "it".to_string(),
        _ => "one".to_string(),
    }
}

pub fn verse(n: u32) -> String {
    let bottles = get_bottles(n);
    match n {
        0 => verse0(),
        _ => format!("{} of beer on the wall, {} of beer.\n\
                Take {} down and pass it around, {} of beer on the wall.\n",
                bottles, bottles, take(n), get_bottles(n-1))
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = "".to_string();
    for i in end..start+1 {
        if i == end {
            song = format!("{}{}",verse(i), song)
        } else {
            song = format!("{}\n{}",verse(i), song)
        }
    } 
    song.to_string()
}
