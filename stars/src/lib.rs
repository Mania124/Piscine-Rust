pub fn stars(n: u32) -> String {
    let base: u32 = 2;
    let mut f = 0;
    let mut l = "".to_string();
    while f != base.pow(n) {
        l += "*";
        f += 1
    }
    return l;

}
