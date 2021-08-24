mod aux {
    pub mod foo;
}

fn main() {
    println!("{}", aux::foo::f());
}
