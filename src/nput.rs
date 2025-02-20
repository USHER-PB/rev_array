use std::io;

pub fn input() -> ([i32; 20], usize, usize) {
    //  let mut rev: [i32; 4] = [0; 4];

    let mut inputs = String::new();
    println!("ask the user how many values do you want to enter");
    io::stdin()
        .read_line(&mut inputs)
        .expect("it seems as if no value was enter please enter back a value");
    let inputs: usize = inputs
        .trim()
        .parse()
        .expect("this is not okay .. the value you enter is not an integer");
    let mut array: [i32; 20] = [0; 20];
    //  let mut index = 0;
    println!("please enter the values you want for the array");
    for i in 0..inputs {
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .expect("it seems as if no value was enter please enter back a value");

        let numbers: i32 = numbers
            .trim()
            .parse()
            .expect("this is not okay .. the value you enter is not an integer");
        array[i] = numbers;
        // index += 1 ;
    }
    let mut i = array.len() - 1;

    let mut j = 0;

    while i > j {
     

          let temp = array[i];
          array[i] = array[j];
          array[j] = temp;
        i -= 1;
        j += 1;
    }

    (array, j, inputs)
}
