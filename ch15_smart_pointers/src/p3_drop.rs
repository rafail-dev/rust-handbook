#![allow(dead_code, unused)]

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping `{}`", self.data);
    }
}

fn main() {
    {
        let a = CustomSmartPointer {
            data: String::from("A"),
        };
        let b = CustomSmartPointer {
            data: String::from("B"),
        };
    }

    {
        let c = CustomSmartPointer {
            data: String::from("C"),
        };
        drop(c);
        println!("Dropped before the end of the scope");
    }
    // Dropping `B`
    // Dropping `A`
    // Dropping `C`
    // Dropped before the end of the scope
}
