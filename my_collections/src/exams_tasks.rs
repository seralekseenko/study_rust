/// Compute the [median](https://en.wikipedia.org/wiki/Median) and the
/// [mode](https://en.wikipedia.org/wiki/Mode_(statistics)) from a vector of numbers.
///
/// # Arguments
///
/// * nums - A vector of numbers (sorted/unsorted) for which you want to calculate the median and mode.
///
/// # Return
///
/// A tuple where the first value is the median, the second value is the mode.
///
/// # Examples
///
/// ```rust
/// let nums = vec![50, 30, 40, 1, 2, 3, 4, 5, 6, 7, 8, 2, 2];
/// let (median, mode) = get_median_and_mode(nums);
/// assert_eq!(median, 5);
/// assert_eq!(mode, 2);
/// ```
fn get_median_and_mode(nums: Vec<isize>) -> (isize, isize) {
    // TODO 1. sorting if needed
    // TODO 2. compute median
    // TODO 3. compute mode
    return (0, 0);
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use statistical::median;
    use statistical::mode;
    use super::*;

    /// Generates a random vector of numbers of the specified length.
    fn gen_rand_nums_vector(len: usize) -> Vec<isize> {
        if len > 1000 { gen_huge_rand_vec(len) } else { gen_small_rand_vec(len) }
    }

    /// Generates a large random vector of numbers.
    fn gen_huge_rand_vec(len: usize) -> Vec<isize> {
        (0..len).map(move |_| rand::thread_rng().gen_range(isize::MIN..isize::MAX)).collect()
    }

    /// Generates a small random vector of numbers.
    fn gen_small_rand_vec(len: usize) -> Vec<isize> {
        let mut rng = rand::thread_rng();
        let mut vec: Vec<isize> = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(rng.gen_range(isize::MIN..isize::MAX));
        }
        return vec;
    }


    #[test]
    fn experiment() {
        let mut nums = vec![50, 30, 40, 1, 2, 3, 4, 5, 6, 7, 8, 2, 2];
        println!("The median: {}", median(&nums));
        println!("The mode: {}", mode(&nums).unwrap());
        nums.sort();
        println!("Sorted vector: {:?}", &nums);
    }

    #[test]
    fn test_get_middle_and_mode() {
        let nums = gen_rand_nums_vector(10);
        let expected_median = median(&nums);
        let expected_mode = mode(&nums).unwrap();

        let (median, mode) = get_median_and_mode(nums);

        assert_eq!(median, expected_median);
        assert_eq!(mode, expected_mode);
    }

    #[test]
    fn test_get_middle_and_mode_with_duplicates() {
        let mut nums = vec![5; 10];
        nums.append(gen_rand_nums_vector(2000).as_mut());
        let expected_median = median(&nums);
        let expected_mode = mode(&nums).unwrap();

        let (median, mode) = get_median_and_mode(nums);
        assert_eq!(median, expected_median);
        assert_eq!(mode, expected_mode);
    }

    #[test]
    fn test_get_middle_and_mode_empty() {
        let nums: Vec<isize> = vec![];
        let (median, mode) = get_median_and_mode(nums);
        assert_eq!(median, 0);
        assert_eq!(mode, 0);
    }
}