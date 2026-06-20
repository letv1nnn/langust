use linal::array::null::NullBuffer;

fn main() {
    let mut nb = NullBuffer::from(vec![0u8, 1u8, 1u8, 1u8, 0u8, 1u8]);
    println!("{}", nb);
    nb.set_valid(0usize);
    println!("{}", nb);
}
