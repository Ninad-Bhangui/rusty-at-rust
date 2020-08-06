struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping instance of CustomSmartPointer : {}", self.data);
    }
}

fn main() {
    let first_instance = CustomSmartPointer {
        data: String::from("first_instance"),
    };
    let second_instance = CustomSmartPointer {
        data: String::from("second_instance"),
    };

    let drop_instance = CustomSmartPointer {
        data: String::from("drop_instance"),
    };
    // drop_instance.drop();    //Not allowed.
    drop(drop_instance); //Allowed early drop
    println!("Created 2 instances");
}
