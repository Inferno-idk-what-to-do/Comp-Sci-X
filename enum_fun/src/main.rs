#[derive(Debug)]
enum Activity {
    Coding(String),
    Exercising(i32),
    Relaxing,
    School(Vec<String>),
}

fn match_activity(a: Activity) {
    match a {
        Activity::Coding(x) => {
            match x.as_str() {
                "enum fun" => println!("you're here!"),
                _ => println!("sounds like fun")
            }
        },
        Activity::Exercising(x) => println!("only {} mins? you can do better.", x),
        Activity::School(x) => println!("{} periods a day? rip. oh, and {} first peroid? rough.", x.len(), x[0]),
        _ => println!("start coding")
    };
}

fn main() {
    let current: Activity = Activity::Coding(String::from("enum fun"));
    let soon: Activity = Activity::Exercising(20);
    let was: Activity = Activity::Relaxing;
    let monday: Activity = Activity::School(vec![
        String::from("music theory"), 
        String::from("orchestra"), 
        String::from("gov"), 
        String::from("linear algebra")
    ]);
    
    print!("current -> ");
    match_activity(current);

    print!("soon -> ");
    match_activity(soon);
    
    print!("was -> ");
    match_activity(was);
    
    print!("monday -> ");
    match_activity(monday);
}
