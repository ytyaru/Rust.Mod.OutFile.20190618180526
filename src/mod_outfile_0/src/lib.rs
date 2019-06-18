mod my_mod_1; // my_mod_1/mod.rs を非公開として宣言する。

fn a() {
    my_mod_1::public();
//    my_mod_1::private(); // error[E0603]: function `private` is private
}

/*
#[cfg(test)]
mod tests {
    use super::my_mod_1;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
