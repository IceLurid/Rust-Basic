// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.



fn main() {
    // 不再需要在这里创建 vec0
    // let vec0 = Vec::new();

    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// `fill_vec()` 不再接受 `vec: Vec<i32>` 作为参数
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new(); // 在函数内部创建一个新的 Vec<i32>

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}