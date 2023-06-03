pub struct Context {
    pub repl_running: bool,
}

impl Context {
    pub fn new() -> Context {
        Context { repl_running: true }
    }
}
