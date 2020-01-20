use std::format;
use std::string::String;

fn main() {
    println!("TWELVE DAYS OF CHRISTMAS");

    let mut day_ctr = 1;
    while day_ctr <= 12 {
        println!(
            "On the {} day of Christmas, my true love sent to me",
            get_number_ordinality(day_ctr)
        );

        let mut verse_ctr = day_ctr;
        let is_in_first_verse = if day_ctr == 1 {
            true
        } else {
            false
        };
        while verse_ctr > 0 {
            let line = match verse_ctr {
                1 => match is_in_first_verse {
                    true => "A partridge in a pear tree.",
                    false => "And a partridge in a pear tree.",
                },
                2 => "Two turtle doves,",
                3 => "Three French hens,",
                4 => "Four calling birds,",
                5 => "Five gold rings,",
                6 => "Six geese a-laying,",
                7 => "Seven swans a-swimming,",
                8 => "Eight maids a-milking,",
                9 => "Nine ladies dancing,",
                10 => "Ten lords a-leaping,",
                11 => "Eleven pipers piping,",
                12 => "Twelve drummers drumming,",
                _ => panic!("verse_ctr should not have reached more than 12 or less than 1!"),
            };

            println!("{}", line);

            verse_ctr -= 1;
        }

        println!("");

        day_ctr += 1;
    }
}

fn get_number_ordinality(num: i32) -> String {
    let has_unique_ordinality = if num % 10 > 0 && num % 10 <= 3 {
        true
    } else {
        false
    };

    match has_unique_ordinality {
        true => match num {
            1 => format!("{}st", num),
            2 => format!("{}nd", num),
            3 => format!("{}rd", num),
            _ => format!("{}th", num),
        },
        false => format!("{}th", num),
    }
}
