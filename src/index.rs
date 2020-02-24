pub trait Visitor {
    fn visit<'a>(&mut self, idx: usize, ptr: u64, name: &'a [u8]);
}
