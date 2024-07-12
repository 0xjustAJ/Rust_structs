/*struct is the fundamental bilding block of rust programming language*/
struct Person {
    name: String,
    age: u32,
    //  is_student: bool,
}
//implementing a method for person
impl Person {
     //associated function to create a new person and age
     fn create_a_new_person(name:&str, age:u32) -> Person {
        Person { 
            name: String::from("name"), 
            age,
        }
     }
     
     //associated function to create a default person
     fn Create_default_person(name:&str, age: u32) -> Person {
        Person {
            name: String::from("Ajayi Damola"),
            age: 27,
        }
     }




    //display the infomation about yourself
    fn display_info(&self) {
        println!("Name:{},  Age:{}", self.name, self.age);
    }
}

/*Compex or multiple structs allows for hierarchical data structures*/
// define astruct named Author
struct Author {
    name: String,
    age: u32,
}
//define a struct named Book for our inventory management system
struct Book {
    title: String,
    genre: String,
    author: String, //Author, //this is our nested structs. the book struct contains the Author struct in the author field. this creates a relationship between the structs
    quantity_in_stock: u32,
}

impl Book {
    /*the methods to display book information*/
    fn display_book_info(&self) {
        //the &self means that we are making reference to the Book struct
        println!(
            "the book title is: {},
             the book author is {},
             the book genre is {},
             the total number of quality in stock is {},
             ",
            self.title, self.author, self.genre, self.quantity_in_stock
        );
    }

    //Method to update stock quantity
    /*when we are updating a variable or a data, we are about to change the state of the original data, which calls for mutating the variable*/
    fn update_stock(&mut self, new_quantity: u32) {
        self.quantity_in_stock = new_quantity;
    }
}

/*A Real world example, lets create an address book application*/
struct Contact {
    name: String,
    phone_number: String,
    email: String,
}

struct Car {
    make: String,
    model: String,
    year: u32,
}

/*tuple struct*/
struct Color(u8, u8, u8);

/*Default structs values*/
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}
/* implemeting Default trait for Point3D */
impl Default for Point3D {
    fn default() -> Self {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/* custom default values */
//in case u might want to provide custom default values for specific fields
#[derive(Debug)]
struct Configuration {
    /* add some fields with default values */
    threshold: f64,
    max_iterations: u32,
}
//implement a function for our custom values
impl Configuration {
    fn new() -> Self {
        Configuration {
            threshold: 0.5,
            max_iterations: 100,
        }
    }
}

/*enumeration */
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, length: f64 },
}

/*A practical user profile*/
#[derive(Debug)]
struct UserProfile {
    username: String,
    email: String,
    profile_picture: Option<String>, //Optional profile picture
}

//implementing a function for custom default values
impl UserProfile {
    //function to create a new user with default values
    // fn new(username: &str, email: &str) -> Self {
    //     UserProfile {
    //         username: String::from("Just_AJ"),
    //         email: String::from("ajayidamola4@gamail.com"),
    //      //   profile_picture: None, //no profile picture by default
    //     }
    // }

    //fn to update profile picture
    // fn update_profile_picture(&mut self, picture_url: &str) {
    //     self.profile_picture = Some(String::from(picture_url));
    // }
}

/*Methods and associated methods in Rust */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height      
    }

    //allowing for other rectangles
    // fn check_for_other_rectangles(&self, other: &Rectangle) -> bool {
    //     self.width >= other.width && self.height >= other.height
    // }

    /*Dimension scalling*/
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}


/*An associated function */
#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn create_circle_with_radius(radius: f64) -> Self { //either self or circle
       Circle { radius }
    }

    fn calculate_area(radius:f64, pi: f64) -> f64 {
         radius* pi 
    }
}


/*combining the power of methods and associate */
#[derive(Debug)]
struct File {
    name: String, //name of file
    size: u64,
}

impl File {
    //associated function to create a new file
    fn create_new_file(name:&str, size: u64) -> File {
        File {
            name: String::from("name"),
            size,
        }
    }

    // creating a method to get the size of the file
    fn get_file_size(&self) -> u64 {
       self.size
    }

    //creating a method to resize file 
    fn resize_file(&mut self, new_size: u64) {
        self.size = new_size
    }

    //associated function to check if the file is_empty
    fn is_empty(file: &File) -> bool {
       file.size == 0
    }

}

/*Practical example of combining the power of methods and associated function*/
/*When we want to create a song library in rust*/
struct Song {
    title: String, //song title
    artist: String, //song artist
    duration_seconds: u32,
}

impl Song{
    //Associated function to create a new song
    fn create_a_new_song(title: &str, artist:&str, duration_seconds:u32) -> Song {
        Song {
            title: String::from(title),
            artist: String::from(artist),
            duration_seconds,
        }
    }

       //method to get the title of the song
   fn get_song_title(&self) -> &str{
    &self.title
} 
// fn to get the song srtist
fn get_song_artist(&self) -> &str {
    &self.artist
}

//methods to get the duration of songs in minutes and seconds
fn get_song_duration(&self) -> String {
    let minutes= self.duration_seconds / 60;
    let seconds = self.duration_seconds % 60;
    
    format!("{:02}:{:02}", minutes, seconds)
}

}


/*life times in Rust Programming language */
#[derive(Debug)]
struct BookShelf <'a>{
   // books:&'a[&'a str],   //the bookshelf sruct holds an immutable reference to bytes of an array of book titles
    books: &'a mut Vec<&'a str>

} 



/*Enums and pattern matching with structs in rust*/
//lets define a traffic light enumeration
#[derive(Debug)]
enum TrafficLight {
    Red,       //enums variants representing a state
    Yellow,     //enums variants representing a state
    Green,     //enums variants representing a state
}


/*Enumeration with data*/
#[derive(Debug)]
enum Animal {
    Dog {name: String, age:u8},
    Cat {name: String, color: String},
    Bird {name: String, can_fly: bool},
}

//function to simulate the traffic light
fn simulate_traffic_light(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("STOP"),
        TrafficLight::Yellow => println!("PREPARE TO STOP"),
        TrafficLight::Green => println!("GO"),
    }
}

/*matching enums with data becomes more powerful*/
#[derive(Debug)]
enum Persons {
    Student {name: String, age: u32, grade: u32},
    Teacher {name: String, subject:String, age: u32},
    
}
//fn to print person based on roles
fn print_person_info(person:Persons) {
    match person {
        Persons::Student {name, age, grade} => {
        println!("student {} ({} years old) - Grade: {}", name, age, grade)
        }
        Persons:: Teacher { name, subject, age } => {
            println!("Teacher {}, of the age {}, teaches {}", name, age, subject);
        }
    }
}


/*matching the options in enums*/
//define a enum named option
#[derive(Debug)]
enum Options<T> {
    Some(T),
    None,   
}
//write a function to print out an optional number
fn print_out_optional_value(value:Option<i32>) {
    match value {
             Option::Some(value) => println!("we have some numbers which will be printed"),
             Option::None => println!("Found no number to print out")
    }
}


/*combining enums and structs*/
#[derive(Debug)]
enum Shapes {
    Circle {radius: f64},
    Rectangle {width: f64, height: f64},
}

fn calc_area(shape:Shapes) -> f64 {
    //using the match expression
    match shape {
        Shapes::Circle {radius} => 3.142 * radius * radius,
        Shapes::Rectangle {width, height} => width * height,
    }
}



/*traits in rust*/
/*traits encapsulate behaviours */

trait Drawable {
    fn draw(&self);

// Default method to get background_color 
fn background_color(&self) -> &'static str {
     "White"
}
}

trait Movable {
    fn move_item(&self);
}

//implement the drawable trait for circle struct
#[derive(Debug)]
struct Circles {
    radius: f64,
}

impl Drawable for Circles {
    fn draw(&self){
        println!("Drawing a circle with radius {}, and background color {}", self.radius, self.background_color());
    }
}


//implement the drawable trait for a square struct
struct Square {
    side_lenght: f64,
}

impl  Drawable for Square {
    fn draw(&self){
        println!("Drawing a square with side lenght {}, and background color {}", self.side_lenght, self.background_color());
    }
} 



/*Trait bounds */
/*trait bounds are used in function signature to ensure that the function accepts only the type implemented by certain traits */
/*this ensures that the functions can use methods defined in the specified traits */

//generic function with trait bounds
fn perform_draw_and_move <T>(item: T)
where 
     T: Drawable + Movable 
     {
        item.draw();
        item.move_item();
     }




/* Associated types with traits */
//define trait named Container with an associated type

trait Container {
    //Associated type representing the content of the container 
    type Item;

    //define a method to insert an item into a container
    fn insert(&mut self, item:Self::Item);

    //method to retrieve an item from the container
    fn retrieve(&self) -> &Self::Item;

}

//implement the Container trait for a struct ganeric 'BoxContainer'
struct BoxContainer <T> {
    content: Option<T>
}

//specify 'i32' as the Associated type for 'BoxContainer'

impl Container for BoxContainer<i32> {
    type Item = i32;

    fn insert(&mut self, item: Self::Item){
        self.content = Some(item);
    }

    fn retrieve(&self) -> &Self::Item {
        self.content.as_ref().expect("Container is empty")
    }



}


/* Blanket implementation */
//define a trait named printable
trait Printable {
    //method to print the item
    fn print(&self); 
}
//blanket implementation for all types that use debug
impl<T:std::fmt::Debug> Printable for T {
       fn print(&self) {
        println!("{:?}", self)
       }
}

/*Orphan Rules */
//define a trait named showable

trait Showable {
    //Method to show the item
    fn show(&self);
}

//implement showable for the standard library String type
impl Showable for String {
    fn show(&self){
        println!("Showing the string: {}", self)
    }
}

/*The basics of Result */
// a function that will return 
fn divide(a:f64, b:f64) -> Result<f64, String> {
    if b == 0.0 {
        //return an error if the divisor is 0
        Err(String::from("Cannot divide by zero"))
    }else {
        //Return the result if the division is successful
        Ok(a/b)
    }
}

/*chaining Results with 'map' and `and_then`*/
//a function that performs two divisions and then returns a `Result`
fn chained_division(a:f64, b:f64, c:f64) -> Result<f64, String> {
    //chain two divisions using map method
    let result = divide(a, b).map(|x| x*c);

    //perform additional operations using `and_then`
    result.and_then(|x| divide(x, 2.0))
}


/*matching on specific errors*/
// A function that performs a division, and handle specific error cases
fn handle_specific_errors(a:f64, b:f64) -> Result<f64, String> {
    match divide(a,b){
        Ok(result) => Ok(result),
        Err(error) => match error.as_str() {
            "Cannot divide by Zero" => Err(String::from("Division by Zero is not allowed")),
        _ => Err(error),
        }
    }
}


//a function that uses ? for operator for concise error propagation. this is useful in fuctions that returns result
fn propagate_error(a: f64, b:f64) -> Result<f64, String> {
    let result = divide(a,b)?;
    Ok(result * 2.0)
}



/*Non - Recoverable Errors*/
fn find_element(collection:Vec<i32>, target:i32) -> Option<i32> {
    for &element in collection.iter() {
        if element == target {
            return Some(element);
        }
    }
    None
}


/*combining options with map and_then */
//A function that combines options with `map` and `and_then`
fn combined_options(a:Option<i32>, b:Option<i32>) -> Option<i32> {
      //use map to add the values if both are present
      //let result = a.map(|x| x + b?);
      let result = a.and_then(|x| b.map(|y| x + y));
      //use `map` for additional operations
      result.map(|x| x*2)
} 


/* A function that unwraps `Options` using `unwrap` and `expect`*/
fn unwrap_options(a:Option<i32>) -> i32 {
    //use unwrap when certain that `a` contains a value
    let unwrapped_value = a.unwrap();
    //use `expect` with a custom error message
    let expected_value = a.expect("Option was None, but an expected value was specified");

    //return the unwrapped values
    unwrapped_value + expected_value
}


/* the `ok_or` or the `ok_or_else` methods*/
/* this methods allow converting an Option into a Result, providing more information in case of an absent value*/
// A function that converts an Option into a Result with 'ok_or' and 'ok_or_else'
fn option_to_result(a:Option<i32>) -> Result<i32, &'static str> {
    //use `ok_or` to convert `Some` to `Ok` and `None` to an error
    let result = a.ok_or("Value was None");

    //use `ok_or_else` for a more complex error message
    result.or_else(|a|Err("An unexpected error occurred."))
}































/*after creating a struct, next we create an instance of the struct (in our case Person), also known as struct object*/
fn main() {
    let Ajayi_Damola = Person {
        name: String::from("Ajayi Damola"), // the name field
        age: 25,                            // the age field in rust
                                            //   is_student: true,                    //the student field
    };

    /*to access the name field in the struct, we do the following*/
    let my_name = &Ajayi_Damola.name;
    // println!("my name is {}", my_name);

    /*In rust, when a struct is created field, is created, it is immutable by default.
    we can make changes to the struct fields by making it mutable, and making mutable reference to it*/
    let mut Ajayi_Damola = Ajayi_Damola;

    //we update the age field
    Ajayi_Damola.age = 26;

    //println!("the updated age is {}", Ajayi_Damola.age)

    /*lets create an instance of Contact*/
    let ajayi = Contact {
        //we need a name for the contact
        name: String::from("ajayi damola"),
        phone_number: String::from("+234 8144102532"),
        email: "ajayidamola4@gmail.com".to_string(),
    };

    let damola = Contact {
        name: "ajayi kingsley".to_string(),
        phone_number: "+234 8144102532".to_string(),
        email: "damola07@gmail".to_string(),
    };

    /* our address book now has two contacts address*/
    //println!("{}: {}: {}:", ajayi.name, ajayi.phone_number, ajayi.email);
    //println!("{}: {}: {}:", damola.name, damola.phone_number, damola.email);

    /*since we have created our car struct, then we create an instance of the struct named my_car which is based on the blue print we have defined*/
    let my_car = Car {
        make: String::from("Toyota"),
        model: String::from("Camry"),
        year: 2024,
    };

    //we create an instance of the struct color named my_color
    let my_color = Color(255, 0, 0);

    /*struct can also have an associated functions or methods defined on it*/
    let my_person = Person {
        name: String::from("titilayo"),
        age: 26,
    };

    //we can call on our method function to print out the name now
    //my_person.display_info();

    /* we create a rust_book instance of the Book struct*/
    let rust_book = Book {
        title: "struct and enums book".to_string(),
        genre: "hands-on".to_string(),
        author: "jp".to_string(),
        quantity_in_stock: 100,
    };

    //next we display infomation about the book by calling on the method
    //rust_book.display_book_info();

    //create a mutable reference to update our struct book
    let mut rust_book_reference = rust_book;
    //rust_book_reference.update_stock(80);
    //then we display the updated reference
    //rust_book_reference.display_book_info();

    /* using rust short hand */
    let x = 5.0;
    let y = 3.2;
    let z = -1.2;
    let point_shorthand = Point3D { x, y, z };
    let updated_point = Point3D {
        x: 5.0,
        ..point_shorthand
    };

    /* uding default strust values */
    /* creating an instance of struct default values */
    let point = Point3D {
        x: 2.5,
        y: 1.0,
        z: -3.2,
    };

    /* creatin an instance with default values */
    let default_point = Point3D::default();

    /* creating an instance with custom configuration default value */
    let default_configuration = Configuration::new();

    //println!("{:?}", default_configuration);

    let circle = Shape::Circle { radius: 3.0 };
    let rectangle = Shape::Rectangle {
        width: 4.0,
        length: 6.0,
    };

    //next we create an instance of the user profile
    // let default_user = UserProfile::new("just_aj", "ajayidamola4@gmail.com");

    // let user_with_picture = UserProfile {
    //     username: "AJ".to_string(),
    //     email: "damola07@gmail.com".to_string(),
    //     profile_picture: /*some*/String::from("this_is_the_picture_url.jpeg"),
    // };

    //creting an instance of the rectangle
    // let rectangle = Rectangle {
    //     width: 100,
    //     height: 200,
    // }; //this is a statement as it brings out no output

  // let area = rectangle.calculate_area();
    // println!("the area of the rectangle is {}", area)

    /*placing the check constraint*/
    let larger_rectangle = Rectangle {
        width: 10,
        height: 20,
    };

    let smaller_rectangle = Rectangle {
        width: 5,
        height: 10,
    };

    //let can_the_rectangle_fit = larger_rectangle.check_for_other_rectangles(&smaller_rectangle);
    // println!("{}", can_the_rectangle_fit);

    // if can_the_rectangle_fit {
    //     println!("the smaller rectangle can fit into the larger rectangle");
    // } else {
    //     println!("the smallest rectangle can not fit into the larger rectangle");
    // }

/*chain methods to calculate the area and then scale the rectangle*/
let mut rectangle = Rectangle {
    width: 20,
    height: 400,
};
 
let rectangle_1 = rectangle.calculate_area();
//  println!("{:#?}", rectangle.scale(1));
//  println!("{}", rectangle.calculate_area());


/*calling the associated function to create a circle */
let smaller_circle = Circle::create_circle_with_radius(14.0);
let bigger_circle = Circle::create_circle_with_radius(100.0);
//we can also call addition methods on the instances as needed

let small_circle_area = Circle::calculate_area(14.0 * 14.0, 3.142);
//  println!("{:?}", smaller_circle);
//  println!("{}", small_circle_area);


/* calling the associated function to create a person*/
let custom_person = Person::create_a_new_person("Ramon", 30); //these are instance creation e.g custom_person
let default_person = Person::Create_default_person("name", 0);//instance creation e.g default_person

// println!("custom_person {} - Age{}", custom_person.name, custom_person.age );
// println!("default_person {} - Age{}", default_person.name, default_person.age);


/*create a file using the associated function*/
let mut data_file = File::create_new_file("data.txt", 1024);
//calling nethods to perform operations from the file
let initial_file = data_file.get_file_size(); 

data_file.resize_file(2048);
let new_size = data_file.get_file_size();

//println!("{}", new_size);
//calling the associated fuction to check if the file is empty

let is_empty = File::is_empty(&data_file);

// println!("the initial size was {}", initial_file);
// println!("the new_size is {}", new_size);

// if is_empty {
//     println!("the file is empty");
// } else {
//     println!("the file is not empty");
// }


/*create a song using the associated fuction */
let favorite_song = Song::create_a_new_song("Things gonna be alright", "AJ", 300);

//call methods to retrieve the information about the song
let title = favorite_song.get_song_title();
let artist = favorite_song.get_song_artist();
let duration_formatted = favorite_song.get_song_duration();

//println!("The song title is {}, by {}, song-lenght {}", title, artist, duration_formatted);


/*create an array of string laterals representing the book titles*/
let book_titles = ["The rust programming language", "Rust Brain Teasers"];
/*create an instance of BookShelf with reference to book titles*/
//let my_book_shelf = BookShelf { books: &book_titles};
//println!("{:?}", my_book_shelf);
//note that my book shelf is now the owner of the reference to the book titles, and its life time is tied to the scope of book titles


/*create a mutable vector of string literals representing book titles */
let mut book_titles = vec!["The Rust Programming language", "Rust Brain Teasers"];

/*create an instance of BookShelf with muhttps://www.notion.so/invite/90387efd415150f195ba194bca45f086c4de9468table reference to the book titles*/
let mut my_bookshelf = BookShelf {books: &mut book_titles};
my_bookshelf.books.push("Rust for Beginners");

//println!("{:?}", my_bookshelf);



/*ENUMERATION*/
//creating an instance of traffic light
let red_light = TrafficLight::Red;
let yellow_light = TrafficLight::Yellow;
let green_light = TrafficLight::Green;

//println!("the traffic lights are {:?}, {:?}, and {:?}", red_light, yellow_light, green_light);

/*creating an instance of Aminal */
let my_dog = Animal::Dog {name: String::from("Labrador"), age: 12};
let my_cat = Animal::Cat {name: String::from("Bramboo"), color: "white and black".to_string()};
let my_bird = Animal::Bird {name: String::from("Eagle"), can_fly: true};

//println!("you can identify my Animals in the following ways, {:?}, {:?}, {:?}", my_dog, my_cat, my_bird);

/*create instances of TrafficLight and  simulate behaviour*/
let red_light = TrafficLight::Red;
let yellow_light = TrafficLight::Yellow;
let green_light = TrafficLight::Green;

// simulate_traffic_light(red_light);
// simulate_traffic_light(yellow_light);
// simulate_traffic_light(green_light);



 /* creating an instance of the person enu*/
let student = Persons::Student {
             name: String::from("Ajayi Damola"),
             age: 27,
             grade: 70,
};

let teacher = Persons::Teacher { 
    name: String::from("Prof Aguegbo"),
    subject:String::from("Stochastic and time series analysis") , 
    age: 63,
};

// print_person_info(student);
// print_person_info(teacher);


/*use option to represent a number, and then process it*/
let some_number = Option::Some(42);
let no_number: Option<i32> = Option::None;

// print_out_optional_value(some_number);
// print_out_optional_value(no_number);


/*create an instance of shape and calculate their areas*/
let circle_shape_is = Shapes::Circle { radius: 40.0 };
let rectangular_shape_is = Shapes::Rectangle { width: 10.0, height: 20.0 };

let area_circle = calc_area(circle_shape_is);
let area_rectangle = calc_area(rectangular_shape_is);

// println!("The area of a circle is: {}", area_circle);
// println!("The area of a rectangle is: {}", area_rectangle);


/*creatiing an instance of circle and square */
let circle = Circles { radius: 10.2};
let square = Square { side_lenght: 7.0};

//call the draw method on both instance
// circle.draw();
// square.draw();


/*Create an instance of of BoxContainer with i32 as the associated type */
let mut my_box = BoxContainer { content: None};
     my_box.insert(42);

     //retrieve and print the item from the container 
     //println!("Item in the container: {}", my_box.retrieve());

let number = 42;
let text = "Hello, Rust!";
let vector = vec![1,2,3];

// number.print();
// text.print();
// vector.print();

//use the showable trait witha string
let my_string = String::from("Rust is Amazing");
//my_string.show();


//use the divide function and handle the result 
match divide(10.0, 2.0) {
  Ok(result) => println!("Result of divsion: {}", result),
  Err(error) => println!("Error: {}", error),
}

// attempt a division by 0
match divide(8.0, 0.0){
    Ok(result) => println!("Result of division {}", result),
    Err(error) => println!("Error: {}", error),
}

/*chaining results with map and_then */
//use the chained_division function and handle the result
match chained_division(10.0, 2.0, 3.0) {
    Ok(final_result) => println!("Final Result: {}", final_result),
    Err(error) => println!("Error: {}", error),
}


/*use the handle_specific_error function and handle the result */
match handle_specific_errors(8.0, 0.0) {
    Ok(result) => println!("Result of division: {}", result),
    Err(error) => println!("Error: {}", error),
}

/*the ? operator for concise error propagation*/
   match propagate_error(12.0, 3.0){
    Ok(final_result) => println!("Final result: {}", final_result),
    Err(error) => println!("Error: {}", error)
   }


//use the find element function and handle the result
   match find_element(vec![1, 2, 3, 4, 5], 3) {
    Some(result) => println!("Element found: {}", result),
    None => println!("Element not found"),

   }

//attempt to find non exixting element
match find_element(vec![1,2,3,4,5], 6) {
    Some(result) => println!("Element found: {}", result),
    None => println!("Element not found"),
}


//use the combined option function to handle the result
match combined_options(Some(5), Some(3)) {
    Some(final_result) => println!("Final result: {}", final_result),
    None => println!("At least one value is absent."),
}


/*Unwrapping Options with unwrap and expect*/
//use the unwrap function and handle the result
let result = unwrap_options(Some(7));
//println!("Result after unwrapping: {}", result);

//use the option_to_result function and handle the result
match option_to_result(Some(10)){
    Ok(final_result) => println!("Final result: {}", final_result),
    Err(error) => println!("Error: {}", error),
}

}
