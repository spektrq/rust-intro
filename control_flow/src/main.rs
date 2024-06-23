use std::io;

fn main() {
    let mut input = String::new();
    loop {
        println!("Enter a temperature in Fahrenheit to convert it to Celsius.");

        

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
        let temp: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        println!("{}F is {}C", input.trim(), (temp - 32) * 5 / 9);
        break;
    }

    input = String::new();
    
    loop {
        println!("Enter which Fibonnaci number you want to find out.");

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let mut first = 1;
        let mut second = 1;
        let mut result = -1;
        
        
        for number in 0..n {
            result = first + second;
            
            if n == number {
                break;
            }

            first = second;
            second = result;
        }

        println!("Fibonnaci number is: {}", result);
        break;
    };

    for number in 1..12 {
        let mut day = "first";
        let mut gift= String::from(if number == 1 {"A partridge in a pear tree"} else {"And a partridge in a pear tree"});  
        
        if number >= 2 {
            day = "second";
            gift = format!("Two turtle doves,\n{}", gift);
        } 

        if number >= 3 {
            day = "third";
            gift = format!("Three French hens,\n{}", gift);
        }

        if number >= 4 {
            day = "fourth";
            gift = format!("Four calling birds,\n{}", gift);
        } 

        if number >= 5 {
            day = "fifth";
            gift = format!("Five gold rings,\n{}", gift);
        }

        if number >= 6 {
            day = "sixth";
            gift = format!("Six geese a-laying,\n{}", gift);
        } 

        if number >= 7 {
            day = "seventh";
            gift = format!("Seven swans a-swimming,\n{}", gift);
        }

        if number >= 8 {
            day = "eight";
            gift = format!("Eight maids a-milking,\n{}", gift);
        } 

        if number >= 9 {
            day = "ninth";
            gift = format!("Nine ladies dancing,\n{}", gift);
        } 

        if number >= 10 {
            day = "tenth";
            gift = format!("Ten lords a-leaping,\n{}", gift);
        }
        
        if number >= 11 {
            day = "eleventh";
            gift = format!("Eleven pipers piping,\n{}", gift);
        }

        if number == 12 {
            day = "twelfth";
            gift = format!("Twelve drummers drumming,\n{}", gift);
        }
        
        println!("On the {} of Christmas my true love sent to me\n{}\n\n", day, gift)

    }
    
}
