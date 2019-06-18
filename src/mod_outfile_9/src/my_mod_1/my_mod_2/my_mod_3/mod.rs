// mod my_mod_4; // 子モジュールを宣言しない。テストコードも実行されない

pub fn public() -> String {
//    my_mod_4::public(); // error[E0433]: failed to resolve: use of undeclared type or module `my_mod_4`
    String::from("my_mod_1::my_mod_2::my_mod_3::public()")
}
fn private() -> String {
//    my_mod_4::private(); // error[E0433]: failed to resolve: use of undeclared type or module `my_mod_4`
    String::from("my_mod_1::my_mod_2::my_mod_3::private()")
}

#[cfg(test)]
mod tests {
//    use super; // error[E0432]: unresolved import `super`
//    use super::*;
    #[test]
    fn test_public() {
        assert_eq!(super::public(), String::from("my_mod_1::my_mod_2::my_mod_3::public()"));
    }
    #[test]
    fn test_private() {
        assert_eq!(super::private(), String::from("my_mod_1::my_mod_2::my_mod_3::private()"));
    }
}
