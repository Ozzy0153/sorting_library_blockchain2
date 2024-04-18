use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};

fn main() {
    let mut numbers = vec![5, 2, 7, 3, 9, 1, 4, 6, 8];
    quick_sort(&mut numbers);
    println!("Quick Sorted: {:?}", numbers);

    let mut strings = vec!["banana", "apple", "orange", "grape", "kiwi"];
    selection_sort(&mut strings);
    println!("Selection Sorted: {:?}", strings);

    let floats = vec![3.5, 1.2, 5.6, 2.3, 4.7];
    let mut floats_as_f64: Vec<f64> = floats.iter().map(|&x| x as f64).collect();
    insertion_sort(&mut floats_as_f64);
    println!("Insertion Sorted: {:?}", floats_as_f64);

    let mut chars = vec!['c', 'b', 'a', 'e', 'd'];
    merge_sort(&mut chars);
    println!("Merge Sorted: {:?}", chars);
}
