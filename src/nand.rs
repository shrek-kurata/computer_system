/// 0: false
/// 1: true
/// 基礎の関数
/// これを使用して他の論理関数を作成する
pub fn nand(a: bool, b: bool) -> bool {
    !(a && b)
}

pub fn not(input: bool) -> bool {
    nand(input, input)
}

pub fn and(a: bool, b: bool) -> bool {
    not(nand(a,b))
}

pub fn or(a: bool, b: bool) -> bool {
    nand(not(a), not(b))
}

pub fn xor(a: bool, b: bool) -> bool {
    let temp = nand(a,b);
    nand(nand(a, temp), nand(b, temp))
}

pub fn mux(a: bool, b: bool, sel: bool) -> bool {
    if sel == false {
        a
    } else {
        b
    }
}

pub fn dmux(input: bool, sel: bool) -> (bool, bool) {
    if sel == false {
        (input, false)
    } else {
        (false, input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_nand() {
        assert_eq!(nand(true, true), false);
        assert_eq!(nand(true, false), true);
        assert_eq!(nand(false, true), true);
        assert_eq!(nand(false, false), true);
    }

    #[test]
    fn test_and() {
        assert_eq!(and(true, true), true);
        assert_eq!(and(true, false), false);
        assert_eq!(and(false, true), false);
        assert_eq!(and(false, false), false);
    }

    #[test]
    fn test_or() {
        assert_eq!(or(true, true), true);
        assert_eq!(or(true, false), true);
        assert_eq!(or(false , true), true);
        assert_eq!(or(false, false), false);
    }

    #[test]
    fn test_not() {
        assert_eq!(not(false), true);
        assert_eq!(not(true), false);
    }

    #[test]
    fn test_xor() {
        assert_eq!(xor(true, true), false);
        assert_eq!(xor(true, false), true);
        assert_eq!(xor(false, false), false);
        assert_eq!(xor(false, true), true);
    }

    #[test]
    fn test_mux() {
        assert_eq!(mux(true, false, true), false)
    }

    #[test]
    fn test_dmux() {
        assert_eq!(dmux(true, false), (true, false))
    }
}
