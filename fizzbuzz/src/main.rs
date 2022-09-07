fn main() {
    for i in 1..=100 {
        println!("{}", 
                    if format!("{}{}", 
                        if i%3==0 {String::from("fizz")} else {String::from("")},
                        if i%5==0 {String::from("buzz")} else {String::from("")}) == ""
                            { i.to_string() } 
                    else 
                        { format!("{}{}", 
                        if i%3==0 {String::from("fizz")} else {String::from("")},
                        if i%5==0 {String::from("buzz")} else {String::from("")}) });
    }
}
