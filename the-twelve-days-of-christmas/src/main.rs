const ORDINAL_NUMS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];
const LYRICS: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];
fn main() {
    for i in 0..12 {
        println!("[Verse {}]", i + 1);
        print_first_sentence_of_verse(ORDINAL_NUMS[i]);
        for j in (0..(i + 1)).rev() {
            println!("{}", LYRICS[j]);
        }
        println!("");
    }
}
fn print_first_sentence_of_verse(ord_num: &str) -> () {
    println!("On the {ord_num} day of Christmas, my true love sent to me.")
}
