fn main() {
    let mut str = String::new();
    str.push_str("foo");
    str.push_str("bar");
    println!("{str}");
    let inital: String = String::from("Initial content");
    println!("{inital}");

    str += &inital;
    println!("{str}");
    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let s = format!("{tic}-{tac}-{toe}");
    println!("{s}");

    let my_s = "happy holidays";
    let my_s_chars = my_s.chars();
    let mut my_s_vec: Vec<char> = Vec::new();
    for char in my_s_chars {
        my_s_vec.push(char);
    }
    println!("{:?}", my_s_vec)
}
