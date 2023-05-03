use std::{
    env,
    fs::File,
    io::{ErrorKind, Read},
};

fn main() {
    let search_str = env::args().nth(1);
    let file_path = env::args().nth(2);

    if search_str == None || file_path == None {
        println!("!! Both search string and file path is required");
        return;
    }

    let search_str_ref = search_str.as_ref().unwrap();
    let file_path_ref = file_path.as_ref().unwrap();

    let file = File::open(file_path_ref);

    match file {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content).unwrap();
            let mut count = 0;

            let lines: Vec<&str> = content.split("\n").collect();

            println!("\n****** Results *******\n");

            for (i, line) in lines.into_iter().enumerate() {
                if line.to_lowercase().contains(&search_str_ref.to_lowercase()) {
                    for word in line.split(" ") {
                        if word.to_lowercase().trim() == search_str_ref.to_lowercase() {
                            count += 1;
                        }
                    }
                    println!("{}. {}", i + 1, line);
                }
            }

            println!(
                "\nThe word `{}` appears `{}` times in `{}`",
                search_str_ref, count, file_path_ref
            )
        }
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                println!("Provided file path not found")
            }
            _ => {
                println!("Something went wrong while opening the file")
            }
        },
    }
}

// fn check_if_string_contains(
//     content: String,
//     search_str: Option<String>,
//     file_path: Option<String>,
// ) {
//     if content
//         .to_lowercase()
//         .contains(&search_str.as_ref().unwrap().to_lowercase())
//     {
//         println!(
//             "The word `{}` does contains in {}",
//             &search_str.unwrap(),
//             &file_path.unwrap()
//         );
//     } else {
//         println!(
//             "The word `{}` doesn't contains in {}",
//             &search_str.unwrap(),
//             &file_path.unwrap()
//         );
//     }
// }
