fn main(){

    struct Student {
        name: String,
        level: u8,
        remote: bool
    }

    struct Person (String, String, i32);

    let first_student: Student = Student {
        name: String::from("Matías"),
        level: 5,
        remote: false
    };

    let person: Person = Person (
        String::from("Ignacio"),
        String::from("A2"),
        14,
    );

    println!("My name is: {}. In the level {} and I study remote?  {}", first_student.name, first_student.level, first_student.remote);
    println!("My name is: {}. I study in the {} and I am {}", person.0, person.1, person.2);

}