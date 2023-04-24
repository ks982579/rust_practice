use basics::sorting;


fn main() {
    let mut list: [i32; 8] = [1,7,3,19,3,5,6,4];
    sorting::int_bubble_sort(&mut list);
    println!("{:?}", list);
}
