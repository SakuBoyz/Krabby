trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

// Example implementation of the traits
struct ExampleStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Person for ExampleStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Student for ExampleStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Programmer for ExampleStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl CompSciStudent for ExampleStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comp_sci_student_greeting() {
        let student = ExampleStudent {
            name: "John Doe".to_string(),
            university: "MIT".to_string(),
            fav_language: "Rust".to_string(),
            git_username: "john_doe".to_string(),
        };

        let expected_greeting = "My name is John Doe and I attend MIT. My favorite language is Rust. My Git username is john_doe";
        let actual_greeting = comp_sci_student_greeting(&student);

        assert_eq!(expected_greeting, actual_greeting);
    }
}
