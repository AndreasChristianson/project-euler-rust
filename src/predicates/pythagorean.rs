pub fn is_pythagorean_triplet<T: num::PrimInt>(a: T, b: T, c: T) -> bool {
    return a * a + b * b == c * c;
}

#[cfg(test)]
mod is_pythagorean_triplet_tests {
    use super::is_pythagorean_triplet;
    #[test]
    fn trivial_triplets() {
        for (a, b, c) in vec![(0, 0, 0)] {
            assert!(is_pythagorean_triplet(a, b, c));
        }
    }

    #[test]
    fn triplets() {
        for (a, b, c) in vec![
            (23, 264, 265),
            (96, 247, 265),
            (69, 260, 269),
            (115, 252, 277),
            (160, 231, 281),
            (161, 240, 289),
            (68, 285, 293),
        ] {
            assert!(is_pythagorean_triplet(a, b, c));
        }
    }
    #[test]
    fn large_triplets() {
        for (a, b, c) in vec![
            (79u128, 3120u128, 3121u128),
            (2499, 100, 2501),
            (2303, 96, 2305),
            (4899, 140, 4901),
            (6237, 316, 6245),
            (6399, 160, 6401),
        ] {
            assert!(is_pythagorean_triplet(a, b, c));
        }
    }
    #[test]
    fn not_triplets() {
        for (a, b, c) in vec![
            (2, 2, 2),
            (1, 1, 1),
            (1000, 1000, 10),
            (-1, -1, -1),
            (0, 0, 1),
            (-5, -55, -555),
        ] {
            assert_eq!(is_pythagorean_triplet(a, b, c), false);
        }
    }
}
