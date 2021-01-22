fn main() {
    for n in 1..=100 {
        let fizz = n % 3 == 0;
        let buzz = n % 5 == 0;

        if fizz && buzz {
            println!("{}", "fizzbuzz");
        }
        else if fizz {
            println!("{}", "fizz");
        }
        else if buzz {
            println!("{}", "buzz");
        }
        else {
            println!("{}", n);
        }
    }
}
