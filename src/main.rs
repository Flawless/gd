use std::{env, fs, path::Path};

fn read_file_numbers<P>(filepath: P) -> Vec<i128>
where
    P: AsRef<Path>,
{
    fs::read_to_string(filepath)
	.expect("Error while reading the file!")
	.lines()
	.flat_map(|s| s.split_whitespace())
	.map(|s| s.parse::<i128>().expect("NaN found in file!"))
	.collect()
}

/// Returns the first dangerous number in a cave and its position
/// the complexity is n^2
fn find_dangerous_number(cave: &Vec<i128>, size: usize) -> Option<(usize, i128)> {
    cave.windows(size + 1)
	.map(|w| w.split_last().unwrap())
	.enumerate()
	.filter_map(|(i, (x, w))| {
	    w.iter()
		.map(|e| (x - e))
		.enumerate()
		.filter(|(i, e)| w.iter().skip(i + 1).any(|ie| ie == e))
		.next()
		.is_none()
		.then_some((i + size, x.clone()))
	})
	.next()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let size: usize = args[2]
	.trim()
	.parse()
	.expect("Please, provide size as integer.");

    let data: Vec<i128> = read_file_numbers(filename);
    let first_dangerous_number = find_dangerous_number(&data, size);

    match first_dangerous_number {
	Some(v) => println!("The first dangerous number is: {:?}", v),
	None => println!("The cave is 100% safe!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::find_dangerous_number;

    #[test]
    fn dangerous_number() {
	let input = vec![
	    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
	    576,
	];
	assert_eq!(find_dangerous_number(&input, 5).unwrap(), (14, 127));
    }
}
