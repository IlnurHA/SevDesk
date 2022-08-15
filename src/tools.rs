pub fn find_with_index<T: Clone, K: Eq, F: Fn(T) -> K>(
    array: &[T],
    key: K,
    transition: F,
) -> Option<(T, usize)> {
    for i in 0..array.len() {
        let item = &array[i];
        if key == transition(item.to_owned()) {
            return Some((item.clone(), i));
        }
    }
    None
}

pub fn find<T: Clone, K: Eq, F: Fn(T) -> K>(array: &[T], key: K, transition: F) -> Option<T> {
    if let Some((item, _)) = find_with_index(array, key, transition) {
        return Some(item);
    }
    None
}
