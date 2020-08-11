macro_rules! token {
    ( $self:ident, $val:literal, $tt:expr ) => {
        Token {
            tt: $tt,
            value: Cow::from($val),
            line: $self.line,
        }
    };

    ( $self:ident, $val:expr, $tt:expr ) => {
        Token {
            tt: $tt,
            value: Cow::from($val),
            line: $self.line,
        }
    };
}

macro_rules! if_then_token {
    ( $self:ident, $if:literal, $then_val:literal, $then:expr, $else_val:literal, $else:expr ) => {
        if $self.expect($if) {
            token!($self, $then_val, $then)
        } else {
            token!($self, $else_val, $else)
        }
    };
}
