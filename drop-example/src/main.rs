struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}` !", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    let e = CustomSmartPointer {
        data: String::from("some data"),
    };

    println!("CustomSmartPointer created!");

    // c.drop(); is not allowed
    drop(c);

    println!("CustomSmartPointer for `c` dropped before the end of main");
}
