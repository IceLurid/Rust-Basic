// traits4.rs
//
// Your task is to replace the '??' sections so the code compiles.
//
// Don't change any line other than the marked one.
//
// Execute `rustlings hint traits4` or use the `hint` watch subcommand for a
// hint.


pub trait Licensed {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware {}

struct OtherSoftware {}

impl Licensed for SomeSoftware {
    fn licensing_info(&self) -> String {
        "SomeSoftware license".to_string()
    }
}

impl Licensed for OtherSoftware {
    fn licensing_info(&self) -> String {
        "OtherSoftware license".to_string()
    }
}

// 使用泛型和 trait bound 来定义函数参数的类型
fn compare_license_types<T: Licensed>(software: T, software_two: T) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware {};
        let other_software = SomeSoftware {}; // 确保类型一致

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = OtherSoftware {};
        let other_software = OtherSoftware {}; // 确保类型一致

        assert!(compare_license_types(some_software, other_software));
    }
}