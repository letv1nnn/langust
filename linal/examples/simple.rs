use linal::array::{core::Array, null::NullBuffer};

fn main() {
    println!("NullBuffer:");
    let mut nb = NullBuffer::from(vec![0u8, 1u8, 1u8, 1u8, 0u8, 1u8]);
    println!("{}", nb);
    nb.set_valid(0usize);
    println!("{}", nb);

    println!("Array<T>:");
    let arr: Array<f32> = vec![3.14f32, 12.34f32, 0.0f32].into();
    println!("{:?}", arr);
    let arr: Array<f32> = vec![Some(3.14f32), None, Some(12.23f32)].into();
    println!("{:?}", arr);
}
