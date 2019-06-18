mod my_mod_1; // my_mod_1/mod.rs を非公開として宣言する。

fn a() {
    my_mod_1::public();
//    my_mod_1::private(); // error[E0603]: function `private` is private

    my_mod_1::my_mod_2::public();
//    my_mod_1::my_mod_2::my_mod_3::public(); // error[E0603]: module `my_mod_3` is private
}

#[cfg(test)]
mod tests {
    use super::my_mod_1;
    #[test]
    fn test_my_mod_1_public() {
        assert_eq!(my_mod_1::public(), String::from("my_mod_1::public()"));
    }
    #[test]
    fn test_my_mod_2_public() {
        assert_eq!(my_mod_1::my_mod_2::public(), String::from("my_mod_1::my_mod_2::public()"));
    }
    #[test]
    fn test_my_mod_3_public() {
//        assert_eq!(my_mod_1::my_mod_2::my_mod_3::public(), String::from("my_mod_1::my_mod_2::my_mod_3::public()")); // error[E0603]: module `my_mod_3` is private
    }
}
