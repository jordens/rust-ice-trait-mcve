pub trait TreeKey<const Y: usize = 1> {
    fn metadata() {
        unimplemented!()
    }
}

impl<T, const N: usize> TreeKey for [T; N] {}
