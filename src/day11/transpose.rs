pub trait Transpose {
    fn transpose(self) -> Self;
}

impl<T> Transpose for Vec<Vec<T>> {
    fn transpose(self) -> Self {
        if self.len() == 0 {
            return self;
        }

        let len = self[0].len();
        let mut iters: Vec<_> = self.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}
