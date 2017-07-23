pub fn twelve_days() {
    let verses = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Gold Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Tweleve Drummers Drumming",
    ];

    let days = [
        "First",
        "Second",
        "Third",
        "Fourth",
        "Fifth",
        "Sixth",
        "Seventh",
        "Eighth",
        "Ninth",
        "Tenth",
        "Eleventh",
        "Twelfth",
    ];

    for day in 1..13 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            days[day - 1]
        );
        let mut j = day - 1;
        while j >= 1 {
            println!("{}", verses[j]);
            j = j - 1;
        }
        if day == 1 {
            println!("{}", verses[0]);
        } else {
            println!("And {}", verses[0]);
        }
        println!("");
    }
}
