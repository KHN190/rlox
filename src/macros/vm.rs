macro_rules! bin_op {
    ($self:ident, $op:tt) => {
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        $self.stack.push(a $op b);
        $self.ip += 1;
    }
}