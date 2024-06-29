use std::collections::HashMap;

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
pub fn get_median_and_mode(nums: &Vec<isize>) -> Option<(isize, isize)> {
    if nums.is_empty() {
        return None;
    }
    let mut nums_copy = nums.clone();
    nums_copy.sort_unstable();
    let median = get_median(&nums_copy);
    let mode = get_mode(&nums_copy).unwrap();
    return Some((median, mode));
}

fn get_median(nums: &Vec<isize>) -> isize {
    let mid = nums.len() / 2;

    return if nums.len() % 2 == 0 {
        (nums[mid - 1] + nums[mid]) / 2
    } else {
        nums[mid]
    };
}

fn get_mode(nums: &Vec<isize>) -> Option<isize> {
    let mut nums_counter: HashMap<isize, i32> = HashMap::new();
    for &element in nums {
        *nums_counter.entry(element).or_insert(0) += 1;
    }

    return nums_counter.into_iter().max_by_key(|&(_, count)| count).map(|(key, _)| key);
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
    fn odd_vec_no_duplicates() {
        let nums = gen_rand_nums_vector(13);
        universal_test_median(nums.clone());
        // Because the check method returns a randomly different element
        // when there are no duplicates among the elements.
        //universal_test_mode(nums);
    }

    #[test]
    fn odd_vec_with_duplicates() {
        let mut nums = gen_rand_nums_vector(13);
        nums.extend(vec![99, 99, 99, 99]);
        universal_test_median(nums.clone());
        universal_test_mode(nums);
    }

    #[test]
    fn even_vec_no_duplicates() {
        let nums = gen_rand_nums_vector(14);
        universal_test_median(nums.clone());
        // Because the check method returns a randomly different element
        // when there are no duplicates among the elements.
        //universal_test_mode(nums);
    }

    #[test]
    fn even_vec_with_duplicates() {
        let mut nums = gen_rand_nums_vector(14);
        nums.extend(vec![99, 99, 99, 99]);
        universal_test_median(nums.clone());
        universal_test_mode(nums);
    }

    #[test]
    fn try_vec_with_one_element() {
        universal_test_median(vec![1]);
        universal_test_mode(vec![1]);
    }

    #[test]
    fn test_with_same_frequency() {
        let nums = vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 9193722811479871133];
        universal_test_median(nums.clone());
        // Because the check method returns a randomly different element
        // when there are no duplicates among the elements.
        //universal_test_mode(nums);
    }

    #[test]
    fn test_get_median_and_mode_empty() {
        let empty_nums: Vec<isize> = vec![];
        let result = get_median_and_mode(&empty_nums);
        assert_eq!(result, None, "An empty vector should result in an empty result.");
    }

    fn universal_test_median(mut test_vector: Vec<isize>) {
        let expected_median = median(&test_vector);
        let (median, _) = get_median_and_mode(&test_vector).unwrap();

        test_vector.sort();
        let median_message = format!("Median FAILED!\nThe vector contains: {:?}", &test_vector);

        assert_eq!(median, expected_median, "{}", median_message);
    }

    fn universal_test_mode(test_vector: Vec<isize>) {
        let expected_mode = mode(&test_vector).unwrap();

        let (_, mode) = get_median_and_mode(&test_vector).unwrap();
        let mode_message = format!("Mode FAILED!\nThe vector contains: {:?}", &test_vector);

        assert_eq!(mode, expected_mode, "{}", mode_message);
    }
}
