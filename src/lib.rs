pub trait T<const Y: usize = 1> {
    fn f() {}
}
impl T for () {}
