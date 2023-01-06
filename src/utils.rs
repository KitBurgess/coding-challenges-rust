use std::collections::VecDeque;

pub struct Utils {}

impl Utils {
    pub fn transpose<T>(v: VecDeque<Vec<T>>) -> VecDeque<Vec<T>> {
        return if v.is_empty() {
            v
        } else {
            let len = v[0].len();
            let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
            (0..len)
                .map(|_| {
                    iters
                        .iter_mut()
                        .map(|n| n.next().unwrap())
                        .collect::<Vec<T>>()
                })
                .collect()
        };
    }
}