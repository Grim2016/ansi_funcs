macro_rules! create_ansi {
    //Colours with optional comments
    (colour1 $func:ident;$code:expr;$($comment:expr)?) => {
        $(#[doc=$comment])?
        pub fn $func () -> String {
            format!("\u{001b}[{}m", $code)
        }
    };
    
    //Cursor with comment
    (cursor $func:ident;$type:expr;$($comment:expr)?) => {
        $(#[doc = $comment])?
        pub fn $func (amount:u16) -> String {
            format!("\u{001b}[{}{}",amount,$type)
        }
    };
    //From string
    (from_str $func:ident;$command:expr;$($comment:expr)?) => {
        $(#[doc = $comment])?
        pub fn $func () -> String {
            format!("\u{001b}[{}",$command)
        }
    }
}
/// Implemented according to https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797

pub mod colours {
    create_ansi!{colour1 bold;          1;}
    create_ansi!{colour1 dim;           2; "Same as faint"}
    create_ansi!{colour1 faint;         2; "Same as dim"}
    create_ansi!{colour1 rm_bd;        22; "Removes Bold or Dim/Faint effect"}

    create_ansi!{colour1 italic;        3;}
    create_ansi!{colour1 rm_italic;    23;}
    create_ansi!{colour1 underline;     4;}
    create_ansi!{colour1 rm_underline; 24;}
    create_ansi!{colour1 blinking;      5;}
    create_ansi!{colour1 rm_blinking;  25;}
    create_ansi!{colour1 inverse;       7;}
    create_ansi!{colour1 rm_inverse;   27;}
    create_ansi!{colour1 hidden;        8;}
    create_ansi!{colour1 visible;      28; "Removes Hidden effect"}
    create_ansi!{colour1 strikken;      9;}
    create_ansi!{colour1 rm_strikken;  29; "Removes Strikken effect"}
        
    create_ansi!{colour1 black_f;      30;}
    create_ansi!{colour1 red_f;        31;}
    create_ansi!{colour1 green_f;      32;}
    create_ansi!{colour1 yellow_f;     33;}
    create_ansi!{colour1 blue_f;       34;}
    create_ansi!{colour1 purple_f;     35;}
    create_ansi!{colour1 cyan_f;       36;}
    create_ansi!{colour1 white_f;      37;}

    create_ansi!{colour1 black_b;      40;}
    create_ansi!{colour1 red_b;        41;}
    create_ansi!{colour1 green_b;      42;}

    create_ansi!{colour1 yellow_b;     43;}
    create_ansi!{colour1 blue_b;       44;}
    create_ansi!{colour1 purple_b;     45;}
    create_ansi!{colour1 cyan_b;       46;}
    create_ansi!{colour1 white_b;      47;}

    create_ansi!{colour1 reset_colour;  0;}
}
pub mod cursor {
    /// Moves cursor to column x and row y        
    fn move_to_pos (column: u16, row: u16) -> String {
            format!("\u{001b}[{row}{column}")
    }
    create_ansi!{from_str move_home;       "H";}
    create_ansi!{cursor   move_up;         "A";}
    create_ansi!{cursor   move_down;       "B";}
    create_ansi!{cursor   move_left;       "C";}
    create_ansi!{cursor   move_right;      "D";}
    create_ansi!{cursor   move_up_b;       "E";  "Moves up n line(s) and to the beginning"}
    create_ansi!{cursor   move_down_b;     "F";  "Moves down n line(s) and to the beginning"}
    create_ansi!{cursor   move_to_column;  "G";}
    create_ansi!{from_str req_cursor_pos;  "6n";}

}
    
pub mod erase {
    create_ansi!{from_str erase_f_beg_to_crsr; "0J"; "Erases from the beginning of the display to the cursor"}
    create_ansi!{from_str erase_f_crsr_to_end; "1J"; "Erases from the cursor to the end of the display"}
    create_ansi!{from_str erase_display;       "2J";}
    create_ansi!{from_str erase_saved_lines;   "3J";}

    create_ansi!{from_str erase_f_crsr_to_eol; "0K"; "Erases from the cursor to the end of the line"}
    create_ansi!{from_str erase_f_bol_to_crsr; "1K"; "Erases from the beginning of the line to the cursor"}
    create_ansi!{from_str erase_line;          "2K";}
}