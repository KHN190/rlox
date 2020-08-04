macro_rules! bin_op {
    ($self:ident, $op:tt) => {
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        $self.stack.push(a $op b);
        $self.ip += 1;
    }
}

macro_rules! unary_op {
	($self:ident, $op:tt) => {
		let val = $op $self.stack.pop().unwrap();
		$self.stack.push(val);
		$self.ip += 1;
	}
}
