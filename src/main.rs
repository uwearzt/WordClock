// ------------------------------------------------------------------------------
// Copyright 2018 Uwe Arzt, mail@uwe-arzt.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// ------------------------------------------------------------------------------

use std::process::Command;
use std::time::Duration;

extern crate time;

// where are the needed words located in the character Array.
struct Word {
    off : u8,
    len : u8
}


fn main() {


// word definitions
let es          = Word { off:   0, len:  2 };
let ist         = Word { off:   3, len:  3 };
let vor         = Word { off:  33, len:  3 };
let funk        = Word { off:  36, len:  4 };
let nach        = Word { off:  40, len:  4 };
let uhr         = Word { off: 107, len:  3 };

// minute definitions
let fuenf       = Word { off:   7, len:  4 };
let zehn        = Word { off:  11, len:  4 };
let viertel     = Word { off:  26, len:  7 };
let zwanzig     = Word { off:  15, len:  7 };
let halb        = Word { off:  44, len:  4 };
let dreiviertel = Word { off:  22, len: 11 };

// hour definitions
let h_ein       = Word { off:  55, len:  3 };
let h_zwei      = Word { off:  62, len:  4 };
let h_drei      = Word { off:  66, len:  4 };
let h_vier      = Word { off:  73, len:  4 };
let h_fuenf     = Word { off:  51, len:  4 };
let h_sechs     = Word { off:  77, len:  5 };
let h_sieben    = Word { off:  88, len:  6 };
let h_acht      = Word { off:  84, len:  4 };
let h_neun      = Word { off: 102, len:  4 };
let h_zehn      = Word { off:  99, len:  4 };
let h_elf       = Word { off:  49, len:  3 };
let h_zwoelf    = Word { off:  94, len:  5 };

// create a array with hour definitions
let hours = [ h_ein, h_zwei, h_drei, h_vier, h_fuenf, h_sechs,
              h_sieben, h_acht, h_neun, h_zehn, h_elf, h_zwoelf];

//*
//* Main loop
//*
loop {
    Command::new("/usr/bin/clear").status().expect("clear failed");

    //*
    //* The logical represenatation of the LEDs on the Display. See display_character for more information.
    //* The LED array is arranged in 11 columns and 10 rows of LEDs + additional 4 minute LEDs.
    //*
    // TODO: Replace with BitSet
    let mut char_leds = [false; 110];
    let mut min_leds  = [false; 4];

    let (mut hour, min) = get_time();

    set_char_leds(&mut char_leds, &es);
    set_char_leds(&mut char_leds, &ist);
    set_char_leds(&mut char_leds, &uhr);

    // minute LEDs
    let leds = (min % 5) as usize;
    for led in 0..leds {
        min_leds[led] = true;
    }

    // hours
    match min {
        0...4 => hour = decrement_hour(hour),
        5...9 => {
            set_char_leds(&mut char_leds, &fuenf);
            set_char_leds(&mut char_leds, &nach);
            hour = decrement_hour(hour);
        },
        10...14 => {
            set_char_leds(&mut char_leds, &zehn);
            set_char_leds(&mut char_leds, &nach);
            hour = decrement_hour(hour);
        },
        15...19 => {
            set_char_leds(&mut char_leds, &viertel);
            set_char_leds(&mut char_leds, &nach);
            hour = decrement_hour(hour);
        },
        20...24 => {
            set_char_leds(&mut char_leds, &zwanzig);
            set_char_leds(&mut char_leds, &nach);
            hour = decrement_hour(hour);
        },
        25...29 => {
            set_char_leds(&mut char_leds, &fuenf);
            set_char_leds(&mut char_leds, &vor);
            set_char_leds(&mut char_leds, &halb);
        },
        30...34 => {
            set_char_leds(&mut char_leds, &halb);
        },
        35...39 => {
            set_char_leds(&mut char_leds, &fuenf);
            set_char_leds(&mut char_leds, &nach);
            set_char_leds(&mut char_leds, &halb);
        },
        40...44 => {
            set_char_leds(&mut char_leds, &zwanzig);
            set_char_leds(&mut char_leds, &vor);
        },
        45...49 => {
            set_char_leds(&mut char_leds, &dreiviertel);
        },
        50...54 => {
            set_char_leds(&mut char_leds, &zehn);
            set_char_leds(&mut char_leds, &vor);
        },
        55...59 => {
            set_char_leds(&mut char_leds, &fuenf);
            set_char_leds(&mut char_leds, &vor);
        },
        _ => println!("should never happen")
    }

    let hour = &hours[hour as usize];
    set_char_leds(&mut char_leds, hour);

    display_console(&char_leds, &min_leds);

    std::thread::sleep(Duration::from_millis(100));
}

}

fn set_char_leds (char_leds: &mut[bool; 110], word: &Word) {
    let start = word.off as usize;
    let end   = (word.off + word.len) as usize;
    for led in start..end  {
        char_leds[led] = true;
    }
}

fn display_console (char_leds: &[bool; 110], min_leds: &[bool; 4]) {

//*
//*The characters are organized on the Display in 11 columns and 10 rows.
//*
let display_characters = [
    'E','S','k','I','S','T','a','F','Ü','N','F',
    'Z','E','H','N','Z','W','A','N','Z','I','G',
    'D','R','E','I','V','I','E','R','T','E','L',
    'V','O','R','F','U','N','K','N','A','C','H',
    'H','A','L','B','a','E','L','F','Ü','N','F',
    'E','I','N','S','x','a','m','Z','W','E','I',
    'D','R','E','I','a','u','j','V','I','E','R',
    'S','E','C','H','S','n','l','A','C','H','T',
    'S','I','E','B','E','N','Z','W','Ö','L','F',
    'Z','E','H','N','E','U','N','k','U','H','R'
];

    println!("------------------------");
    println!("{}                      {}",
             if min_leds[0] {"1"} else {"0"},
             if min_leds[1] {"1"} else {"0"});
    for row in 0..10 {
        print!(" ");
        for column in 0..11 {
            let pos = (row*11) + column;
            if char_leds[pos] {
                print!("{} ", display_characters[pos]);
            }
            else {
                print!("  ");
            }
        } 
        println!("");
    }
    println!("{}                      {}",
             if min_leds[2] {"1"} else {"0"},
             if min_leds[3] {"1"} else {"0"});
    println!("------------------------");
}

fn get_time() -> (u8, u8) {
    let t = time::now();
    let mut hour = t.tm_hour as u8;
    if hour > 11 {
        hour -= 12;
    }
    return (hour, t.tm_min as u8);
}

fn decrement_hour(hour: u8) -> u8 {
    if hour == 0  { 11 } else { hour - 1 }  
}

