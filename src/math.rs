pub fn div_rem<T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy>(
    left: T,
    right: T,
) -> (T, T) {
    let quotient = left / right;
    let remainder = left % right;
    (quotient, remainder)
}
