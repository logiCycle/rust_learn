fn main() {
    let c = CustomSmartPointer{data: String::from("my world")};
    let d = CustomSmartPointer{data: String::from("other world")};
    println!("CustomSmartPointer created.");
}

struct CustomSmartPointer {data: String}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with Data {}!", self.data);
    }
}