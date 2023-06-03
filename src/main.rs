mod repl;
mod sql;

#[cfg(not(tarpaulin_include))]
fn main() {
    repl::main_loop();
}
