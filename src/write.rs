pub trait Write {
    fn write_return(&mut self, f: impl FnOnce(&mut Self));

    fn write_function(
        &mut self,
        ident: &str,
        args: impl FnOnce(&mut Self),
        block: impl FnOnce(&mut Self),
    );
}

impl Write for String {
    fn write_return(&mut self, f: impl FnOnce(&mut Self)) {
        self.push_str("return ");
        f(self);
        self.push(';');
    }

    fn write_function(
        &mut self,
        ident: &str,
        write_args: impl FnOnce(&mut Self),
        write_block: impl FnOnce(&mut Self),
    ) {
        self.push_str("function ");
        self.push_str(ident);
        self.push('(');
        write_args(self);
        self.push_str("){");
        write_block(self);
        self.push('}');
    }
}
