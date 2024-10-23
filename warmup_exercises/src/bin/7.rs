
struct Student { 
    /*some defining data elements of a student*/
 }
struct Teacher { /*some defining data elements of a teacher*/ }

impl Hello for Student {
    fn say_hi(&self) -> String {
        "hi".to_string()
    }
    fn say_something(&self) -> String {
        "I'm a good student".to_string()
    }
}

impl Hello for Teacher {
    fn say_hi(&self) -> String {
        "Hi, I'm your new teacher".to_string()
    }    
    fn say_something(&self) -> String {
        "I'm not a bad teacher".to_string()
    }
}
    

//TODO complete the trait definition and create implementations for the `Student` and `Teacher` type to make the code work
trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;

}


//don't modify this function
fn main() {
    let s = Student{};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");

    let t = Teacher{};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");

    println!("Success!");
}
