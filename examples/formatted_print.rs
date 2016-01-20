fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    println!("{subject} {verb} {predicate}.",
            predicate="over the lazy dog",
            subject="The quick brown fox",
            verb="jumps");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{word:>width$}", word="Append to right", width=45);
}
