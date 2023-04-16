pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a.abs() < b.abs() {
        return gcd(b, a)
    }

    while b.abs() > 0 {
        let r: i64 = a % b;
        a = b;
        b = r;
    }

    a
}

pub fn extended_euclidean(a: i64, b: i64) -> (i64, i64, i64) {
    if b.abs() > a.abs() {
        let (x, y, d) = extended_euclidean(b, a);
        return (y, x, d);
    }

    if b.abs() == 0 {
        return (1, 0, a);
    }

    let mut x1 = 0;
    let mut x2 = 1;
    let mut y1 = 1;
    let mut y2 = 0;

    let mut aa = a;
    let mut bb = b;

    while bb.abs() > 0 {
        let q = aa / bb;
        let r = aa % bb;
        let x = x2 - q * x1;
        let y = y2 - q * y1;
        aa = bb;
        bb = r;
        x2 = x1;
        x1 = x;
        y2 = y1;
        y1 = y;
    }

    (x2, y2, aa)
}

fn main() {
    let output: (i64, i64, i64) = extended_euclidean(84, 18);
    println!("Extended Euclidean algorithm on 84 and 18 gives ({}, {}, {}).", output.0, output.1, output.2);
}

#[cfg(test)]
mod tests {
    use super::*;

    // prints out triples in a pretty format for debugging
    fn print_triple(triple: (i64, i64, i64)) -> String {
        format!("({}, {}, {})", triple.0, triple.1, triple.2)
    }

    // checks if the triples have the same values in a different order
    fn check_triples(triple1: (i64, i64, i64), triple2: (i64, i64, i64)) -> bool {
        let mut triple1_vec = vec![triple1.0, triple1.1, triple1.2];
        let mut triple2_vec = vec![triple2.0, triple2.1, triple2.2];

        triple1_vec.sort();
        triple2_vec.sort();

        triple1_vec == triple2_vec
    }

    // tests stolen frome https://www.calstatela.edu/sites/default/files/users/u2536/MATH-4460/hw2solutions.pdf
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 24), 12);
        assert_eq!(gcd(16, 36), 4);
        assert_eq!(gcd(39, 17), 1);
        assert_eq!(gcd(5, 18), 1);
        assert_eq!(gcd(0, 3), 3);
        assert_eq!(gcd(2689, 4001), 1);
        assert_eq!(gcd(1819, 3587), 17);
        assert_eq!(gcd(864, 468), 36);
    }

    // tests stolen from https://www.extendedeuclideanalgorithm.com/calculator.php?mode=1&a=32&b=18#num
    #[test]
    fn test_extended_euclidean() {

        let results = vec![
            extended_euclidean(84, 18),
            extended_euclidean(35, 15),
            extended_euclidean(30, 20),
            extended_euclidean(32, 18),
            extended_euclidean(17, 17),
        ];

        let expected = vec![
            (-1, 5, 6),
            (5, 1, -2),
            (10, 1, -1),
            (2, 4, -7), 
            (0, 1, 17) 
        ];

        for i in 0..results.len() {
            let left: String = print_triple(results[i]);
            let right: String = print_triple(expected[i]);
            assert!(
                check_triples(results[i], expected[i]), 
                "left: {}, right: {}", left, right 
            );
        }

    }
}

