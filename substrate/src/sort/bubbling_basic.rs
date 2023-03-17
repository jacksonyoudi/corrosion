pub fn bubbling(nums: &mut Vec<i64>) {
    let l: usize = nums.len();

    for i in 0..l {
        for j in 0..l - i - 1 {
            if nums[j] > nums[j + 1] {
                let t: i64 = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = t;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubbling_test() {
        let mut vec1: Vec<i64> = vec![-9, 3, 4, 2, 1];
        bubbling(&mut vec1);
        println!("result: {:?}", vec1);
        assert_eq!(vec1[0], -9);
        assert_eq!(vec1[1], 1);
        assert_eq!(vec1[2], 2);
    }
}