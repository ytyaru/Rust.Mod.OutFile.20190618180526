pub mod my_mod_3; // 子モジュールを公開として宣言する（lib.rsで`my_mod_1::my_mod_2`として参照できる） 

pub fn public() -> String { String::from("my_mod_1::my_mod_2::public()") }
fn private() -> String { String::from("my_mod_1::my_mod_2::private()") }

#[cfg(test)]
mod tests {
//    use super; // error[E0432]: unresolved import `super`
//    use super::*;
    #[test]
    fn test_public() {
        assert_eq!(super::public(), String::from("my_mod_1::my_mod_2::public()"));
    }
    #[test]
    fn test_private() {
        assert_eq!(super::private(), String::from("my_mod_1::my_mod_2::private()"));
    }
}
