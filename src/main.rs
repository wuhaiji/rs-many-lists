struct Car<'a> {
    name: &'a String,
}

fn print_name<'b>(c: &'b mut Car) {
    println!("{:?}", c.name)
}

fn main() {
    let mut c = Car { name: &"AAA".to_string() };
    print_name(&mut c);
    print_name(&mut c);
    
}