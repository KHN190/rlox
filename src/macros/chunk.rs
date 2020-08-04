macro_rules! grow_capacity {
    ( $x:expr ) => {
        if $x < 8 {
            8
        } else {
            $x * 2
        }
    };
}
