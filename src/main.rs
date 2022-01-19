fn main() {
    //Random Variable
    let s = "World!";
    println!("Hello, {}", s);

    //Integer variables
    let mut num:i32 = 5;
    println!("Current value is {}", num);
    num = 9;
    println!("Current value is {}", num);

    //Boolean Variables
    let x:bool = true;
    if x {
        println!("X is True Bitch!");
    }

    //Tuples for accessing multiple values of different data types at once
    let tuple = (9, "Ashutosh Tripathi", true);
    println!("The values stored in Tuple are : {} {} {}", tuple.0, tuple.1, tuple.2);

    //Arrays in Rust
    let arr = [1,2,3,4,5];
    println!("{}", arr[4]);

    //Conditional Statements
    if x == true {
        println!("You Won");
    }

    else {
        println!("You Lost");
    }

    //For Loop Similar to Python
    for i in 0..10
    {
        println!("{}", i);
    }

    //Traversing array using For Loop Iterators
    for val in arr
    {
        println!("{}", val);
    }

    //Match Statements
    num = 69;
    match num {
        69 => println!("Horny AF!!!"),              //Use Commas Here
        99 => println!("Mood to Spooooonnnn"),
        _  => println!("Normal Cuddling"),
    }

    println!("You have mastered the basics of Rust programming language");
}
