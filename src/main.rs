use nput::input;

mod nput;

fn main() {

let (array, _, inputs) = input();  

   if inputs < 20-inputs {
    let num = 20 - inputs ;
    println!("the array was {:?}", &array[num..20]);
   }

  else if inputs > 20-inputs{
    let num = 20 -inputs;
   
   
    println!("the array was {:?}", &array[num..20]);
  }
}
   

