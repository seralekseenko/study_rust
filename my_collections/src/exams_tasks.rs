/// Compute the [median](https://en.wikipedia.org/wiki/Median) and the
/// [mode](https://en.wikipedia.org/wiki/Mode_(statistics)) from a vector of numbers.
///
/// # Arguments
///
/// * nums - A vector of numbers (sorted/unsorted) for which you want to calculate the median and mode.
///
/// # Return
///
/// An Option of tuple where the first value is the median, the second value is the mode.
/// If the vector is empty — return None.
///
/// # Examples
///
/// ```rust
/// let nums = vec![50, 30, 40, 1, 2, 3, 4, 5, 6, 7, 8, 2, 2];
/// let (median, mode) = get_median_and_mode(nums);
/// assert_eq!(median, 5);
/// assert_eq!(mode, 2);
/// ```
fn get_median_and_mode(nums: Vec<isize>) -> Option<(isize, isize)> {
    // TODO 0. check if empty
    // TODO 1. sorting if needed
    // TODO 2. compute median
    // TODO 3. compute mode
    return Some((0, 0));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
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
        let mut rng = rand::thread_rng();
        let mut set = HashSet::new();

        (0..)
            .map(move |_| rng.gen_range(isize::MIN..isize::MAX))
            .filter(|&x| set.insert(x))  // Додає унікальні елементи в HashSet і пропускає дублікати
            .take(len)  // Беремо тільки len елементів
            .collect()
    }

    /// Generates a small random vector of numbers.
    fn gen_small_rand_vec(len: usize) -> Vec<isize> {
        let mut set = HashSet::new();
        let mut rng = rand::thread_rng();

        while set.len() < len {
            set.insert(rng.gen_range(isize::MIN..isize::MAX));
        }

        set.into_iter().collect()
    }


    #[test]
    fn experiment() {
        let mut nums = vec![500, 300, 100, 200, 300, 400, 500, 600, 700, 800, 200, 200, 200, 500];
        println!("The median: {}", median(&nums));
        println!("The mode: {}", mode(&nums).unwrap());
        nums.sort();
        println!("Sorted vector: {:?}", &nums);
    }

    #[test]
    fn odd_vec_no_duplicates() {
        universal_test_median_mode(gen_rand_nums_vector(13))
    }

    #[test]
    fn odd_vec_with_duplicates() {
        let mut nums = gen_rand_nums_vector(13);
        nums.extend(vec![99, 99, 99, 99]);
        universal_test_median_mode(nums);
    }

    #[test]
    fn even_vec_no_duplicates() {
        universal_test_median_mode(gen_rand_nums_vector(14))
    }

    #[test]
    fn even_vec_with_duplicates() {
        let mut nums = gen_rand_nums_vector(14);
        nums.extend(vec![99, 99, 99, 99]);
        universal_test_median_mode(nums);
    }

    #[test]
    fn try_vec_with_one_element() {
        universal_test_median_mode(vec![1])
    }

    #[test]
    fn test_get_middle_and_mode_empty() {
        let empty_nums: Vec<isize> = vec![];
        let result = get_median_and_mode(empty_nums);
        assert_eq!(result, None);
    }

    fn universal_test_median_mode(test_vector: Vec<isize>) {
        let expected_median = median(&test_vector);
        let expected_mode = mode(&test_vector).unwrap();

        let (median, mode) = get_median_and_mode(test_vector).unwrap();

        assert_eq!(median, expected_median);
        assert_eq!(mode, expected_mode);
    }
}