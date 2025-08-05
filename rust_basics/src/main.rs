fn main() {
    println!("Hello, world!");

    // let b = 12;
    // b = 34;
    // we cannot change the value it is immutable

    // we have to add mut keyword to change the value of the variable
    let  a = 24;
    let a = 25;
    println!("The Number : {a}");

    //const must have its type we can't declare const without its type 
    const AB : u8 = 10;

    println!("Value of const : {AB}");

}