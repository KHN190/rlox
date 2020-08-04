macro_rules! line_info {
	( $self:ident, $bytes:expr ) => {
		if $self.ip > 0 && 
		   $bytes.get_line($self.ip) == $bytes.get_line($self.ip - 1) {
			String::from("    | ")
		} else {
			format!(" {:4} ", $bytes.get_line($self.ip))
		}
	}
}