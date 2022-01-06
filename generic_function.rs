fn test_generic<T>(value: T) {
    let _ = value;
}

fn main(){
    let a = "hello";
    let b = 31;
    test_generic(a);
    test_generic(b);
}
