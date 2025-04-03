pub fn delete_and_backspace(s: &mut String) {
    let s_copy = s.clone();
    s.clear();

    let mut skip_next = 0;
    for v in s_copy.chars() {
        if v == '-' {
            s.pop();
        } else if v == '+' {
            skip_next += 1;
        } else if skip_next > 0 {
            skip_next -= 1;
        } else {
            s.push(v);
        }
    }

}

pub fn do_operations(v: &mut [String]) {
    v.iter_mut().for_each(|equation| {
        let (l, r) = equation.split_once(['+', '-']).unwrap();
        let (l, r) = (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap());

        let result = if equation.contains('+') { l + r } else { l - r };
        *equation = result.to_string();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_backspace() {
        let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
        let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
        let mut a_3 = String::from("pad-rtic+eulqw--+rar");
        let mut a_4 = String::from("--++++");

        delete_and_backspace(&mut a_1);
        delete_and_backspace(&mut a_2);
        delete_and_backspace(&mut a_3);
        delete_and_backspace(&mut a_4);

        assert_eq!(a_1, "borrow");
        assert_eq!(a_2, "help");
        assert_eq!(a_3, "particular");
        assert_eq!(a_4, "");
    }

    #[test]
    fn test_do_operations() {
        let mut b_1 = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "0+0".to_owned(),
            "0-0".to_owned(),
            "10-100".to_owned(),
        ];
        do_operations(&mut b_1);

        assert_eq!(b_1, ["4", "5", "7", "0", "0", "-90"]);
    }
}
