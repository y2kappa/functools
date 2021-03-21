mod vector {
    pub fn init<T>(capacity: usize, f: fn(usize) -> T) -> Vec<T> {
        let mut v = Vec::with_capacity(capacity);
        for i in 0..capacity {
            v.push(f(i));
        }
        v
    }

    pub fn zip_with<T, U, V>(mut first: Vec<T>, mut second: Vec<U>, with: fn(T, U) -> V) -> Vec<V> {
        if first.len() != second.len() {
            panic!("You need to have equal lists");
        }

        let length = first.len();
        let mut v = Vec::with_capacity(length);
        for _ in 0..length {
            let x = first.drain(0..1).next().unwrap();
            let y = second.drain(0..1).next().unwrap();
            v.push(with(x, y));
        }
        v
    }

    pub fn zip<T, U>(first: Vec<T>, second: Vec<U>) -> Vec<(T, U)> {
        zip_with(first, second, |x, y| (x, y))
    }
}

mod truth {
    // or something that implements iter
    pub fn all<'a, T>(elems: &'a Vec<T>, f: fn(&'a T) -> bool) -> bool {
        elems.iter().fold(true, |acc, x| acc && f(x))
    }

    pub fn any<'a, T>(elems: &'a Vec<T>, f: fn(&'a T) -> bool) -> bool {
        elems.iter().fold(false, |acc, x| acc || f(x))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_vec_init() {
        let v = vector::init(4, |x| x.pow(2));
        assert_eq!(v[0], 0);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 4);
        assert_eq!(v[3], 9);

        println!("{:?}", v);
    }

    #[test]
    fn test_zip_with() {
        let xs = vec![1, 2, 3];
        let ys = vec![4, 5, 6];

        let zs = vector::zip_with(xs, ys, |x, y| x + y);
        println!("{:?}", zs);

        assert_eq!(zs[0], 5);
        assert_eq!(zs[1], 7);
        assert_eq!(zs[2], 9);
    }

    #[test]
    fn test_zip() {
        let xs = vec![1, 2, 3];
        let ys = vec![4, 5, 6];

        let zs = vector::zip(xs, ys);
        println!("{:?}", zs);

        assert_eq!(zs[0], (1, 4));
        assert_eq!(zs[1], (2, 5));
        assert_eq!(zs[2], (3, 6));
    }

    #[test]
    fn test_all() {
        let xs = vec![1, 3, 5, 7, 9];
        let is_odd = |x| x % 2 != 0;

        assert_eq!(truth::all(&xs, is_odd), true)
    }

    #[test]
    fn test_any() {
        let xs = vec![1, 4, 5, 7, 9];
        let is_even = |x| x % 2 == 0;

        assert_eq!(truth::any(&xs, is_even), true)
    }
}
