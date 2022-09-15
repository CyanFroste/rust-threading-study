use std::{
    error::Error,
    io::{Read, Seek, Write},
    sync::{Arc, RwLock},
    thread,
};

pub fn read_write_to_same_file() -> Result<(), Box<dyn Error>> {
    const NUM_THREADS: usize = 2;
    const NUM_ADDITIONS: usize = 10;

    // `my_file.txt` is a file with a number `x` spanning `n` lines
    //  eg. my_file.txt
    //  1
    //  1
    //  1
    //  1

    // Open file in `rw` mode
    let mut file = std::fs::File::options()
        .read(true)
        .write(true)
        .open("my_file.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // let file_ref = Arc::new(Mutex::new(file)); // Mutex
    let file_ref = Arc::new(RwLock::new(file));
    let mut handles = vec![];

    for _ in 0..NUM_THREADS {
        // Clone the Atomic Reference
        let file = file_ref.clone();

        let handle = thread::spawn(move || {
            for _ in 0..NUM_ADDITIONS {
                // let mut file = file.lock().expect("Deadlock must have occured!"); // Mutex
                let mut file = file.write().unwrap();

                // Read file
                let mut contents = String::new();
                file.rewind().unwrap();
                file.read_to_string(&mut contents).unwrap();

                // Parse each line as i32
                let numbers: Vec<_> = contents
                    .lines()
                    .map(|it| {
                        it.trim()
                            .parse::<i32>()
                            .expect("Each line should be a number")
                    })
                    .collect();

                // Do operation
                let contents = numbers
                    .iter()
                    .map(|it| format!("{}", it + 1))
                    .collect::<Vec<_>>()
                    .join("\n");

                // Clear file
                file.set_len(0).unwrap();
                // Set cursor to beginning of file
                file.rewind().unwrap();
                // Write to file
                file.write(contents.as_bytes()).unwrap();
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
