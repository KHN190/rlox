#[macro_use]
pub mod vm;

#[macro_use]
pub mod scan;

// generic macros

macro_rules! grow_capacity {
    ( $x:expr ) => {
        if $x < 8 {
            8
        } else {
            $x * 2
        }
    };
}

macro_rules! line_info {
    ( $self:ident, $bytes:ident ) => {
        if $self.ip > 0 && $bytes.get_line($self.ip) == $bytes.get_line($self.ip - 1) {
            String::from("    | ")
        } else {
            format!(" {:4} ", $bytes.get_line($self.ip))
        }
    };
}
