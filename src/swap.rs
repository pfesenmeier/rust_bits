fn swap(x: i32, y: i32) -> (i32, i32) {
    let x = x ^ y;
    let y = x ^ y;
    let x = x ^ y;
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_without_tmp() {
        let x_init = 0b101100111011101;
        let y_init = 0b010111000101011;

        let (x, y) = swap(x_init, y_init);

        assert_eq!(x_init, y);
        assert_eq!(y_init, x);
    }
}
