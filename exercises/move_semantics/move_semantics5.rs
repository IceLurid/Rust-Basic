// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.



fn main() {
    let mut x = 100;
    let y = &mut x; // 创建第一个可变引用
    *y += 100;      // 使用第一个可变引用修改 x

    // 在创建第二个可变引用之前，第一个可变引用已经失效
    let z = &mut x; // 创建第二个可变引用
    *z += 1000;     // 使用第二个可变引用修改 x

    assert_eq!(x, 1200); // 检查 x 的值是否为 1200
}