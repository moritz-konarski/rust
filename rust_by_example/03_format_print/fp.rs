fn main() {

    // this automatically fills arguments in order
    println!("{} days", 31);

    // positional arguments can be used
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // named arguments can also be used
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps");

    // formatting can be done after a ':'
    println!("{} of {:b} people know binary, the other half does not", 1, 2);

    // right aligning text
    println!("{number:>width$}", number=1, width=6);

    // pad with zeroes 
    println!("{number:>0width$}", number=1, width=6);

    // printing to standard error
    eprintln!("This is a mistake");

    // print formatted decimal places
    println!("Pi roughly equals {pi:.3}", pi=3.141592);

}
