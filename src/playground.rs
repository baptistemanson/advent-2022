struct ByCol<'a, T> {
    rows: Vec<std::slice::Iter<'a, T>>,
}
trait Columnizable<T> {
    fn by_col(&self) -> ByCol<T>;
}

impl<'a, T> Iterator for ByCol<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.rows.iter_mut().map(|iter| iter.next()).collect()
    }
}

impl<T> Columnizable<T> for Vec<Vec<T>> {
    fn by_col(&self) -> ByCol<T> {
        let rows: Vec<_> = self.into_iter().map(|v| v.into_iter()).collect();
        return ByCol { rows };
    }
}

pub fn main() {
    let array = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    for column in array.by_col() {
        println!("{:?}", column)
    }
}

#[allow(dead_code)]
const INPUT: &str = "";
