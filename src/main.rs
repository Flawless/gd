use std::{
    env, fs,
    iter::{self, Iterator},
    path::Path,
};

fn read_file_numbers<P>(filepath: P) -> Vec<u128>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filepath)
	.expect("Error while reading the file!")
	.lines()
	.flat_map(|s| s.split_whitespace())
	.map(|s| s.parse::<u128>().expect("NaN found in file!"))
	.collect()
}

/// Calculates and returns the sums of unique pairs in a collection xs over a window of given size
fn calculate_sums<'a>(xs: &'a Vec<u128>, size: usize) -> impl Iterator<Item = u128> + 'a {
    xs.windows(size).flat_map(|window| {
	let x = window.first().unwrap();
	let rest = &window[1..];
	let mut result: Vec<u128> = Vec::new();
	for y in rest.iter() {
	    result.push(x + *y);
	}
	let result = result;
	result
    })
}

/// Returns a vector of indexes of desired sums in sums vector for number on n position
fn get_indexes(size: usize, n: usize) -> Vec<usize> {
    let mut indexes: Vec<usize> = vec![];
    let step = size - 1;
    let offset = n * step;
    for i in 0..step {
	for j in 0..step - i {
	    indexes.push(offset + (i * step) + j);
	}
    }
    indexes
}

/// Returns the first dangerous number in a cave and its position
fn find_dangerous_number(cave: &Vec<u128>, size: usize) -> Option<(usize, u128)> {
    let mut data = cave.clone();

    // pad data with zeros to allow calculate sum for all cave steps, probably there is a space to refactor
    data.extend(iter::repeat(0).take(size - 1));
    let data = data;

    let sums: Vec<u128> = calculate_sums(&data, size).collect();
    // iterate over potentially dangerous numbers
    for (n, value) in cave[size..].into_iter().enumerate() {
	let mut safe = false;
	// check if the number is safe
	for i in get_indexes(size, n).into_iter() {
	    if sums[i] == *value {
		safe = true;
		break;
	    }
	}
	if !safe {
	    return Some((n + size, *value));
	}
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let size: &usize = &args[2]
	.trim()
	.parse()
	.expect("Please, provide size as integer.");

    let data: Vec<u128> = read_file_numbers(filename);
    match find_dangerous_number(&data, *size) {
	Some(v) => println!("The first dangerous number is: {:?}", v),
	None => println!("All path is clear! Be sure."),
    }

    // println!("{:?}", get_indexes(5, 1));
}

#[cfg(test)]
mod tests {
    use crate::{calculate_sums, find_dangerous_number, get_indexes};

    #[test]
    fn indexes() {
	let indexes = get_indexes(5, 1);
	assert_eq!(indexes, vec![4, 5, 6, 7, 8, 9, 10, 12, 13, 16]);
    }

    #[test]
    fn sums() {
	let input = vec![
	    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
	    576, 0, 0, 0, 0,
	];
	let expected_sums = vec![
	    55, 50, 60, 82, 35, 45, 67, 60, 40, 62, 55, 77, 72, 65, 87, 80, 87, 109, 102, 112, 102,
	    95, 105, 135, 117, 127, 157, 164, 120, 150, 157, 172, 160, 167, 182, 215, 197, 212,
	    245, 277, 219, 252, 284, 229, 267, 299, 244, 336, 332, 277, 369, 449, 309, 401, 481,
	    459, 346, 426, 404, 436, 518, 496, 528, 795, 576, 608, 875, 299, 586, 853, 277, 277,
	    885, 309, 309, 309, 576, 576, 576, 576,
	];
	assert_eq!(
	    calculate_sums(&input, 5).collect::<Vec<u128>>(),
	    expected_sums
	);
    }

    #[test]
    fn dangerous_number() {
	let input = vec![
	    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
	    576,
	];
	assert_eq!(find_dangerous_number(&input, 5).unwrap(), (14, 127));
    }
}
