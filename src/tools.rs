pub fn find_with_index<T, K: Eq>(
    array: &[T],
    key: K,
    transition: Fn(T) -> K,
) -> Option<(T, usize)> {
    for i in 0..array.len() {
        let item = array[i];
        if key == transition(item) {
            return Some((item, i));
        }
    }
    None
}
