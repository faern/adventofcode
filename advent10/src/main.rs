fn main() {
    let input = "1113222113";
    let mut output = input.to_string();
    for i in 1..51 {
        output = look_say(&output[..]);
        if [40, 50].contains(&i) {
            println!("Playing look-and-say {} times gives a number {} digits long", i, output.len());
        }
    }
}

fn look_say(input: &str) -> String {
    let mut out = String::new();
    let mut last_opt_d = None;
    let mut count_d = 0;
    for d in input.chars() {
        if let Some(last_d) = last_opt_d {
            if d != last_d {
                out.push_str(&format!("{}{}", count_d, last_d)[..]);
                count_d = 0;
            }
        }
        last_opt_d = Some(d);
        count_d += 1;
    }
    out.push_str(&format!("{}{}", count_d, last_opt_d.unwrap())[..]);
    out
}
