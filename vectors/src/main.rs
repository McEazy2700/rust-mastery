fn main() {
    // let my_vec: Vec<i32> = Vec::new();
    let mut v = Vec::new();
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("Vec {:?}", v);
    let third = &v[2];
    println!("Third {}", third);
    let third = v.get(2);
    if let Some(num) = third {
        println!("The value was found {}", num)
    }
    let first = &mut v[0];
    println!("first {first}");
    v[0] = 23;
    println!("first {}", v[0]);
    for item in &mut v {
        *item += 50;
        println!("{item}")
    }
}
