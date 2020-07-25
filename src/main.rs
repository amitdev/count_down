use count_down::count_down;

fn main() {
    let results = count_down::solutions(vec![1, 3, 7, 10, 15, 50], 765);
    for result in results {
        println!("{:?}", result);
    }
}
