use itertools::Itertools;
use std::fs;

pub fn read_file_into_vector<T, F>(path: &str, fun: F) -> Vec<T>
where
    F: Fn(&str) -> T,
{
    fs::read_to_string(path)
        .expect("input data missing")
        .lines()
        .map(fun)
        .collect_vec()
}
