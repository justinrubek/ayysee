pub fn append<T>(mut accum: Vec<T>, item: T) -> Vec<T> {
    accum.push(item);
    accum
}
