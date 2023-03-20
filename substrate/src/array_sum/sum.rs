use std::path::is_separator;

/** 在 Rust 中，`u32` 类型提供了 `checked_add` 方法，
 * 用于对两个 `u32` 类型的值进行加法操作，并返回一个 `Option<u32>` 类型的结果。
* 如果加法操作没有溢出，则返回 `Some(result)`，其中 `result` 是加法操作的结果；如果加法操作溢出，则返回 `None`。
 */


fn sum_u32(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for &num in numbers {
        match sum.checked_add(num) {
            Some(result) => sum = result,
            None => return None,
        }
    }
    Some(sum)
}


fn sum_u32_backup(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0u32;
    let mut is_overflow: bool = false;
    for &num in numbers {
        (sum, is_overflow) = sum.overflowing_add(num);
        if is_overflow {
            return None;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn sum_test() {
        let numbers: [u32; 5] = [1, 2, 3, 4, 5];
        match sum_u32_backup(&numbers) {
            Some(result) => println!("Sum of numbers: {}", result),
            None => println!("Sum of numbers overflowed!"),
        }

        let numbers: [u32; 5] = [1, 2, 3, 4, u32::MAX];
        match sum_u32_backup(&numbers) {
            Some(result) => println!("Sum of numbers: {}", result),
            None => println!("Sum of numbers overflowed!"),
        }
    }
}