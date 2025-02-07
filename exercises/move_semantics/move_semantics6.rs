// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 传递引用，而不是所有权

    string_uppercase(data); // 传递所有权
}

// Should not take ownership
fn get_char(data: &String) -> char { // 修改参数为引用
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) { // 修改参数为所有权
    let uppercased = data.to_uppercase(); // 创建一个新的字符串

    println!("{}", uppercased);
}