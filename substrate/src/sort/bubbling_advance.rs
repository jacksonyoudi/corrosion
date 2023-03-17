pub fn bubbling_advance<T>(nums: &mut Vec<T>)
    where T: PartialOrd + Copy,
{
    let l: usize = nums.len();

    for i in 0..l {
        for j in 0..l - i - 1 {
            if nums[j] > nums[j + 1] {
                let t: T = nums[j];
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
    fn bubbling_advance_test() {
        let mut vec1: Vec<i64> = vec![-9, 3, 4, 2, 1];
        bubbling_advance(&mut vec1);
        println!("result: {:?}", vec1);
        assert_eq!(vec1[0], -9);
        assert_eq!(vec1[1], 1);
        assert_eq!(vec1[2], 2);


        let mut vec1 = vec!["v", "a", "z", "m", "a"];
        bubbling_advance(&mut vec1);
        println!("result: {:?}", vec1);
        assert_eq!(vec1[0], "a");
        assert_eq!(vec1[1], "a");
        assert_eq!(vec1[2], "m");
    }
}