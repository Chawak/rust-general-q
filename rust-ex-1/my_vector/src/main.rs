use my_vector::MyVec;

fn main() {
    let mut vec : MyVec<usize> = MyVec::new();
    for i in 1..=5{
        vec.push(i);
    }
    println!("{}",vec[2]);
}