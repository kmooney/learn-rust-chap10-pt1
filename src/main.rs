fn largest<T: PartialOrd>(list: &[T]) -> &T {
	let mut largest = &list[0];
	for item in list {
		if largest < item {
			largest = item;
		}
	}
	largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest::<i32>(&number_list));

    let char_list = vec!['a', 'g', 'z', 'x', 'a', 'b'];
    println!("The largest character is {}", largest::<char>(&char_list));
}
