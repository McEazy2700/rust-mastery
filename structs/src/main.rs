/// A regular struct
struct User {
    first_name: String,
    last_name: String,
    email: String,
    is_active: bool,
    password_hash: String,
    color: Option<Color>
}

/// A tupple struct
struct Color(i32, i32, i32);

/// A unit-like struct
// struct HappyPerson;

fn build_user(first_name: String, last_name: String, email: String, is_active: bool, paswd_hash: String) -> User {
    User {
        email,
        is_active,
        last_name,
        first_name,
        password_hash: paswd_hash,
        color: Option::Some(Color(0, 0, 0))
    }
}

fn main() {
    let anya = build_user(
        "Anya".to_string(),
        "Lare".to_string(),
        "anyalare@gmail.com".to_string(),
        true, "sdfadfsuiqojwenr39023".to_string()
    );
    println!(
        "User(first_name={}, lastname={}, email={}, is_active={}, is_authenticated={})",
        anya.first_name, anya.last_name, anya.email, anya.is_active, anya.password_hash == "sdfadfsuiqojwenr39023"
    );
    let anya_sis = User{first_name: String::from("Yuri"), ..anya};
    let col = anya_sis.color.unwrap();
    let col_str = format!("({}, {}, {})", col.0, col.1, col.2);
    println!("{} {} {}", anya_sis.first_name, anya_sis.last_name, col_str)
}
