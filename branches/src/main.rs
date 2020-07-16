fn main() {
    let number = 3;

    if number != 0 {
        println!("number was somthing over than 0")
    }

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was flase");
    }

    let a = [1, 10, 20, 100, 4000];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

}
