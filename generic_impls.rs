struct Container<T> {
    value: T
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
}


// more specific impl block for concrete type u32
impl Container<u32> {
    fn sum(value: u32) -> Self {
        Container { value }
    }
}

fn main() {
    // hello
}
