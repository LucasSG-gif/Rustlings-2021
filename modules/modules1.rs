// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

// I AM DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    pub fn make_sausage(){
        println!("sausage")
    }
}

fn main() {
    sausage_factory::make_sausage();
}
