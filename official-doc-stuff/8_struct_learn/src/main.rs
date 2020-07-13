fn main() {
    //Basic struct which is also mutable. Whole struct must be mutable not just individual properties
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    //This works because above object is mutable
    user1.email = String::from("changedsomeone@example.com");

    println!(
        "{}, {}, {}, {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );

    //Using builder
    let user2 = build_user(
        String::from("anotherone@example.com"),
        String::from("anotherone"),
    );
    println!("{},{}", user2.email, user2.username);

    //Creating instance from another using struct update syntax
    let user3 = User {
        email: String::from("thirdguy@example.com"),
        username: String::from("thirdguy"),
        ..user1
    };
    println!("{},{}", user3.email, user3.username);

    //Tuple structs - Like structs but without named properties
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    print_color(black);
    // print_color(origin); //This would not work as it expects Color struct instance. In case of normal tuples this would work.

    //Unit structs () exist but did not understand. Explained more in chapter 10. Useful to implement traits without data.
}

//Struct (Sounds like class in python so far)
struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}
//Builder function
fn build_user(email: String, username: String) -> User {
    //Can return instance of struct as expression
    User {
        email: email,
        username, //Can use this instead of above as shorthand if variable name is same as property name
        active: true,
        sign_in_count: 1,
    }
}
//Below would not work as an instance of struct must own it's properties. The property must not go out of scope before the struct.println!
// To use string slices in structs, lifetimes have to be used which are not explained yet.

// struct UserWrong {
//     email: &str,
//     username: String,
//     sign_in_count: u64,
//     active: bool,
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn print_color(color: Color) {
    println!("red = {}, blue = {}, green = {}", color.0, color.1, color.2);
}
